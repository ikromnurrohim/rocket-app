#[macro_use] extern crate rocket;

mod auth;

use auth::BasicAuth;
use rocket::serde::json::{Value, json};
use rocket::response::status;

#[get("/rustaceans")]
fn get_rustaceans(_auth: BasicAuth) -> Value {
    json!([{ "id": 1, "name": "John Doe" }, { "id": 2, "name": "John Doe again" }])
}

#[get("/rustaceans/<id>")]
fn view_rustaceans(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe"})
}

#[post("/rustaceans/<id>", format="json")]
fn create_rustaceans(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe"})
}

#[put("/rustaceans/<id>", format="json")]
fn update_rustaceans(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe"})
}

#[delete("/rustaceans/<_id>")]
fn delete_rustaceans(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
}

// function to handle 404 not found
#[catch(404)]
fn not_found() -> Value {
    json!("Not Found!")
}

// function to handle 401 unauthorized
#[catch(401)]
fn unauthorized() -> Value {
    json!("Invalid/Missing authorization!")
}

#[rocket::main]
async fn main(){
    let _ = rocket::build()
    // rounting
    .mount("/", routes![
        get_rustaceans,
        view_rustaceans,
        create_rustaceans,
        update_rustaceans,
        delete_rustaceans
    ])
    // to handle error 
    .register("/", catchers![
        not_found,
        unauthorized
    ])
    .launch()
    .await;
}

