
use std::io;

use bytes::BytesMut;

use tokio_io::codec::{Encoder, Decoder};

use super::length_base;

pub struct FrameCodec {
    decoder: length_base::Decoder,
}

impl FrameCodec {
    pub fn new() -> FrameCodec {
        let mut _decoder = length_base::Decoder {
            builder: length_base::Builder::new(),
            state: length_base::DecodeState::Head,
        };

        FrameCodec {
            decoder: _decoder,
        }
    }
}

impl Decoder for FrameCodec {
    type Item = BytesMut;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<BytesMut>> {
        self.decoder.decode(buf)
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