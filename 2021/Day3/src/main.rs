use std::fs;
fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let split = contents.trim().split("\n");

    let mut v: Vec<Vec<char>>  = Vec::new();
    let mut gamma: String = "".to_owned();
    let mut epsilon: String = "".to_owned();
    

    for item in split {
        let arr: Vec<char> =  item.trim().to_owned().chars().collect();
        v.push(arr);
    }

    for j in 0..v[0].len() {

        let mut i_count = 0;
        let mut o_count = 0;

        for i in 0..v.len() {
            if v[i][j] == '1'{
                i_count = i_count + 1;
            }
            else {
                o_count = o_count + 1;
            }
        }

        if i_count > o_count {
            gamma.push_str("1");
            epsilon.push_str("0");
        }
        else {
            gamma.push_str("0");
            epsilon.push_str("1");
        }

    }
    print!("Gamma: {} and Epsilon: {}", isize::from_str_radix(&gamma[..], 2).unwrap(), isize::from_str_radix(&epsilon[..], 2).unwrap());

    // let mostCommon: char = if i_count > o_count {'1'} else {'0'};

    // let mut co2 = String::new();
    // for i in 0..v.len(){
    //     if v[i][0] == mostCommon {

    //     }
    //     else {

    //     }

    // }
}
