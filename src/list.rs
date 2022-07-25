#[get("/")]
pub fn print() -> &'static str {
    "Hello world from list"
}

#[get("/<new_item>")]
pub fn put_item(new_item: String) -> String {
    let item = String::from(new_item);
    let mut arr = Vec::new();
    arr.push(&item);
    arr.push(&item);

    let v1_iter = arr.iter();

    let mut output: String = String::new();

    for val in v1_iter {
        output = format!("{output} {val}")
    }

    return output;
}
