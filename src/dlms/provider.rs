
use log::debug;

use crate::dlms::service;
use crate::dlms::core;
use crate::io;

pub fn new_service() -> Box<dyn service::Interface> {
    debug!("init service provider");
    env_logger::init();
    service::new()
}

pub fn new_instance(endpoint: &str) -> Box<dyn core::Interface> {
    debug!("new service instance");
    core::new(io::create_tcp_io(
        io::ConfigIO{ config: String::from(endpoint) }
    ))
}
