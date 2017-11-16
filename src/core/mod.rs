
use bytes::BytesMut;

use time;

pub struct Event {
    pub peer_addr: String,
    pub time_spec: time::Timespec,
    pub data: BytesMut,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    #[serde(rename="tcp.threads")]
    tcp_threads: usize,
    #[serde(rename="out.workers")]
    out_workers: usize,
    kafka: KafkaConfig,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct KafkaConfig {
    brokers: String,
    #[serde(rename="compression.codec")]
    compression_codec: String,
    #[serde(rename="message.max.bytes")]
    message_max_bytes: usize,
}
