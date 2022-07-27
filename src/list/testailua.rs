use std::time::SystemTime;

use super::Person;
use super::Post;

static mut STATE: Vec<String> = Vec::new();

static mut MESSAGES: Vec<Post> = Vec::new();

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

pub fn put_post(message: String, title: String) -> Vec<Post> {
    let now = SystemTime::now();

    let post = Post {
        message: message.to_string(),
        title: title.to_string(),
        time: now,
    };

    unsafe {
        MESSAGES.push(post);
        return MESSAGES.clone();
    }
}
