use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let split = contents.trim().split("\n");

    let mut v: Vec<(&str, i32)>  = Vec::new();

    for item in split {
        
        let temp: Vec<&str> = item.trim().split(' ').into_iter().collect();
        let curr_val: (&str, i32) = (temp[0], temp[1].parse::<i32>().unwrap());
        v.push(curr_val);
    }

    let mut depth: i32 = 0;
    let mut pos: i32  = 0;
    let mut aim: i32  = 0;

    for command in v {
        println!("Command -> \"{} {}\"", command.0, command.1);
        match command.0{
            "forward" => {
                pos = pos + command.1;
                depth = depth + (aim * command.1); 
            }, 
            "backward" => pos = pos - command.1,
            "down" => aim = aim + command.1,
            "up" => aim = aim - command.1,
            _ => println!("Error: Invalid command"),
        }
    }

    println!("X={} and Depth={}. Product is {}", pos, depth, pos*depth);

}
