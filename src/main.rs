use rocket::{
    response::status,
    serde::{
        json::{json, Value},
        uuid::Uuid,
        Serialize,
    },
};

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
fn get_user_by_id(id: Uuid) -> Value {
    todo!()
}

#[post("/user", format = "json")]
fn post_user() -> Value {
    todo!()
}

#[put("/user/<id>", format = "json")]
fn update_user(id: Uuid) -> Value {
    todo!()
}

#[delete("/user/<id>")]
fn delete_user(id: Uuid) -> status::NoContent {
    status::NoContent
}

#[catch(404)]
fn not_found() -> Value {
    json!("Not Found! 😢")
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
        .register("/", catchers![not_found])
}
