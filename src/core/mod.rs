
use bytes::BytesMut;

use time;

pub struct Event {
    pub peer_addr: String,
    pub time_spec: time::Timespec,
    pub data: BytesMut,
    pub data_type: usize,
}

pub mod config;