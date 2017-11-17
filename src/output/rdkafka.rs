
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::mpsc::RecvTimeoutError;

use bytes::BigEndian;
use bytes::ByteOrder;

use futures::Future;

use rdkafka::error::KafkaError;
use rdkafka::client::EmptyContext;
use rdkafka::config::ClientConfig;
use rdkafka::producer::FutureProducer;
use rdkafka::producer::future_producer::DeliveryFuture;

use core;
use utils;

pub fn worker(rx: mpsc::Receiver<Result<core::Event, RecvTimeoutError>>, config_center: Arc<core::config::ConfigCenter>) {
    let config = config_center.get();
    let topic = config.kafka.topic.clone();

    let mut kafka_producer = RDKafkaProducer::new(config.kafka);

    loop {
        let rt = rx.recv().unwrap();
        let result = match rt {
            Ok(event) => kafka_producer.send_batch(&topic, event),
            Err(_) => kafka_producer.flush_batch(),
        };

        match result {
            Ok(len) => info!("kafka send batch {:?} / {}", len, kafka_producer.get_counter()),
            Err(e) => error!("kafka send batch error {:?}", e),
        }
    }
}

struct RDKafkaProducer {
    producer: FutureProducer<EmptyContext>,
    queue: Vec<DeliveryFuture>,
    capacity: usize,
    counter: usize,
}

impl RDKafkaProducer {
    fn new(config: core::config::KafkaConfig) -> RDKafkaProducer {
        let mut _producer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .set("produce.offset.report", "true")
            .set("request.required.acks", "-1")
            .set("compression.codec", &config.compression_codec)
            .set("message.max.bytes", format!("{}", config.message_max_bytes).as_ref())
            .create::<FutureProducer<_>>()
            .expect("Producer creation error");

        let _capacity = 10;
        let _queue = Vec::with_capacity(_capacity);

        RDKafkaProducer {
            producer: _producer,
            queue: _queue,
            capacity: _capacity,
            counter: 0,
        }
    }

    fn send_batch(&mut self, topic: &str, event: core::Event) -> Result<usize, KafkaError> {
        let mut data = event.data;
        let mills = utils::get_mills(event.time_spec);

        let mut header = data.split_to(9);

        header.split_to(4); // message_id
        let type_buf = header.slice(1);

        let data_type = BigEndian::read_i32(type_buf.as_ref());

        let key = format!("{}#{}#{}#{}", mills, 5, &event.peer_addr, data_type);
        info!("event key: {}", &key);

        let delivery_future = self.producer.send_copy(
            &topic.to_owned(),
            None,
            Some(&data.to_vec()),
            Some(&key),
            Some(mills),
            0);

        let mut flush = false;
        let len;
        {
            let queue = &mut self.queue;

            queue.push(delivery_future);
            len = queue.len();

            if len >= self.capacity {
                for df in queue {
                    let delivery_status = df.wait().unwrap();
                    match delivery_status {
                        Ok((_, _)) => {},
                        Err((err, msg)) => error!("send error: {:?}, message: {:?}", err, msg),
                    }
                }

                flush = true;
            }
        }
        {
            let queue = &mut self.queue;
            if flush {
                queue.clear();
            }
        }

        Ok(len)
    }

    fn flush_batch(&mut self) -> Result<usize, KafkaError> {
        let len;
        {
            let queue = &mut self.queue;
            len = queue.len();
            for df in queue {
                let delivery_status = df.wait().unwrap();
                match delivery_status {
                    Ok((_, _)) => {}
                    Err((err, msg)) => error!("send error: {:?}, message: {:?}", err, msg),
                }
            }
        }
        {
            let queue = &mut self.queue;
            queue.clear();
        }

        Ok(len)
    }

    fn get_counter(&mut self) -> usize {
        self.counter
    }
}