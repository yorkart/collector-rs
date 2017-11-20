
use std::io;
use std::sync::mpsc::SyncSender;
use std::convert::From;

use bytes::BytesMut;
//use bytes::BufMut;

use futures::{Future, Poll};

use time;

use tokio_core::net::UdpSocket;
use tokio_core::reactor::Core;

use core;

pub struct Server {
    socket : UdpSocket,
    buf: Vec<u8>,
    tx: SyncSender<core::Event>,
}

impl Future for Server {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        loop {
            let (size, peer_addr) = try_nb!(self.socket.recv_from(&mut self.buf));
            info!("len : {} / {}", size, self.buf.len());

            let event = core::Event {
                peer_addr: peer_addr.ip().to_string(),
                time_spec : time::get_time(),
                data: BytesMut::from(&self.buf[0..size]),
                data_type: 20,
            };

//            self.tx.send(event).unwrap();
            let rt = self.tx.try_send(event).is_err();
            if rt {
                error!("try send event to buffer error");
            }
        }
    }
}

pub fn udp_serve(tx: SyncSender<core::Event>) {
    let addr = "0.0.0.0:36365".parse().unwrap();

    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let socket = UdpSocket::bind(&addr, &handle).unwrap();

    info!("udp server listening on: {}", socket.local_addr().unwrap());

    core.run(Server{
        socket,
        buf: vec![0; 1024*1024 * 9],
        tx,
    }).unwrap();
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::sync::mpsc;
    use std::sync::mpsc::{SyncSender, Receiver};

    use super::*;

    #[test]
    pub fn udp_serve_test() {
        let (tx, rx): (SyncSender<core::Event>, Receiver<core::Event>) = mpsc::sync_channel(100000);

        thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(event) => {
                        println!("recv {}", event.data.len());

                        for x in event.data {
                            print!("{},", x);
                        }
                    },
                    Err(e) => println!("error {:?}", e),
                };
            }
        });
        udp_serve(tx);
    }
}