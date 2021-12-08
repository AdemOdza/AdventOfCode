use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt");
    let mut v: Vec<i32> = Vec::new();

    for item in contents.unwrap().trim().split("\n") {
        let temp: i32 = item.parse::<i32>().unwrap();
        v.push(temp);
    }

    let mut prev = v[0];
    let mut count = 0;
    for i in v{
        println!("{}", i);
        if i > prev{
            count = count + 1;
        } 
        prev = i;      
    }

    println!("Count is {}", count);
    
}
