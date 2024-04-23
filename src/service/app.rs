use log::debug;

use crate::service::core;

pub trait AppInterface {
    fn init(&self);
    fn run(&self);
}


struct App {
    _core: Box<dyn core::CoreInterface>
}

impl AppInterface for App {
    fn init(&self) {
        debug!("init service application");
        self._core.init();
    }
    fn run(&self) {
        debug!("run service application");
        self._core.run();
    }
}

pub fn new(core: Box<dyn core::CoreInterface>) -> Box<dyn AppInterface> {
    debug!("new application instance");
    return Box::new(App {
        _core: core,
    })
}
