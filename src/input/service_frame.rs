
use std::io;
use std::sync::mpsc::*;
use std::sync::Arc;

use bytes::BytesMut;

use futures::{future, Future};

use tokio_service::{Service, NewService};

pub struct FrameService {
    pub sender: Arc<SyncSender<BytesMut>>,
}

impl FrameService {
    pub fn new(_sender: Arc<SyncSender<BytesMut>>) -> FrameService {
        FrameService {
            sender: _sender,
        }
    }
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
        info!("request data size: {} -> ", req.len());

        self.sender.send(req);

        // In this case, the response is immediate.
        Box::new(future::ok("ok".to_string()))
    }
}

pub struct FrameNewService {
    pub sender: Arc<SyncSender<BytesMut>>,
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