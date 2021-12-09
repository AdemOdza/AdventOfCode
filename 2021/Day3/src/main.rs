use std::fs;
fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string("data.txt").unwrap();
    let split = contents.trim().split("\n");

    let mut v: Vec<(&str, i32)>  = Vec::new();

    for item in split {
        
        let temp: Vec<&str> = item.trim().split(' ').into_iter().collect();
        let curr_val: (&str, i32) = (temp[0], temp[1].parse::<i32>().unwrap());
        v.push(curr_val);
    }
}
