mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;
use rocket::{get, http::Status, serde::json::Json};

#[get("/")]
fn hello() -> Result<Json<String>, Status> {
    Ok(Json(String::from("Hello from gotapi!")))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}