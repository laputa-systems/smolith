use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::sync::Arc;

use bytes::Bytes;

/// A request head that the HTTP/1 client writes byte for byte.
///
/// Insert it into a request's extensions to send a request line and header
/// block exactly as given, keeping their case, order and bytes. The client
/// ignores the request's method, URI, version and header map on the wire, and
/// it frames the request body as `body` declares, whatever the body's size
/// hint says. The head must end with the empty line that terminates the header
/// block and must carry the framing headers `body` implies.
///
/// A [`Http1RequestBody::Chunked`] request without body frames sends only the
/// last chunk. A request whose body contradicts the declared framing, a body on
/// [`Http1RequestBody::Empty`] or no body on a nonzero
/// [`Http1RequestBody::Length`], fails with a user body error.
#[derive(Clone, Debug)]
pub struct Http1RequestHead {
    head: Bytes,
    body: Http1RequestBody,
}

impl Http1RequestHead {
    /// Creates a request head from its exact bytes and the body framing it declares.
    pub fn new(head: Bytes, body: Http1RequestBody) -> Self {
        Http1RequestHead { head, body }
    }

    pub(crate) fn head(&self) -> &Bytes {
        &self.head
    }

    pub(crate) fn body(&self) -> Http1RequestBody {
        self.body
    }
}

/// How the HTTP/1 client frames the body that follows an [`Http1RequestHead`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Http1RequestBody {
    /// The request has no body.
    Empty,
    /// The body is exactly this many bytes, written as they are.
    Length(u64),
    /// Each body data frame is written as one chunk, followed by the last chunk.
    Chunked,
}

/// One read from the transport, as the HTTP/1 client handed it to a [`ResponseFraming`].
#[derive(Debug)]
pub enum TransportRead<'read> {
    /// The bytes one read returned.
    Data(&'read [u8]),
    /// The read returned end of stream.
    Closed,
    /// The read failed.
    Failed(&'read io::Error),
}

/// What a [`ResponseFraming`] has for the response body.
#[derive(Debug)]
pub enum BodyProgress {
    /// The body needs another transport read.
    NeedRead,
    /// Body bytes for the caller.
    Data(Bytes),
    /// The body is complete; `reusable` says whether the connection may carry another request.
    End {
        /// Whether the connection may carry another request.
        reusable: bool,
    },
}

/// Parses one response head and frames its body in place of the HTTP/1 client's own parser.
///
/// The client passes every transport read to [`receive`](Self::receive), in
/// order, without buffering across them, and asks for the head or the next
/// body step after each one. After a [`TransportRead::Closed`] or
/// [`TransportRead::Failed`], `poll_head` must return a head or an error and
/// `poll_body` must return data, the end or an error. An error from either
/// method fails the response: from `poll_head` it is the cause of a parse
/// [`Error`](crate::Error), and from `poll_body` it is the inner error of the
/// body's I/O error.
pub trait ResponseFraming: Send + 'static {
    /// Accepts the result of one transport read.
    fn receive(&mut self, read: TransportRead<'_>);

    /// Returns the response head once it is complete, or `None` to wait for another read.
    ///
    /// Only the status, version, headers and extensions of the returned response are used.
    ///
    /// # Errors
    ///
    /// Returns the reason the response is rejected; the request fails with it.
    fn poll_head(&mut self) -> Result<Option<http::Response<()>>, Box<dyn StdError + Send + Sync>>;

    /// Returns the next step of the response body.
    ///
    /// # Errors
    ///
    /// Returns the reason the body is rejected; the body fails with it and the connection closes.
    fn poll_body(&mut self) -> Result<BodyProgress, Box<dyn StdError + Send + Sync>>;
}

type Factory = dyn Fn() -> Box<dyn ResponseFraming> + Send + Sync;

#[derive(Clone)]
pub(crate) struct ResponseFramingFactory(Arc<Factory>);

impl ResponseFramingFactory {
    pub(crate) fn create(&self) -> Box<dyn ResponseFraming> {
        (self.0)()
    }
}

impl fmt::Debug for ResponseFramingFactory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ResponseFramingFactory")
    }
}

/// Parses and frames this request's HTTP/1 response with the framing that `framing` creates.
///
/// The HTTP/1 client calls `framing` once, when it writes the request. Other
/// protocols ignore it.
///
/// # Example
///
/// ```
/// # use hyper::ext::{BodyProgress, ResponseFraming, TransportRead};
/// struct Empty;
///
/// impl ResponseFraming for Empty {
///     fn receive(&mut self, _read: TransportRead<'_>) {}
///     fn poll_head(&mut self) -> Result<Option<http::Response<()>>, Box<dyn std::error::Error + Send + Sync>> {
///         Ok(Some(http::Response::new(())))
///     }
///     fn poll_body(&mut self) -> Result<BodyProgress, Box<dyn std::error::Error + Send + Sync>> {
///         Ok(BodyProgress::End { reusable: false })
///     }
/// }
///
/// let mut req = hyper::Request::new(());
/// hyper::ext::frame_response(&mut req, || Box::new(Empty));
/// ```
pub fn frame_response<B, F>(req: &mut http::Request<B>, framing: F)
where
    F: Fn() -> Box<dyn ResponseFraming> + Send + Sync + 'static,
{
    req.extensions_mut()
        .insert(ResponseFramingFactory(Arc::new(framing)));
}
