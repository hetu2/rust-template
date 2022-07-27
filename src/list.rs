use std::time::SystemTime;

use rocket::serde::{json::Json, Serialize};

mod testailua;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Person {
    name: String,
    age: u8,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[derive(Clone)]
pub struct Post {
    message: String,
    title: String,
    time: SystemTime,
}

#[get("/")]
pub fn print() -> Json<Person> {
    let output: Person = testailua::get();

    return Json(output);
}

#[get("/string/<new_item>")]
pub fn put_item(new_item: String) -> Json<Vec<String>> {
    let output = testailua::put_item(new_item);

    return Json(output);
}

#[get("/<title>/<message>")]
pub fn put_post(title: String, message: String) -> Json<Vec<Post>> {
    let output = testailua::put_post(title, message);

    return Json(output);
}
