use std::sync::mpsc;
use std::sync::mpsc::RecvTimeoutError;

use bytes::BytesMut;

use futures::Future;

use rdkafka::error::KafkaError;
use rdkafka::client::EmptyContext;
use rdkafka::config::ClientConfig;
use rdkafka::producer::FutureProducer;
use rdkafka::producer::future_producer::DeliveryFuture;

pub fn worker(rx: mpsc::Receiver<Result<BytesMut, RecvTimeoutError>>) {
    let brokers = "10.100.49.2:9092,10.100.49.3:9092,10.100.49.4:9092";
    let topic = "rust-demo".to_string();
    let key = "key".to_owned();

    let mut kafka_producer = RDKafkaProducer::new(brokers);

    loop {
        let rt = rx.recv().unwrap();
        let result = match rt {
            Ok(data) => kafka_producer.send_batch(&topic, &key, data),
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
    fn new(brokers: &str) -> RDKafkaProducer {
        let mut _producer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("produce.offset.report", "true")
            .set("request.required.acks", "-1")
            .set("compression.codec", "snappy")
            .set("message.max.bytes", "9437184")
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

    fn send_batch(&mut self, topic: &str, key: &str, data: BytesMut) -> Result<usize, KafkaError> {
        let delivery_future = self.producer.send_copy(
            &topic.to_owned(),
            None,
            Some(&data.to_vec()),
            Some(&key.to_owned()),
            None,
            0);

        let mut flush = false;
        let len;
        {
            let queue = &mut self.queue;

            queue.push(delivery_future);
            len = queue.len();

            if len >= self.capacity {
                for mut df in queue {
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
            for mut df in queue {
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