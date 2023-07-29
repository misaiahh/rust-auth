#[macro_use] extern crate rocket;

#[path = "./db/db.rs"]
mod db;

#[get("/")]
fn index() -> &'static str {
    let result = db::verify();
    let response;

    match result {
        true => response = "Verified",
        _ => response = "Not Verified",
    };
    
    response
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}