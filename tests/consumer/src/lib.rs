//! A crate outside the smolith workspace that links `laputa-smol` by path.

/// Returns `value` only when `futures_io::AsyncRead` and `smol::io::AsyncRead` are one trait.
pub fn io_trait_is_shared<T: futures_io::AsyncRead>(value: T) -> T {
    fn assert_same<U: smol::io::AsyncRead>(_: &U) {}
    assert_same(&value);
    value
}

/// Returns `value` only when `futures_core::Stream` and `smol::prelude::Stream` are one trait.
pub fn stream_trait_is_shared<T: futures_core::stream::Stream>(value: T) -> T {
    fn assert_same<U: smol::prelude::Stream>(_: &U) {}
    assert_same(&value);
    value
}

/// Drives a future on the laputa executor.
pub fn block_on<F: std::future::Future>(future: F) -> F::Output {
    smol::block_on(future)
}
