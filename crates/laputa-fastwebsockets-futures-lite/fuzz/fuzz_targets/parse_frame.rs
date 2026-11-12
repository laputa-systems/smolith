#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let (mut read, _write) = fastwebsockets::after_handshake_split(
        futures_lite::io::Cursor::new(data),
        futures_lite::io::sink(),
        fastwebsockets::Role::Server,
    );
    read.set_max_message_size(u16::MAX as usize);
    let _ = futures_lite::future::block_on(read.read_frame(&mut |_| {
        std::future::ready(Ok::<(), std::convert::Infallible>(()))
    }));
});
