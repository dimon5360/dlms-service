
use rocket::Build;
use rocket::response::Redirect;

use crate::http::endpoint::device;

pub trait Interface: Send {
    fn init(&self) -> rocket::Rocket<Build>;
}

struct Service;

impl Interface for Service {
    fn init(&self) -> rocket::Rocket<Build> {
        rocket::build()
        .mount("/", routes![index])
        .mount("/api/v1/", routes![
            device::get,
            device::list,
            device::create,
            device::run,
            device::stop
        ])
    }
}

pub fn new() -> Box<dyn Interface> {
    Box::new(Service{  })
}

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!("/api/v1/devices"))
}