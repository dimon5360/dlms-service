
use log::debug;

use crate::service::instance;
use crate::service::core;
use crate::io;

pub trait ProviderInterface {
    fn init(&self) -> Box<dyn instance::Interface>;
    fn core(&self, endpoint: &str) -> Box<dyn core::CoreInterface + Sync + Send>;
}

struct Provider { }

impl ProviderInterface for Provider {
    
    fn init(&self) -> Box<dyn instance::Interface> {
        debug!("init service provider");        
        env_logger::init();
        instance::new()
    }
    
    fn core(&self, endpoint: &str) -> Box<dyn core::CoreInterface + Sync + Send> {
        debug!("new service instance");        
        core::new(io::create_tcp_io( 
            io::ConfigIO{ config: String::from(endpoint) }
        ))
    }
}

pub fn new() -> Box<dyn ProviderInterface> {
    debug!("new service provider instance");    
    return Box::new(Provider {});
}
