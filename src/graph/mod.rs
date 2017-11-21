
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::mpsc::{SyncSender, Receiver};

use core;
use input;
use output;

pub fn run(config_center : core::config::ConfigCenter) {
    let config = config_center.get();
    info!("sync channel size : {}", config.channel_buffer_size);
    let (tx, rx): (SyncSender<core::Event>, Receiver<core::Event>) = mpsc::sync_channel(config.channel_buffer_size);

    let arc_config_center = Arc::new(config_center);
    output::poll_start(rx, &arc_config_center);

    input::udp_serve(tx.clone(), &arc_config_center);
    input::tcp::serve_frame(tx.clone(), &arc_config_center);
}