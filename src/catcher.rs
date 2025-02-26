use rocket::serde::json::{Value, json};

// function to handle 401 unauthorized
#[catch(401)]
pub fn unauthorized() -> Value {
    json!( { "message" : "Invalid/Missing authorization!"} )
}

// function to handle 404 not found
#[catch(404)]
pub fn not_found() -> Value {
    json!("Not Found!")
}

// function to handle 422 Unprocesable entity
#[catch(422)]
pub fn unprocessable_entity() -> Value {
    json!( { "message" : "Unprocessable Entity"} )
}