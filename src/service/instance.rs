use log::debug;

use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use crate::service::core;

pub trait Interface {
    fn run(&self, core: Box<dyn core::CoreInterface + Sync + Send>) -> JoinHandle<()>;
}

struct Instance {}

impl Interface for Instance {
    
    fn run(&self, core: Box<dyn core::CoreInterface + Sync + Send>) -> JoinHandle<()> {
                
        let sync_core = Arc::new(Mutex::new(core));          
        let core_clone =  Arc::clone(&sync_core);

        debug!("run service application");

        let handler = thread::spawn(move || {
            core_clone.lock().unwrap().init();
            core_clone.lock().unwrap().run();
        });

        handler
    }
}

pub fn new() -> Box<dyn Interface> {
    debug!("new application instance");

    return Box::new(Instance {})
}
