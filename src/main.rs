use std::io::Result;
use std::thread::JoinHandle;
use dotenv::dotenv;

mod service;
mod io;

fn version() {
    dotenv().ok();

    let version = format!(
        "Application version v.{}.{}.{}.{}",
        std::env::var("SERVICE_MAJOR_VERSION").unwrap(),
        std::env::var("SERVICE_MINOR_VERSION").unwrap(),
        std::env::var("SERVICE_PATCH_VERSION").unwrap(),
        std::env::var("SERVICE_BUILD_VERSION").unwrap()
    );

    println !("{}", version);
}

fn main() -> Result<()>{

    version();

    let provider = service::provider::new();
    let service = provider.init();

    let mut threads = Vec::<JoinHandle<()>>::new();

    threads.push(service.run(provider.core("localhost:40400")));
    threads.push(service.run(provider.core("localhost:40401")));

    for t in threads {
        t.join().unwrap();
    }

    Ok(())
}
