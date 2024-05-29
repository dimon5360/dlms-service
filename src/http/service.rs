use rocket::response::Redirect;
use rocket::Build;

use rocket_dyn_templates::{context, Template};

use crate::http::endpoint::{device, user};

pub trait Interface: Send {
    fn init(&self) -> rocket::Rocket<Build>;
}

struct Service;

impl Interface for Service {
    fn init(&self) -> rocket::Rocket<Build> {
        rocket::build()
            .attach(Template::fairing())
            .mount("/", routes![root, hello, index])
            .mount(
                "/api/v1/device/",
                routes![
                    device::get,
                    device::list,
                    device::create,
                    device::run,
                    device::stop
                ],
            )
            .mount("/api/v1/user", routes![user::get])
    }
}

pub fn new() -> Box<dyn Interface> {
    Box::new(Service {})
}

#[get("/")]
pub fn root() -> Redirect {
    Redirect::to(uri!(index))
}

#[get("/index")]
pub fn index() -> Template {
    Template::render("index", context! { message: "Hello, Rust"})
}

#[get("/hello?<name>")]
async fn hello(name: String) -> Template {
    Template::render("hello", context! { name , message: "Hello, Rust" })
}
