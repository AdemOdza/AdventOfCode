use std::fs;

fn main() {
    let contents = fs::read_to_string("data.txt");
    let mut v: Vec<i32> = Vec::new();

    for item in contents.unwrap().trim().split("\n") {
        let temp: i32 = item.parse::<i32>().unwrap();
        v.push(temp);
    }

    let mut x = v[0] + v[1] + v[2];
    let mut count = 0;
    for i in 1..v.len()-2{
        let temp = v[i] + v[i+1] + v[i+2];
        if temp > x {
            count = count + 1;
        }

        x = temp;
    }

    println!("Count is {}", count);
}
