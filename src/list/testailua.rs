use super::Person;

static mut STATE: Vec<String> = Vec::new();

pub fn get() -> Person {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    return peter;
}

pub fn put_item(new_item: String) -> Vec<String> {
    let item = String::from(new_item);
    unsafe {
        STATE.push(item);
        return STATE.clone();
    }
}
