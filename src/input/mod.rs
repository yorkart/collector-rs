mod codec_frame;
mod length_base;

mod proto_frame;

mod service_frame;

pub mod tcp;
pub mod udp;

use std::thread;
use std::sync::Arc;
use std::sync::mpsc::SyncSender;

use core;

pub fn udp_serve(tx: SyncSender<core::Event>, config_center: &Arc<core::config::ConfigCenter>) {
    let config_center = config_center.clone();
    thread::Builder::new().name("udp-server".to_owned()).spawn(move || {
        udp::udp_serve(tx, &config_center);
    }).unwrap();
}