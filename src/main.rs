use std::io::Result;

mod service;
mod io;

fn main() -> Result<()>{
    let provider = service::provider::new();

    provider.init();
    provider.run();

    Ok(())
}
