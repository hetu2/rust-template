#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, wtf!"
}

#[get("/")]
fn foo() -> &'static str {
    "bar!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/foo", routes![foo])
}
