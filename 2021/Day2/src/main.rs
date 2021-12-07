use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt");
    println!("Hello, world!");
}
