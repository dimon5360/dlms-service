use std::io::Result;

mod service;
mod io;

fn main() -> Result<()>{

    let mut service = service::provider::new();
    
    service.init();
    service.run();
        
    Ok(())
}
