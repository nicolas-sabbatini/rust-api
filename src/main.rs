#![allow(dead_code, unused_variables, unused_imports)]
use auth_guard::BasicAuth;
use rocket::{
    response::status,
    serde::{
        json::{json, Value},
        uuid::Uuid,
        Serialize,
    },
};

mod auth_guard;

#[macro_use]
extern crate rocket;

// #[derive(Debug, Serialize)]
// struct User<'a> {
//     name: &'a str,
// }

#[get("/user")]
fn get_user() -> Value {
    todo!()
}

#[get("/user/<id>")]
fn get_user_by_id(_auth: BasicAuth, id: Uuid) -> Value {
    todo!()
}

#[post("/user", format = "json")]
fn post_user(_auth: BasicAuth) -> Value {
    todo!()
}

#[put("/user/<id>", format = "json")]
fn update_user(_auth: BasicAuth, id: Uuid) -> Value {
    todo!()
}

#[delete("/user/<id>")]
fn delete_user(_auth: BasicAuth, id: Uuid) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not Found! 😢")
}

#[catch(401)]
fn unauthorized() -> Value {
    json!("Unauthorized! 😢")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                get_user,
                get_user_by_id,
                post_user,
                update_user,
                delete_user,
            ],
        )
        .register("/", catchers![not_found, unauthorized])
}
