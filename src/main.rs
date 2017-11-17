
extern crate bytes;
extern crate num_cpus;

#[macro_use]
extern crate log;
extern crate log4rs;

extern crate futures;
extern crate futures_cpupool;

extern crate serde;
#[macro_use]
extern crate serde_derive;
//extern crate serde_json;
extern crate serde_yaml;

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
mod config;
mod utils;
mod input;
mod output;
mod graph;

fn main() {
    run();
}

fn run() {
    init_log();
    let config_center = match init_config() {
        Ok(t) => t,
        Err(e) => {
            error!("read config error {}", e);
            return
        },
    };

    info!("============collector starting============");

    graph::run(config_center);
}

fn init_log() {
    log4rs::init_file("config/log4rs.yaml", Default::default())
        .unwrap();
}

fn init_config() -> Result<core::config::ConfigCenter, config::ConfigError> {
    let config = config::read_config_from_path("config/collector.yaml")?;
    let config_center = core::config::ConfigCenter::new(config);
    Ok(config_center)
}