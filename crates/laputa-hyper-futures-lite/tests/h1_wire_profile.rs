use std::collections::VecDeque;
use std::convert::Infallible;
use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};

use bytes::Bytes;
use http_body::{Body, Frame, SizeHint};
use hyper::client::conn::http1::{Builder, SendRequest};
use hyper::ext::{BodyProgress, Http1RequestBody, Http1RequestHead, ResponseFraming, TransportRead};
use hyper::rt::{Read, ReadBufCursor, Write};

enum Arrival {
    Data(&'static [u8]),
    Fail,
}

struct Transport {
    arrivals: VecDeque<(usize, Arrival)>,
    open: bool,
    written: Vec<u8>,
    reader: Option<Waker>,
    reads: usize,
}

#[derive(Clone)]
struct Io(Arc<Mutex<Transport>>);

impl Io {
    fn new(arrivals: Vec<(usize, Arrival)>, open: bool) -> Self {
        Io(Arc::new(Mutex::new(Transport { arrivals: arrivals.into(), open, written: Vec::new(), reader: None, reads: 0 })))
    }

    fn written(&self) -> Vec<u8> {
        self.0.lock().unwrap().written.clone()
    }

    fn reads(&self) -> usize {
        self.0.lock().unwrap().reads
    }
}

impl Read for Io {
    fn poll_read(self: Pin<&mut Self>, cx: &mut Context<'_>, mut buf: ReadBufCursor<'_>) -> Poll<io::Result<()>> {
        let mut transport = self.0.lock().unwrap();
        transport.reads += 1;
        let written = transport.written.len();
        match transport.arrivals.front() {
            Some((released_after, _)) if written < *released_after => {
                transport.reader = Some(cx.waker().clone());
                Poll::Pending
            }
            Some(_) => match transport.arrivals.pop_front() {
                Some((_, Arrival::Data(data))) => {
                    buf.put_slice(data);
                    Poll::Ready(Ok(()))
                }
                _ => Poll::Ready(Err(io::ErrorKind::ConnectionReset.into())),
            },
            None if transport.open => {
                transport.reader = Some(cx.waker().clone());
                Poll::Pending
            }
            None => Poll::Ready(Ok(())),
        }
    }
}

impl Write for Io {
    fn poll_write(self: Pin<&mut Self>, _cx: &mut Context<'_>, buf: &[u8]) -> Poll<io::Result<usize>> {
        let mut transport = self.0.lock().unwrap();
        transport.written.extend_from_slice(buf);
        if let Some(reader) = transport.reader.take() {
            reader.wake();
        }
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }
}

struct Frames(VecDeque<Bytes>);

impl Body for Frames {
    type Data = Bytes;
    type Error = Infallible;

    fn poll_frame(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Result<Frame<Bytes>, Infallible>>> {
        Poll::Ready(self.0.pop_front().map(|frame| Ok(Frame::data(frame))))
    }

    fn is_end_stream(&self) -> bool {
        self.0.is_empty()
    }

    fn size_hint(&self) -> SizeHint {
        SizeHint::with_exact(self.0.iter().map(|frame| frame.len() as u64).sum())
    }
}

#[derive(Debug, PartialEq)]
enum Seen {
    Data(Vec<u8>),
    Closed,
    Failed(io::ErrorKind),
}

#[derive(Debug)]
struct Rejected(&'static str);

impl fmt::Display for Rejected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

impl StdError for Rejected {}

#[derive(Clone, Copy)]
enum Ending {
    Close,
    KeepAlive,
    Fail,
}

struct LineFraming {
    seen: Arc<Mutex<Vec<Seen>>>,
    buffer: Vec<u8>,
    head_done: bool,
    ended: bool,
    ending: Ending,
}

impl LineFraming {
    fn new(seen: Arc<Mutex<Vec<Seen>>>, ending: Ending) -> Box<dyn ResponseFraming> {
        Box::new(LineFraming { seen, buffer: Vec::new(), head_done: false, ended: false, ending })
    }
}

impl ResponseFraming for LineFraming {
    fn receive(&mut self, read: TransportRead<'_>) {
        let seen = match read {
            TransportRead::Data(data) => {
                self.buffer.extend_from_slice(data);
                Seen::Data(data.to_vec())
            }
            TransportRead::Closed => {
                self.ended = true;
                Seen::Closed
            }
            TransportRead::Failed(error) => {
                self.ended = true;
                Seen::Failed(error.kind())
            }
        };
        self.seen.lock().unwrap().push(seen);
    }

    fn poll_head(&mut self) -> Result<Option<http::Response<()>>, Box<dyn StdError + Send + Sync>> {
        let Some(end) = self.buffer.windows(2).position(|window| window == b"\n\n") else {
            return if self.ended { Err(Box::new(Rejected("no head before the transport ended"))) } else { Ok(None) };
        };
        let head = self.buffer.drain(..end + 2).collect::<Vec<u8>>();
        self.head_done = true;
        let mut response = http::Response::new(());
        response.extensions_mut().insert(String::from_utf8(head).unwrap());
        Ok(Some(response))
    }

    fn poll_body(&mut self) -> Result<BodyProgress, Box<dyn StdError + Send + Sync>> {
        assert!(self.head_done, "the body follows its head");
        if let Some(line) = self.buffer.iter().position(|&byte| byte == b'\n') {
            let data = self.buffer.drain(..=line).collect::<Vec<u8>>();
            if data == b".\n" {
                return match self.ending {
                    Ending::Fail => Err(Box::new(Rejected("body rejected"))),
                    Ending::KeepAlive => Ok(BodyProgress::End { reusable: true }),
                    Ending::Close => Ok(BodyProgress::End { reusable: false }),
                };
            }
            return Ok(BodyProgress::Data(Bytes::from(data)));
        }
        if self.ended {
            return Err(Box::new(Rejected("body ended without its terminator")));
        }
        Ok(BodyProgress::NeedRead)
    }
}

fn request(head: &'static [u8], framing: Http1RequestBody, frames: &[&'static [u8]]) -> http::Request<Frames> {
    let mut request = http::Request::new(Frames(frames.iter().map(|frame| Bytes::from_static(frame)).collect()));
    request.headers_mut().insert("x-ignored", http::HeaderValue::from_static("1"));
    request.extensions_mut().insert(Http1RequestHead::new(Bytes::from_static(head), framing));
    request
}

fn framed(mut request: http::Request<Frames>, seen: &Arc<Mutex<Vec<Seen>>>, ending: Ending) -> http::Request<Frames> {
    let seen = seen.clone();
    hyper::ext::frame_response(&mut request, move || LineFraming::new(seen.clone(), ending));
    request
}

fn connect(io: Io, executor: &smol::LocalExecutor<'_>) -> SendRequest<Frames> {
    let (sender, connection) = smol::block_on(Builder::new().handshake::<_, Frames>(io)).unwrap();
    executor.spawn(async move { let _ = connection.await; }).detach();
    sender
}

fn run<T>(executor: &smol::LocalExecutor<'_>, future: impl std::future::Future<Output = T>) -> T {
    smol::block_on(executor.run(future))
}

async fn collect(response: http::Response<hyper::body::Incoming>) -> Result<Vec<u8>, hyper::Error> {
    let mut body = response.into_body();
    let mut out = Vec::new();
    while let Some(frame) = std::future::poll_fn(|cx| Pin::new(&mut body).poll_frame(cx)).await {
        out.extend_from_slice(&frame?.into_data().unwrap());
    }
    Ok(out)
}

const HEAD: &[u8] = b"PUT /p HTTP/1.1\r\nHOST: example\r\nx-Mixed-Case: \x01 odd \xff\r\nContent-Length: 5\r\n\r\n";

#[test]
fn raw_head_and_length_body_are_written_byte_for_byte() {
    let executor = smol::LocalExecutor::new();
    let io = Io::new(Vec::new(), true);
    let mut sender = connect(io.clone(), &executor);
    let pending = sender.send_request(request(HEAD, Http1RequestBody::Length(5), &[b"hello"]));
    for _ in 0..100 {
        if !executor.try_tick() {
            break;
        }
    }
    drop(pending);
    assert_eq!(io.written(), [HEAD, b"hello"].concat());
}

#[test]
fn chunked_head_writes_each_frame_as_one_chunk() {
    let executor = smol::LocalExecutor::new();
    let head: &[u8] = b"POST / HTTP/1.1\r\nTransfer-Encoding: chunked\r\n\r\n";
    let io = Io::new(Vec::new(), true);
    let mut sender = connect(io.clone(), &executor);
    let pending = sender.send_request(request(head, Http1RequestBody::Chunked, &[b"abc", b"0123456789abcdef!"]));
    while executor.try_tick() {}
    drop(pending);
    assert_eq!(io.written(), [head, b"3\r\nabc\r\n11\r\n0123456789abcdef!\r\n0\r\n\r\n"].concat());
}

#[test]
fn chunked_head_without_a_body_writes_only_the_last_chunk() {
    let executor = smol::LocalExecutor::new();
    let head: &[u8] = b"POST / HTTP/1.1\r\nTransfer-Encoding: chunked\r\n\r\n";
    let io = Io::new(Vec::new(), true);
    let mut sender = connect(io.clone(), &executor);
    let pending = sender.send_request(request(head, Http1RequestBody::Chunked, &[]));
    while executor.try_tick() {}
    drop(pending);
    assert_eq!(io.written(), [head, b"0\r\n\r\n"].concat());
}

#[test]
fn body_that_contradicts_the_head_framing_is_rejected() {
    let executor = smol::LocalExecutor::new();
    let io = Io::new(Vec::new(), true);
    let mut sender = connect(io, &executor);
    let error = run(&executor, sender.send_request(request(b"GET / HTTP/1.1\r\n\r\n", Http1RequestBody::Empty, &[b"unexpected"]))).unwrap_err();
    assert!(error.is_user(), "{error:?}");
}

#[test]
fn framing_receives_each_read_and_frames_the_body() {
    let executor = smol::LocalExecutor::new();
    let head: &[u8] = b"GET / HTTP/1.1\r\n\r\n";
    let io = Io::new(vec![(head.len(), Arrival::Data(b"status")), (0, Arrival::Data(b"\n\nline one\nline")), (0, Arrival::Data(b" two\n.\n"))], false);
    let seen = Arc::new(Mutex::new(Vec::new()));
    let mut sender = connect(io, &executor);
    let response = run(&executor, sender.send_request(framed(request(head, Http1RequestBody::Empty, &[]), &seen, Ending::Close))).unwrap();
    assert_eq!(response.extensions().get::<String>().unwrap(), "status\n\n");
    assert_eq!(run(&executor, collect(response)).unwrap(), b"line one\nline two\n");
    assert_eq!(*seen.lock().unwrap(), [Seen::Data(b"status".to_vec()), Seen::Data(b"\n\nline one\nline".to_vec()), Seen::Data(b" two\n.\n".to_vec())]);
}

#[test]
fn framing_receives_a_close_before_any_response_byte() {
    let executor = smol::LocalExecutor::new();
    let head: &[u8] = b"GET / HTTP/1.1\r\n\r\n";
    let io = Io::new(vec![(head.len(), Arrival::Data(b""))], false);
    let seen = Arc::new(Mutex::new(Vec::new()));
    let mut sender = connect(io, &executor);
    let error = run(&executor, sender.send_request(framed(request(head, Http1RequestBody::Empty, &[]), &seen, Ending::Close))).unwrap_err();
    assert!(error.is_parse(), "{error:?}");
    assert_eq!(error.source().and_then(|source| source.downcast_ref::<Rejected>()).map(|rejected| rejected.0), Some("no head before the transport ended"));
    assert_eq!(*seen.lock().unwrap(), [Seen::Closed]);
}

#[test]
fn framing_receives_a_read_failure() {
    let executor = smol::LocalExecutor::new();
    let head: &[u8] = b"GET / HTTP/1.1\r\n\r\n";
    let io = Io::new(vec![(head.len(), Arrival::Data(b"partial")), (0, Arrival::Fail)], false);
    let seen = Arc::new(Mutex::new(Vec::new()));
    let mut sender = connect(io, &executor);
    run(&executor, sender.send_request(framed(request(head, Http1RequestBody::Empty, &[]), &seen, Ending::Close))).unwrap_err();
    assert_eq!(*seen.lock().unwrap(), [Seen::Data(b"partial".to_vec()), Seen::Failed(io::ErrorKind::ConnectionReset)]);
}

#[test]
fn framing_waits_for_reads_without_spinning() {
    let executor = smol::LocalExecutor::new();
    let head: &[u8] = b"GET / HTTP/1.1\r\n\r\n";
    let io = Io::new(vec![(head.len(), Arrival::Data(b"no blank line yet"))], true);
    let seen = Arc::new(Mutex::new(Vec::new()));
    let mut sender = connect(io.clone(), &executor);
    let pending = executor.spawn(sender.send_request(framed(request(head, Http1RequestBody::Empty, &[]), &seen, Ending::Close)));
    let mut ticks = 0;
    while executor.try_tick() {
        ticks += 1;
        assert!(ticks < 100, "the connection keeps waking itself while its framing waits");
    }
    assert!(!pending.is_finished());
    assert!(io.reads() < 10, "{} transport reads while waiting", io.reads());
}

#[test]
fn reusable_end_keeps_the_connection_for_the_next_request() {
    let executor = smol::LocalExecutor::new();
    let head: &[u8] = b"GET / HTTP/1.1\r\n\r\n";
    let io = Io::new(vec![(head.len(), Arrival::Data(b"one\n\na\n.\n")), (2 * head.len(), Arrival::Data(b"two\n\nb\n.\n"))], true);
    let seen = Arc::new(Mutex::new(Vec::new()));
    let mut sender = connect(io.clone(), &executor);
    let first = run(&executor, sender.send_request(framed(request(head, Http1RequestBody::Empty, &[]), &seen, Ending::KeepAlive))).unwrap();
    assert_eq!(run(&executor, collect(first)).unwrap(), b"a\n");
    run(&executor, std::future::poll_fn(|cx| sender.poll_ready(cx))).unwrap();
    let second = run(&executor, sender.send_request(framed(request(head, Http1RequestBody::Empty, &[]), &seen, Ending::KeepAlive))).unwrap();
    assert_eq!(second.extensions().get::<String>().unwrap(), "two\n\n");
    assert_eq!(run(&executor, collect(second)).unwrap(), b"b\n");
    assert_eq!(io.written(), [head, head].concat());
}

#[test]
fn unreusable_end_closes_the_connection() {
    let executor = smol::LocalExecutor::new();
    let head: &[u8] = b"GET / HTTP/1.1\r\n\r\n";
    let io = Io::new(vec![(head.len(), Arrival::Data(b"one\n\na\n.\n"))], true);
    let seen = Arc::new(Mutex::new(Vec::new()));
    let mut sender = connect(io, &executor);
    let first = run(&executor, sender.send_request(framed(request(head, Http1RequestBody::Empty, &[]), &seen, Ending::Close))).unwrap();
    assert_eq!(run(&executor, collect(first)).unwrap(), b"a\n");
    while executor.try_tick() {}
    assert!(sender.is_closed());
}

#[test]
fn body_framing_error_is_the_body_error_source() {
    let executor = smol::LocalExecutor::new();
    let head: &[u8] = b"GET / HTTP/1.1\r\n\r\n";
    let io = Io::new(vec![(head.len(), Arrival::Data(b"one\n\na\n.\n"))], true);
    let seen = Arc::new(Mutex::new(Vec::new()));
    let mut sender = connect(io, &executor);
    let response = run(&executor, sender.send_request(framed(request(head, Http1RequestBody::Empty, &[]), &seen, Ending::Fail))).unwrap();
    let error = run(&executor, collect(response)).unwrap_err();
    let rejected = error
        .source()
        .and_then(|source| source.downcast_ref::<io::Error>())
        .and_then(|io| io.get_ref())
        .and_then(|inner| inner.downcast_ref::<Rejected>())
        .map(|rejected| rejected.0);
    assert_eq!(rejected, Some("body rejected"));
}
