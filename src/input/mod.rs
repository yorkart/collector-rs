mod codec_frame;
mod length_base;

mod proto_frame;

mod service_frame;

pub mod tcp;
pub mod udp;

use std::thread;
use std::sync::mpsc::SyncSender;

use core;

pub fn udp_serve(tx: SyncSender<core::Event>) {
    thread::spawn(move || {
        udp::udp_serve(tx);
    });
}