// get info about device by id
#[get("/<id>")]
pub async fn get(id: u32) -> String {
    format!("User with ID #{}", id)
}
