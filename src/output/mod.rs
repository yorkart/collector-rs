

use std::sync::mpsc;
use std::sync::mpsc::Receiver;

use std::thread;
use std::time::Duration;

use core;

//pub mod kafka;
pub mod rdkafka;


pub fn poll_start(rx: Receiver<core::Event>) {
    thread::Builder::new()
        .name("kafka-output".to_string())
        .spawn( move || {
            poll_with_multi_worker(rx, 5);
        }).unwrap();
}

fn poll_with_multi_worker(rx :Receiver<core::Event>, num_threads: i32) {
    let recv_timeout_sec = 3;

    let mut channels = Vec::new();
    for n in 0..num_threads {
        let (worker_tx, worker_rx) = mpsc::channel();
        channels.push(worker_tx);
        thread::Builder::new()
            .name(format!("kafka-worker-{}", n).to_string())
            .spawn(move || rdkafka::worker(worker_rx)).unwrap();
    }

    let mut channel_index = 0;
    loop {
        let rt = rx.recv_timeout(Duration::new(recv_timeout_sec, 0));

        let index = if channel_index < num_threads {
            channel_index as usize
        } else {
            channel_index = 0;
            channel_index as usize
        };

        let worker_tx = &channels[index];
        worker_tx.send(rt).unwrap();

        channel_index = channel_index + 1;
    }
}