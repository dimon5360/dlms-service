
use dotenv::dotenv;
use log::{debug, info};

use crate::io;
use crate::service::app;
use crate::service::core;

pub trait ProviderInterface {
    fn init(&self);
    fn run(&self);
}

struct Provider {
    _app: Box<dyn app::AppInterface>,
}

impl ProviderInterface for Provider {
    
    fn init(&self) {

        dotenv().ok();
        env_logger::init();

        debug!("init environment");
        debug!("init logger");
        
        debug!("init service provider");

        let version = format!(
            "{}.{}.{}.{}",
            std::env::var("SERVICE_MAJOR_VERSION").unwrap(),
            std::env::var("SERVICE_MINOR_VERSION").unwrap(),
            std::env::var("SERVICE_PATCH_VERSION").unwrap(),
            std::env::var("SERVICE_BUILD_VERSION").unwrap()
        );
        info!("application version {}", version);
        
        self._app.init();
    }
    fn run(&self) {
        info!("run service provider");
        self._app.run();
    }

}

pub fn new() -> Box<dyn ProviderInterface> {
    debug!("new service provider instance");
    
    let serial_io = io::create_serial_io( 
        io::ConfigIO{ config: "".to_string() }
    );
    let _tcp_io = io::create_tcp_io( 
        io::ConfigIO{ config: "".to_string() }
    );

    return Box::new(Provider {
        _app: app::new(core::new(serial_io)),
    });
}
