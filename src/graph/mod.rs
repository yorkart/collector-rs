
use std::sync::mpsc;
use std::sync::mpsc::{SyncSender, Receiver};

use core;
use input;
use output;

pub fn run() {
    let (tx, rx): (SyncSender<core::Event>, Receiver<core::Event>) = mpsc::sync_channel(100000);

    output::poll_start(rx);

    input::tcp::serve_frame(tx.clone());
}