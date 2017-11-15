use std::io;

use bytes::BytesMut;

use tokio_proto::pipeline::ServerProto;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;

use super::codec_frame::FrameCodec;

pub struct FrameProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for FrameProto {
    // For this protocol style, `Request` matches the `Item` type of the codec's `Decoder`
    type Request = BytesMut;

    // For this protocol style, `Response` matches the `Item` type of the codec's `Encoder`
    type Response = String;

    // A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, FrameCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(FrameCodec::new()))
    }
}