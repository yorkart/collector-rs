
use bytes::BytesMut;

use time;

pub struct Event {
    pub peer_addr: String,
    pub time_spec: time::Timespec,
    pub data: BytesMut,
    pub data_type: usize,
}

pub const BASE_V2: usize = 64;

pub const AGENT_INFO: usize = 1;
pub const AGENT_INFO_V2: usize = AGENT_INFO + BASE_V2;

pub const METADATA_API: usize = 8;
pub const METADATA_API_V2: usize = METADATA_API + BASE_V2;

pub const AGENT_STAT: usize = 9;
pub const AGENT_STAT_V2: usize = AGENT_STAT + BASE_V2;

pub const TRACE_ORIGIN: usize = 20;
pub const TRACE_ORIGIN_V2: usize = TRACE_ORIGIN + BASE_V2;

//pub static  TRACE_SNAPPY : i32 = 21;
//pub static  TRACE_SNAPPY_V2 : i32 = TRACE_SNAPPY + BASE_V2;

pub mod config;