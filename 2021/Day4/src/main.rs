use std::fs;
fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let split = contents.trim().split("\n\n");

    let mut seq: Vec<char> = split[0].trim().to_owned().chars().collect();


    // for i in 1..split.len() {

    // }

}
