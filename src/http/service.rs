
use rocket::Build;

use crate::http::device;

pub trait Interface {
    fn init(&self) -> rocket::Rocket<Build>;
}

struct Service;

impl Interface for Service {
    fn init(&self) -> rocket::Rocket<Build> {
        rocket::build()
        .mount("/", routes![device::index])
        .mount("/api/v1/", routes![
            device::get_device, 
            device::get_devices_list,
            device::run_device
        ])
    }
}

pub fn new() -> Box<dyn Interface + Send> {
    Box::new(Service{  })
}