    
use rocket::response::Redirect;
use rocket::State;

use crate::dlms::service;
use crate::dlms::provider;

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!("/api/v1/devices"))
}

#[get("/device/<id>")]
pub fn get_device(_dlms_service: &State<Box<dyn service::Interface>>, id: u32)  -> String {
    format!("Device with ID #{}", id)
}

#[get("/devices")]
pub fn get_devices_list(_dlms_service: &State<Box<dyn service::Interface>>)  -> String {
    format!("Devices list: ")
}

#[post("/device")]
pub fn run_device(_dlms_service: &State<Box<dyn service::Interface>>)  -> String {
    _dlms_service.run(provider::new_instance("localhost:40400"));
    format!("New device running...")
}