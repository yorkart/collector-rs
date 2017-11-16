
use std::io;
use std::sync::mpsc::SyncSender;
use std::sync::Arc;

use bytes::BytesMut;

use futures::{future, Future};

use time;

use tokio_service::{Service, NewService};

use core;

pub struct FrameService {
    pub sender: Arc<SyncSender<core::Event>>,
}

impl Service for FrameService {
    // These types must match the corresponding protocol types:
    type Request = BytesMut;
    type Response = String;

    // For non-streaming protocols, service errors are always io::Error
    type Error = io::Error;

    // The future for computing the response; box it for simplicity.
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    // Produce a future for computing a response from a request.
    fn call(&self, req: Self::Request) -> Self::Future {
//        info!("request data size: {} -> ", req.len());
        let ts = time::get_time();
        let mills = ts.sec + ts.nsec as i64 / (1000 * 1000);
        info!("timestamp: {}", mills);

        let event = core::Event {
            peer_addr: "".to_owned(),
            time_spec : ts,
            data: req,
        };
        self.sender.send(event).unwrap();

        // In this case, the response is immediate.
        Box::new(future::ok("".to_string()))
    }
}

pub struct FrameNewService {
    pub sender: Arc<SyncSender<core::Event>>,
}

impl NewService for FrameNewService {
    type Request = BytesMut;
    type Response = String;
    type Error = io::Error;
    type Instance = FrameService;

    fn new_service(&self) -> io::Result<Self::Instance> {
        Ok(FrameService {
            sender: self.sender.clone()
        })
    }
}