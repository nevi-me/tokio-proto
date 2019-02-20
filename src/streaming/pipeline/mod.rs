//! Pipelined, streaming protocols.
//!
//! See the crate-level docs for an overview.

use std::io;
use futures::{Stream, Sink};
use tokio::codec::{Framed, Encoder, Decoder};
use tokio::io::{AsyncRead, AsyncWrite};

mod frame;
pub use self::frame::Frame;

mod client;
pub use self::client::ClientProto;

mod server;
pub use self::server::ServerProto;

pub mod advanced;

/// A marker used to flag protocols as being streaming and pipelined.
///
/// This is an implementation detail; to actually implement a protocol,
/// implement the `ClientProto` or `ServerProto` traits in this module.
#[derive(Debug)]
pub struct StreamingPipeline<B>(B);

/// Additional transport details relevant to streaming, pipelined protocols.
///
/// All methods added in this trait have default implementations.
pub trait Transport: 'static +
    Stream<Error = io::Error> +
    Sink<SinkError = io::Error>
{
    /// Allow the transport to do miscellaneous work (e.g., sending ping-pong
    /// messages) that is not directly connected to sending or receiving frames.
    ///
    /// This method should be called every time the task using the transport is
    /// executing.
    fn tick(&mut self) {}

    /// Cancel interest in the current stream
    fn cancel(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl<T, C> Transport for Framed<T,C>
    where T: AsyncRead + AsyncWrite + 'static,
          C: Encoder<Error=io::Error> +
                Decoder<Error=io::Error> + 'static,
{}
