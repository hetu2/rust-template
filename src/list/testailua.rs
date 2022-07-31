use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
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
        id: get_id(),
        message: message.to_string(),
        title: title.to_string(),
        time: now,
    };

    unsafe {
        MESSAGES.push(post);
        return MESSAGES.clone();
    }
}

fn get_id() -> String {
    let id = loop {
        let rand_string: String = randomizer();

        unsafe {
            let is_used = MESSAGES.iter().find(|&r| r.id == rand_string.to_string());

            if is_used.is_none() {
                break rand_string;
            }
        }
    };

    return id;
}

fn randomizer() -> String {
    let string_length: usize = 6;
    return thread_rng()
        .sample_iter(&Alphanumeric)
        .take(string_length)
        .map(char::from)
        .collect();
}

#[cfg(test)]
mod testailua_tests {
    use super::*;

    #[test]
    fn randomizer_get() {
        let r = randomizer();
        assert_eq!(r.len(), 6);
    }
}
