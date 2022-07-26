use rocket::serde::{json::Json, Serialize};

mod testailua;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Person {
    name: String,
    age: u8,
}

#[get("/")]
pub fn print() -> Json<Person> {
    let output: Person = testailua::get();

    return Json(output);
}

#[get("/<new_item>")]
pub fn put_item(new_item: String) -> Json<Vec<String>> {
    let output = testailua::put_item(new_item);

    return Json(output);
}
