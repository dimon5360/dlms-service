
#[macro_use] extern crate rocket;

use std::io::Result;
use dotenv::dotenv;

mod dlms;
mod io;
mod http;

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

/// TODO:
/// 1. implement service state struct
/// 2. keep dlms service in state
/// 3. add storage to keep running devices

#[rocket::main]
async fn main() -> Result<()> {

    version();

    let service = dlms::provider::new_service();

    http::service::new()
        .init()
        .manage(service as Box<dyn dlms::service::Interface>)
        .launch()
        .await.unwrap();

    Ok(())
}
