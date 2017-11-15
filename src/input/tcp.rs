
use std::sync::Arc;
use std::sync::mpsc::*;

use bytes::BytesMut;

use tokio_proto::TcpServer;

use super::proto_frame::FrameProto;
use super::service_frame::FrameNewService;

pub fn serve_frame(tx: SyncSender<BytesMut>) {
    // Specify the localhost address
    let addr = "0.0.0.0:12345".parse().unwrap();

    let proto = FrameProto;// { codec_builder: builder };

    // The builder requires a protocol and an address
    let server = TcpServer::new(proto, addr);

    let pair: Arc<SyncSender<BytesMut>> = Arc::new(tx);
    let frame_new_service = FrameNewService {
        sender: pair,
    };

    // We provide a way to *instantiate* the service for each new
    // connection; here, we just immediately return a new instance.
    server.serve(frame_new_service);
}