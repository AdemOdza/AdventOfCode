use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let mut split = contents.trim().split("\n");

    let mut v: Vec<(&str, i32)>  = Vec::new();

    for item in split {
        let temp: Vec<&str> = item.trim().split(' ').into_iter().collect();
        
    }
}
