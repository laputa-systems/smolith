use smolith_consumer::{block_on, io_trait_is_shared, stream_trait_is_shared};

#[test]
fn tcp_roundtrip_uses_the_shared_io_trait() {
    block_on(async {
        let listener = smol::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = smol::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buf = [0; 4];
            smol::io::AsyncReadExt::read_exact(&mut stream, &mut buf)
                .await
                .unwrap();
            buf
        });
        let client = smol::net::TcpStream::connect(address).await.unwrap();
        let mut client = io_trait_is_shared(client);
        smol::io::AsyncWriteExt::write_all(&mut client, b"ping")
            .await
            .unwrap();
        assert_eq!(&server.await, b"ping");
    });
}

#[test]
fn channel_uses_the_shared_stream_trait() {
    let (sender, receiver) = smol::channel::bounded::<u8>(1);
    let receiver = stream_trait_is_shared(receiver);
    block_on(async {
        sender.send(7).await.unwrap();
        assert_eq!(receiver.recv().await.unwrap(), 7);
    });
}

#[test]
fn unblock_writes_through_laputa_blocking() {
    block_on(async {
        let mut writer = smol::Unblock::new(Vec::<u8>::new());
        smol::io::AsyncWriteExt::write_all(&mut writer, b"ok")
            .await
            .unwrap();
        let buffer = writer.into_inner().await;
        assert_eq!(buffer, b"ok");
    });
}
