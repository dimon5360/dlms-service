use rocket::State;

use crate::dlms::provider;
use crate::http;

const TEST_ID: u32 = 1;

// get info about device by id
#[get("/device/<id>")]
pub fn get(_state: &State<Box<dyn http::state::Interface>>, id: u32) -> String {
    format!("Device with ID #{}", id)
}

// get list of all devices
#[get("/devices")]
pub fn list(_state: &State<Box<dyn http::state::Interface>>) -> String {
    format!("Devices list: ")
}

// add new device
#[post("/device/create")]
pub fn create(_state: &State<Box<dyn http::state::Interface>>) -> String {
    // TODO: keep device info into storage
    format!("New device added")
}

// run device
#[post("/device/run")]
pub fn run(_state: &State<Box<dyn http::state::Interface>>) -> String {
    // TODO: get device info by id and run
    _state
        .dlms()
        .run(TEST_ID, provider::new_instance("localhost:40400"));

    format!("New device running...")
}

// stop running device
#[post("/device/stop")]
pub fn stop(_state: &State<Box<dyn http::state::Interface>>) -> String {
    // TODO: get handler by id and stop
    _state.dlms().stop(TEST_ID);

    format!("device stopping...")
}
