
use log::debug;

use crate::io;

pub trait CoreInterface {
    fn init(&self);
    fn run(&self);
}


struct Core {
    _io: Box<dyn io::InterfaceIO>
}

impl CoreInterface for Core {
    fn init(&self) {
        debug!("init service core");
    }
    fn run(&self) {
        debug!("run service core");
    }

}

pub fn new(io: Box<dyn io::InterfaceIO>) -> Box<dyn CoreInterface> {
    debug!("new application core instance");
    return Box::new(Core {
        _io: io
    })
}
