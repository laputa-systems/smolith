use futures::channel::oneshot;
use futures::StreamExt;
use h2_support::prelude::*;
use laputa_protocol_test_support::block_on_with_limit;
use std::task::Poll;
use std::time::Duration;

const WRITE_CAPACITY: usize = 1024;
const REQUEST_DATA: usize = 64 * 1024;
const RESPONSE_DATA: usize = 8 * 1024;

fn queue_request_body(send: &mut SendStream<Bytes>) {
    let chunk = Bytes::from(vec![b'q'; 16 * 1024]);
    let mut remaining = REQUEST_DATA;
    while remaining > chunk.len() {
        send.send_data(chunk.clone(), false).unwrap();
        remaining -= chunk.len();
    }
    send.send_data(chunk.slice(..remaining), true).unwrap();
}

async fn run_case(user_ping: bool) {
    let (io, mut peer) = mock::new_with_write_capacity(WRITE_CAPACITY);
    let (handshake, ()) = futures::future::join(client::handshake(io), async {
        peer.read_preface().await.unwrap();
        peer.send_bytes(frames::SETTINGS).await;
        peer.send_bytes(frames::SETTINGS_ACK).await;
    })
    .await;
    let (mut sender, mut conn) = handshake.unwrap();
    peer.set_write_capacity(WRITE_CAPACITY);

    let mut user_pings = client::Connection::ping_pong(&mut conn);
    let request = Request::builder()
        .method(Method::POST)
        .uri("https://example.com/")
        .body(())
        .unwrap();
    let (response, mut send) = sender.send_request(request, false).unwrap();
    queue_request_body(&mut send);
    drop(send);
    drop(sender);

    let mut driver = Box::pin(async move {
        let result = conn.await;
        result.unwrap()
    });
    futures::future::poll_fn(|cx| {
        if peer.write_capacity() == 0 {
            return Poll::Ready(());
        }
        assert!(driver.as_mut().poll(cx).is_pending());
        if peer.write_capacity() == 0 {
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    })
    .await;

    if user_ping {
        user_pings
            .as_mut()
            .expect("user pings")
            .send_ping(Ping::opaque())
            .unwrap();
    }

    peer.send_frame(frames::headers(1).response(200)).await;
    peer.send_frame(frames::ping([7; 8])).await;
    peer.send_frame(frames::ping([8; 8])).await;
    peer.send_frame(frames::settings().initial_window_size(0)).await;
    peer.send_frame(frames::settings().initial_window_size(65_535)).await;
    peer.send_frame(frames::data(1, Bytes::from(vec![b'r'; RESPONSE_DATA])).eos())
        .await;

    let (data_seen_tx, data_seen_rx) = oneshot::channel();
    let body = async move {
        let response = response.await.unwrap();
        let mut body = response.into_body();
        let chunk = body.data().await.expect("a DATA frame").unwrap();
        assert_eq!(chunk.len(), RESPONSE_DATA);
        data_seen_tx.send(()).unwrap();
        assert!(body.data().await.is_none());
    };
    let peer_task = async move {
        data_seen_rx.await.expect("body data delivered");
        peer.unbounded_bytes().await;
        let mut acknowledgments = Vec::new();
        let mut saw_ping = false;
        while let Some(frame) = peer.next().await {
            match frame.unwrap() {
                h2::frame::Frame::Ping(ping) if ping.is_ack() => {
                    acknowledgments.push(Some(*ping.payload()));
                }
                h2::frame::Frame::Ping(_) => saw_ping = true,
                h2::frame::Frame::Settings(settings) if settings.is_ack() => {
                    acknowledgments.push(None);
                }
                _ => {}
            }
            if acknowledgments.len() == 5 && (saw_ping || !user_ping) {
                break;
            }
        }
        assert_eq!(acknowledgments, vec![None, Some([7; 8]), Some([8; 8]), None, None]);
        if user_ping {
            assert!(saw_ping, "peer never received the user PING");
        }
    };

    join3(driver, body, peer_task).await;
}

#[test]
fn pending_control_writes_do_not_stop_reading() {
    block_on_with_limit(
        async move {
            futures::future::join(run_case(false), run_case(true)).await;
        },
        Duration::from_secs(2),
    )
    .expect("both connections complete while a PONG or PING write is pending");
}

#[test]
fn control_response_overflow_closes_without_panicking() {
    block_on_with_limit(async {
        for settings in [false, true] {
            let (io, mut peer) = mock::new_with_write_capacity(WRITE_CAPACITY);
            let (handshake, ()) = futures::future::join(client::handshake(io), async {
                peer.read_preface().await.unwrap();
                peer.send_bytes(frames::SETTINGS).await;
                peer.send_bytes(frames::SETTINGS_ACK).await;
            }).await;
            let (mut sender, conn) = handshake.unwrap();
            let (response, mut send) = sender.send_request(
                Request::builder().method(Method::POST).uri("https://example.com/").body(()).unwrap(), false,
            ).unwrap();
            queue_request_body(&mut send);
            let mut driver = Box::pin(conn);
            futures::future::poll_fn(|cx| {
                assert!(driver.as_mut().poll(cx).is_pending());
                if peer.write_capacity() == 0 { Poll::Ready(()) } else { Poll::Pending }
            }).await;
            for payload in 0..64 {
                if settings {
                    peer.send_frame(frames::settings().initial_window_size(payload)).await;
                } else {
                    peer.send_frame(frames::ping([payload as u8; 8])).await;
                }
            }
            futures::future::poll_fn(|cx| {
                assert!(driver.as_mut().poll(cx).is_pending());
                Poll::Ready(())
            }).await;
            peer.unbounded_bytes().await;
            let (result, ()) = futures::future::join(driver, async {
                while let Some(frame) = peer.next().await {
                    if let h2::frame::Frame::GoAway(frame) = frame.unwrap() {
                        assert_eq!(frame.reason(), Reason::ENHANCE_YOUR_CALM);
                        return;
                    }
                }
                panic!("control flood did not produce GOAWAY");
            }).await;
            assert_eq!(result.unwrap_err().reason(), Some(Reason::ENHANCE_YOUR_CALM));
            drop((response, send, sender));
        }
    }, Duration::from_secs(2)).expect("bounded control backlog closes");
}
