use std::fs;
fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let mut split = contents.trim().split("\n\n");

    let numbers: Vec<i32> = split.next().unwrap().split(',').map(|s| s.parse().unwrap()).collect();
    println!("{:?}", numbers);

    for block in split {

    }
}

fn check_bingo() {
    
}
