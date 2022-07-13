fn main() {
    let test = first_function("rock n roll");
    println!("{}", test);
    // panic!("testing panic");
}

fn first_function(p: &str) -> &str {
    p
}
