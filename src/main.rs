#[macro_use]
extern crate rocket;

#[path = "./db/db.rs"]
mod db;

#[get("/")]
fn index() -> &'static str {
    let result = db::verify();

    match result {
        true => "Verified",
        false => "Not Verified",
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
