mod hello;
mod list;
#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    return hello::print_hello();
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
        .mount("/list", routes![list::print, list::put_item])
}
