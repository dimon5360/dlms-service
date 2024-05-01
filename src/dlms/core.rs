use log::debug;
use std::sync::Mutex;

use std::sync::mpsc::Receiver;

use crate::io;

pub trait Interface: Send + Sync {
    fn init(&self, tx: Receiver<bool>);
    fn run(&self);
    fn stop(&self);
}

struct Core {
    _io: io::tcp::Tcp,
    _is_running: Mutex<Option<Receiver<bool>>>,
}

impl Interface for Core {
    fn init(&self, tx: Receiver<bool>) {
        debug!("init service core");
        self._io.init();

        let mut lock = self._is_running.lock().unwrap();
        *lock = Some(tx);
    }

    fn run(&self) {
        debug!("run service core");

        println!("core running...");

        loop {
            println!("Suspending...");
            let lock = self._is_running.lock().unwrap();

            let rx = lock.as_ref();

            match rx.unwrap().recv() {
                Ok(res) => {
                    if res {
                        println!("Terminating.");
                        break;
                    }
                    println!("Working...");
                    self._io.run();
                }
                Err(_) => {
                    println!("Terminating.");
                    break;
                }
            }
        }
        println!("core stopping...");
    }

    fn stop(&self) {
        let mut lock = self._is_running.lock().unwrap();
        *lock = None;

        println!("need to stop");
        self._io.stop();
    }
}

pub fn new(io: io::tcp::Tcp) -> Box<dyn Interface> {
    debug!("new application core instance");
    return Box::new(Core {
        _io: io,
        _is_running: Mutex::new(None),
    });
}
