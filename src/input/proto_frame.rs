use std::io;
use std::net;

use tokio_proto::pipeline::ServerProto;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_core::net::TcpStream;

use super::codec_frame::DataPackage;
use super::codec_frame::FrameCodec;

pub struct FrameProto;

impl<T: NetSocket + 'static> ServerProto<T> for FrameProto {
    // For this protocol style, `Request` matches the `Item` type of the codec's `Decoder`
    type Request = DataPackage;

    // For this protocol style, `Response` matches the `Item` type of the codec's `Encoder`
    type Response = String;

    // A bit of boilerplate to hook in the codec:
    type Transport = Framed<T, FrameCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let local_addr = io.local_addr().unwrap();
        let peer_addr = io.peer_addr().unwrap();

        Ok(io.framed(FrameCodec::new(local_addr, peer_addr)))
    }
}

pub trait NetSocket : AsyncRead + AsyncWrite {
    fn peer_addr(&self) -> io::Result<net::SocketAddr>;
    fn local_addr(&self) -> io::Result<net::SocketAddr>;
}

impl NetSocket for TcpStream {
    fn peer_addr(&self) -> io::Result<net::SocketAddr> {
        self.peer_addr()
    }

    fn local_addr(&self) -> io::Result<net::SocketAddr> {
        self.local_addr()
    }
}