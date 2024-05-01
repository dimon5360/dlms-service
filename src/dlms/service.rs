use log::debug;

use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

use crate::dlms::core;

pub trait Interface: Send + Sync {
    fn run(&self, uuid: u32, core: Box<dyn core::Interface>);
    fn stop(&self, uuid: u32);
}

struct Instance {
    _cores: Mutex<HashMap<u32, (JoinHandle<()>, Sender<bool>)>>,
}

impl Interface for Instance {
    fn run(&self, uuid: u32, core: Box<dyn core::Interface>) {
        let (tx, rx) = mpsc::channel::<bool>();

        let sync_core = Arc::new(Mutex::new(core));
        let core_clone = Arc::clone(&sync_core);

        debug!("run core service");

        let handler = thread::spawn(move || {
            core_clone.lock().unwrap().init(rx);
            core_clone.lock().unwrap().run();
        });

        let mut lock = self._cores.lock().unwrap();
        lock.insert(uuid, (handler, tx));
    }

    fn stop(&self, uuid: u32) {
        debug!("stop core service");

        let mut lock = self._cores.lock().unwrap();
        let (handler, tx) = lock.remove(&uuid).unwrap();

        tx.send(true).unwrap();
        handler.join().unwrap();
    }
}

pub fn new() -> Box<dyn Interface> {
    debug!("new application instance");
    return Box::new(Instance {
        _cores: Mutex::new(HashMap::new()),
    });
}
