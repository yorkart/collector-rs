
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(rename="env.index")]
    pub env_index: usize,
    #[serde(rename="tcp.threads")]
    pub tcp_threads: usize,
    #[serde(rename="output.workers")]
    pub out_workers: usize,
    pub kafka: KafkaConfig,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KafkaConfig {
    #[serde(rename="batch.size")]
    pub batch_size: usize,
    pub topic: String,
    pub brokers: String,
    #[serde(rename="compression.codec")]
    pub compression_codec: String,
    #[serde(rename="message.max.bytes")]
    pub message_max_bytes: usize,
}

#[derive(Clone)]
pub struct ConfigCenter {
    config: Config,
}

impl ConfigCenter {

    pub fn new(conf: Config) -> ConfigCenter {
        ConfigCenter{
            config: conf,
        }
    }

    pub fn get(&self) ->Config {
        self.config.clone()
    }
}