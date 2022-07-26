use super::Person;

pub fn get() -> Person {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    return peter;
}

pub fn put_item(new_item: String) -> Vec<String> {
    let item = String::from(new_item);
    let mut arr = Vec::new();

    let item2 = item.clone();
    arr.push(item);
    arr.push(item2);
    return arr;
}
