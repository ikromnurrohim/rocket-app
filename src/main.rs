#[macro_use] extern crate rocket;

mod auth;
mod schema;
mod models;
mod catcher;


use diesel::prelude::*;
use auth::BasicAuth;
use rocket::serde::json::{Value, json, Json};
use rocket::response::status;
use rocket_sync_db_pools::database;
use crate::schema::rustaceans;
use crate::models::{NewRustacean, Rustacean};
use crate::catcher::{not_found, unauthorized, unprocessable_entity};

/*
initialize database conn
value of key sqlite it will check to file Rocket.toml which has some value that is database url
 */
#[database("sqlite")]
struct DbConn(diesel::SqliteConnection);


#[get("/rustaceans")]
async fn get_rustaceans(_auth: BasicAuth, db: DbConn) -> Value {
    /*
    variable db is a pool, is not database connection.
    to retrieve connection from the pool we need call method run
    and run method is accept the callback which is sqlite connection in this case.
    after that we can use that connection to query to database
    */
    db.run(|c| {
        let rustaceans = rustaceans::table.order(rustaceans::id.desc()).limit(100).load::<Rustacean>(c).expect("Error loading rustaceans");
        json!(rustaceans)
    }).await
}

#[get("/rustaceans/<id>")]
fn view_rustaceans(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe"})
}


/*
    why new_rustacean using Json<NewRustacean> ?
    cause we have been declacre here the format is json, so in here we catch the data through struct Json<>.
    cause the NewRustacean struct is from the json format, that mean to converted to our data structure
    we need to derive the Deserialize in our struct NewRustacean, to be enable to convert that json data to our data structure
 */
#[post("/rustaceans", format="json", data="<new_rustacean>")]
async fn create_rustaceans(_auth: BasicAuth, db: DbConn, new_rustacean: Json<NewRustacean>) -> Value {
    db.run(|c|{
        let result = diesel::insert_into(rustaceans::table)
            // using into_inner() here cause the data passed from client is json, so we need to unwrap that json to be our data structure, it can be done using this method
            .values(new_rustacean.into_inner())
            .execute(c)
            .expect("Error saving new rustacean");
        json!({"id": result})
    }).await
}

#[put("/rustaceans/<id>", format="json")]
fn update_rustaceans(id: i32, _auth: BasicAuth) -> Value {
    json!({"id": id, "name": "John Doe"})
}

#[delete("/rustaceans/<_id>")]
fn delete_rustaceans(_id: i32, _auth: BasicAuth) -> status::NoContent {
    status::NoContent
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
        unauthorized,
        not_found,
        unprocessable_entity,
    ])
    // attach database connection before launch the rocket server
    .attach(DbConn::fairing())
    .launch()
    .await;
}

