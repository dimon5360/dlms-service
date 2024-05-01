use std::thread;
use std::time::Duration;

use super::ConfigIO;

pub struct Tcp {
    _config: ConfigIO
}

impl Tcp {
    pub fn init(&self) {

    }

    pub fn run(&self) {
        println!("io running...");
        thread::sleep(Duration::from_secs(5));
    }

    pub fn stop(&self) {
        println!("io stopping...")
    }
}

pub fn new(config: ConfigIO) -> Tcp {
    return Tcp {
        _config: config,
    };
}