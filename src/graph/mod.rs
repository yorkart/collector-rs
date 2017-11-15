
use std::sync::mpsc;
use std::sync::mpsc::{SyncSender, Receiver};

use bytes::BytesMut;

use input;
use output;

pub fn run() {
    let (tx, rx): (SyncSender<BytesMut>, Receiver<BytesMut>) = mpsc::sync_channel(100000);

    output::kafka::poll_start(rx);

    input::tcp::serve_frame(tx.clone());
}