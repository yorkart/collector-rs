use std::io;
use std::net::SocketAddr;

use bytes::BytesMut;

use tokio_io::codec::{Encoder, Decoder};

use super::length_base;

pub struct DataPackage {
    pub data: BytesMut,
    pub local_addr: SocketAddr,
    pub peer_addr: SocketAddr,
}

pub struct FrameCodec {
    decoder: length_base::Decoder,
    local_addr: SocketAddr,
    peer_addr: SocketAddr,
}

impl FrameCodec {
    pub fn new(_local_addr: SocketAddr, _peer_addr: SocketAddr) -> FrameCodec {
        let mut _decoder = length_base::Decoder {
            builder: length_base::Builder::new(),
            state: length_base::DecodeState::Head,
        };

        FrameCodec {
            decoder: _decoder,
            local_addr: _local_addr,
            peer_addr: _peer_addr,
        }
    }
}

impl Decoder for FrameCodec {
    type Item = DataPackage;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<DataPackage>> {
        match self.decoder.decode(buf) {
            Ok(opt) => {
                match opt {
                    Some(_data) =>
                        Ok(
                            Some(DataPackage {
                                data: _data,
                                local_addr: self.local_addr.clone(),
                                peer_addr: self.peer_addr.clone(),
                            })
                        ),
                    None => Ok(None)
                }
            }
            Err(e) => Err(e)
        }
    }
}

impl Encoder for FrameCodec {
    type Item = String;
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) -> io::Result<()> {
        buf.extend(msg.as_bytes());
        buf.extend(b"\n");
        Ok(())
    }
}