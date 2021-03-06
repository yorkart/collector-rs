use std::sync::Arc;
use std::sync::mpsc;
use std::sync::mpsc::RecvTimeoutError;

//use bytes::BigEndian;
//use bytes::ByteOrder;

use futures::Future;

use rdkafka::error::KafkaError;
use rdkafka::client::EmptyContext;
use rdkafka::config::ClientConfig;
use rdkafka::producer::FutureProducer;
use rdkafka::producer::future_producer::DeliveryFuture;

use core;
use utils;
use super::common::PackageTypeMap;

pub fn worker(rx: mpsc::Receiver<Result<core::Event, RecvTimeoutError>>, config_center: Arc<core::config::ConfigCenter>) {
    let config = config_center.get();

    let mut kafka_producer = RDKafkaProducer::new(config.kafka, config.env_index);

    loop {
        let rt = rx.recv().unwrap();
        let result = match rt {
            Ok(event) => kafka_producer.send_batch(event),
            Err(_) => kafka_producer.flush_batch(),
        };

        match result {
            Ok(/*len*/ _) => {} // info!("kafka send batch {:?} / {}", len, kafka_producer.get_counter()),
            Err(e) => error!("kafka send batch error {:?}", e),
        }
    }
}

struct RDKafkaProducer<'a> {
    env_index: usize,
    producer: FutureProducer<EmptyContext>,
    queue: Vec<DeliveryFuture>,
    capacity: usize,
    counter: usize,
    package_type_map_topic: PackageTypeMap<'a>,
}

impl<'a> RDKafkaProducer<'a> {
    fn new(config: core::config::KafkaConfig, env_index: usize) -> RDKafkaProducer<'a> {
        let mut _producer = ClientConfig::new()
            .set("bootstrap.servers", &config.brokers)
            .set("produce.offset.report", "true")
            .set("request.required.acks", "-1")
            .set("compression.codec", &config.compression_codec)
            .set("message.max.bytes", format!("{}", config.message_max_bytes).as_ref())
            .create::<FutureProducer<_>>()
            .expect("Producer creation error");

//        let _capacity = 10;
        let _queue = Vec::with_capacity(config.batch_size);

        RDKafkaProducer {
            env_index,
            producer: _producer,
            queue: _queue,
            capacity: config.batch_size,
            counter: 0,
            package_type_map_topic: PackageTypeMap::new(),
        }
    }

    fn send_batch(&mut self, mut event: core::Event) -> Result<usize, KafkaError> {
        self.counter = self.counter + 1;

        let mut data = event.data;
        let mills = utils::get_mills(event.time_spec);

        if event.data_type == 0 {
            let mut header = data.split_to(9);

            let _ /*message_id_buf*/ = header.split_to(4);
            let type_buf = header.split_to(1);

            //            let message_id = BigEndian::read_i32(message_id_buf.as_ref());
            let data_type = type_buf.as_ref().to_vec()[0] as usize;

            event.data_type = data_type;
        }

        let data_type = event.data_type;

        let key = format!("{}#{}#{}#{}", mills, self.env_index, &event.peer_addr, data_type);
        //        info!("event key: {}, messageId: {}", &key, message_id);

        let _topic = self.package_type_map_topic.get_topic(data_type);
        //        info!("event topic: {}, data_type: {}", _topic, data_type);

        let delivery_future = self.producer.send_copy(
            &_topic.to_owned(),
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
                        Ok((_, _)) => {}
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
                info!("flush kafka batch: {} / {}", len, self.counter);
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

            info!("flush kafka batch: {} / {}", len, self.counter);
        }

        Ok(len)
    }

//    fn get_counter(&mut self) -> usize {
//        self.counter
//    }
}