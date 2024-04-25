
use log::{debug, info};
use dotenv::dotenv;

use crate::io;
use crate::service::app;
use crate::service::config;
use crate::service::core;

pub trait ProviderInterface {
    fn init(&mut self);
    fn run(&self);
}

struct Provider {
    _app: Box<dyn app::AppInterface>,
    _config:  Box<dyn config::ConfigInterface>
}

impl ProviderInterface for Provider {
    
    fn init(&mut self) {
        
        dotenv().ok();  
        env_logger::init();

        config::version();
      
        self._config.init();
        self._app.init();
                
        for endpoint in self._config.get_config() {
            self._app.core(core::new(io::create_tcp_io( 
                io::ConfigIO{ config: endpoint.get_port().to_string() }
            )));
        }

        debug!("init service provider");
    }
    fn run(&self) {
        info!("run service provider");
        self._app.run();
    }

}

pub fn new() -> Box<dyn ProviderInterface> {
    debug!("new service provider instance");
    
    return Box::new(Provider {
        _app: app::new(),
        _config: config::new(),
    });
}
