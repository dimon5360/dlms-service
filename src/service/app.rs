use log::debug;

use crate::service::core;

pub trait AppInterface {
    fn init(&self);
    fn core(&mut self, core: Box<dyn core::CoreInterface>);
    fn run(&self);
}


struct App {
    _cores: Vec<Box<dyn core::CoreInterface>>,
}

impl AppInterface for App {
    fn init(&self) {
        debug!("init service application");
    }

    fn core(&mut self, core: Box<dyn core::CoreInterface>) {
        
        core.init(); 
        core.run();
        
        // self._cores.push(core);        
    }
    
    fn run(&self) {
        debug!("run service application");
    }
}

pub fn new() -> Box<dyn AppInterface> {
    debug!("new application instance");

    return Box::new(App {
        _cores: vec![],
    })
}
