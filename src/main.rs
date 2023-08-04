#[macro_use]
extern crate rocket;
use rocket::http::Status;
use rocket::Request;

mod database;
mod service;

use service::verify;

#[get("/")]
async fn index() -> &'static str {
    let result = verify().await;

    match result {
        true => "Verified",
        false => "Not Verified",
    }
}

#[catch(default)]
fn default_catcher(status: Status, _request: &Request) -> Status {
    status
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .register("/", catchers![default_catcher])
}
