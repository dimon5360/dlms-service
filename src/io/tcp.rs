use super::{InterfaceIO, ConfigIO};

struct Tcp {
    _config: ConfigIO
}

impl InterfaceIO for Tcp {
    fn init(&self) {

    }
    fn run(&self) {

    }
}

pub fn new(config: ConfigIO) -> Box<dyn InterfaceIO> {
    return Box::new(Tcp {
        _config: config,
    });
}