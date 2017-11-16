
use std::sync::Arc;
use std::sync::mpsc::SyncSender;

use tokio_proto::TcpServer;

use super::proto_frame::FrameProto;
use super::service_frame::FrameNewService;

use core;

pub fn serve_frame(tx: SyncSender<core::Event>) {
    // Specify the localhost address
    let addr = "0.0.0.0:9019".parse().unwrap();

    let proto = FrameProto;// { codec_builder: builder };

    // The builder requires a protocol and an address
    let mut server = TcpServer::new(proto, addr);
    server.threads(5);

    let pair: Arc<SyncSender<core::Event>> = Arc::new(tx);
    let frame_new_service = FrameNewService {
        sender: pair,
    };

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    server.serve(frame_new_service);
}
