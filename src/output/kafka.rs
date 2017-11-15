
use std::str;
use std::sync::mpsc::Receiver;
use std::thread;

use bytes::BytesMut;

pub fn poll_start(rx : Receiver<BytesMut>) {
    thread::spawn(move || {
        rx.iter().for_each(move |data| {
            let ss = str::from_utf8(&data).unwrap().to_string();
            info!("receive: {} -> ", ss);
        });
    });
}