#[macro_use] 
extern crate rocket;
extern crate serde_json;

use rocket::serde::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use serde_json::json;
use serde_json::Value;


#[get("/reverse/<name>")]
fn reverse_name(name: &str) -> Json<Value> {
    let reversed: String = name.chars().rev().collect();
    Json(json!({"reversed_name": reversed}))
}

#[launch]
fn rocket() -> _ {

    //Handling Cross Origin issue
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000"]); // Replace with your frontend server address

    let cors = CorsOptions {
        allowed_origins,
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("CORS configuration failed");


    rocket::build().attach(cors).mount("/", routes![reverse_name])
}