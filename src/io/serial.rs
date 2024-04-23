use super::{InterfaceIO, ConfigIO};

struct Serial {
    _config: ConfigIO
}

impl InterfaceIO for Serial {
    fn init(&self) {

    }
    fn run(&self) {

    }
}

pub fn new(config: ConfigIO) -> Box<dyn InterfaceIO> {
    return Box::new(Serial {
        _config: config,
    });
}