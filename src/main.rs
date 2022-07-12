fn main() {
    let test = first_function("rock n roll");
    println!("{}", test);
}

fn first_function(p: &str) -> &str {
    return p;
}
