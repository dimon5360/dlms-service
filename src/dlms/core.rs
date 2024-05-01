
use log::debug;
use crate::io;

pub trait CoreInterface {
    fn init(&self);
    fn run(&self);
}

struct Core {
    _io: io::tcp::Tcp
}

impl CoreInterface for Core {
    fn init(&self) {
        debug!("init service core");
        self._io.init();
    }
    
    fn run(&self) {
        debug!("run service core");

        println!("core running...");
        self._io.run();
        println!("core stopping...");
    }
}

pub fn new(io: io::tcp::Tcp) -> Box<dyn CoreInterface + Sync + Send> {
    debug!("new application core instance");
    return Box::new(Core {
        _io: io,
    });
}
