use std::str;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use bytes::BytesMut;

use kafka;
use kafka::client::{Compression, RequiredAcks};
use kafka::producer::{Producer, Record};

pub fn poll_start(rx: Receiver<BytesMut>) {
    thread::spawn(move || {
        let brokers = vec!["10.100.49.2:9092".to_owned()];
        let topic = "rust-demo".to_string();
        let mut kafka_producer = KafkaProducer::new(brokers).unwrap();

        rx.iter().for_each(|data| {
//            let ss = str::from_utf8(&data).unwrap().to_string();
//            info!("receive: {} -> ", ss);

            kafka_producer.send_batch(&topic, data).unwrap();
        });
    });
}

struct KafkaProducer<'a> {
    producer: Producer,
    queue: Vec<Record<'a, (), Vec<u8>>>,
}

impl<'a> KafkaProducer<'a> {
    fn new(_brokers: Vec<String>) -> Result<KafkaProducer<'a>, kafka::Error> {
        let mut _producer = Producer::from_hosts(_brokers)
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::All)
            .with_compression(Compression::SNAPPY)
            .create()?;

        let _queue = Vec::with_capacity(200);

        let kafka_producer = KafkaProducer {
            producer: _producer,
            queue: _queue,
        };

        Ok(kafka_producer)
    }

    fn send_batch(&mut self ,_topic: &'a str, data: BytesMut) -> Result<(), kafka::Error>{
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

        if queue.len() > 200 {
            self.producer.send_all(queue)?;
            queue.clear();
        }

        Ok(())
    }
}