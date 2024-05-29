use rocket::State;

use crate::dlms::provider;
use crate::http;

const TEST_ID: u32 = 1;

// get info about device by id
#[get("/<id>")]
pub fn get(_service_state: &State<Box<dyn http::state::Interface>>, id: u32) -> String {
    format!("Device with ID #{}", id)
}

// get list of all devices
#[get("/list")]
pub fn list(_service_state: &State<Box<dyn http::state::Interface>>) -> String {
    format!("Devices list: ")
}

// add new device
#[post("/create")]
pub fn create(_service_state: &State<Box<dyn http::state::Interface>>) -> String {
    // TODO: keep device info into storage
    format!("New device added")
}

// run device
#[post("/run")]
pub fn run(_service_state: &State<Box<dyn http::state::Interface>>) -> String {
    // TODO: get device info by id and run
    // get id from body or cookie
    _service_state
        .dlms()
        .run(TEST_ID, provider::new_instance("localhost:40400"));

    format!("New device running...")
}

// stop running device
#[post("/stop")]
pub fn stop(_service_state: &State<Box<dyn http::state::Interface>>) -> String {
    // TODO: get handler by id and stop
    // get id from body or cookie
    _service_state.dlms().stop(TEST_ID);

    format!("device stopping...")
}
