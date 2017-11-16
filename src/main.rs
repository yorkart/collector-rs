
extern crate bytes;
extern crate num_cpus;

#[macro_use]
extern crate log;
extern crate log4rs;

extern crate futures;
extern crate futures_cpupool;

extern crate serde;
//#[macro_use]
//extern crate serde_derive;
//extern crate serde_json;
//extern crate serde_yaml;

extern crate tokio_core;
//#[macro_use]
extern crate tokio_io;
extern crate tokio_service;
extern crate tokio_proto;
extern crate tokio_timer;

extern crate http;
extern crate httparse;

extern crate time;

extern crate kafka;
extern crate rdkafka;

mod core;
mod utils;
mod input;
mod output;
mod graph;

fn main() {
    run();
}

fn run() {
    init_log();

    info!("============collector starting============");

    graph::run();
}

fn init_log() {
    log4rs::init_file("config/log4rs.yaml", Default::default())
        .unwrap();
}