use std::str;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use bytes::BytesMut;

use kafka;
use kafka::client::{Compression, RequiredAcks};
use kafka::producer::{Producer, Record};

pub fn poll_start(rx: Receiver<BytesMut>) {
    thread::Builder::new()
        .name("kafka-output".to_string())
        .spawn(move || {
            let brokers = vec!["10.100.49.2:9092".to_owned()];
            let topic = "rust-demo".to_string();
            let mut kafka_producer = KafkaProducer::new(brokers).unwrap();

            loop {
                let result = match rx.recv_timeout(Duration::new(3, 0)) {
                    Ok(data) => kafka_producer.send_batch(&topic, data),
                    Err(_) => kafka_producer.flush_batch(),
                };

                match result {
                    Ok(len) => info!("kafka send batch {:?} / {}", len, kafka_producer.get_counter()),
                    Err(e) => error!("kafka send batch error {:?}", e),
                }
            }
        }).unwrap();
}

struct KafkaProducer<'a> {
    producer: Producer,
    queue: Vec<Record<'a, (), Vec<u8>>>,
    capacity: usize,
    counter: usize,
}

impl<'a> KafkaProducer<'a> {
    fn new(_brokers: Vec<String>) -> Result<KafkaProducer<'a>, kafka::Error> {
        let mut _producer = Producer::from_hosts(_brokers)
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::All)
            .with_compression(Compression::SNAPPY)
            .create()?;

        let _capacity = 500;
        let _queue = Vec::with_capacity(_capacity);

        let kafka_producer = KafkaProducer {
            producer: _producer,
            queue: _queue,
            capacity: _capacity,
            counter: 0,
        };

        Ok(kafka_producer)
    }

    fn send_batch(&mut self, _topic: &'a str, data: BytesMut) -> Result<usize, kafka::Error> {
        //        let _topic = topic.clone();
        //        let _key = topic;
        let message = Record {
            key: (),
            partition: -1,
            topic: _topic,
            value: data.to_vec(),
        };

        let queue = &mut self.queue;
        queue.push(message);
        self.counter = self.counter + 1;

        let len = queue.len();
        if len >= self.capacity {
            self.producer.send_all(queue)?;
            queue.clear();
        }

        Ok(len)
    }

    fn flush_batch(&mut self) -> Result<usize, kafka::Error> {
        let queue = &mut self.queue;

        let len = queue.len();
        if len > 0 {
            self.producer.send_all(queue)?;
            queue.clear();
        }

        Ok(len)
    }

    fn get_counter(&mut self) -> usize {
        self.counter
    }
}