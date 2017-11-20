
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::mpsc::{SyncSender, Receiver};

use core;
use input;
use output;

pub fn run(config_center : core::config::ConfigCenter) {
    let (tx, rx): (SyncSender<core::Event>, Receiver<core::Event>) = mpsc::sync_channel(100000);

    let arc_config_center = Arc::new(config_center);
    output::poll_start(rx, &arc_config_center);

    input::udp_serve(tx.clone());
    input::tcp::serve_frame(tx.clone(), &arc_config_center);
}