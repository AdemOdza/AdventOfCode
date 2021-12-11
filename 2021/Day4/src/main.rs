use std::fs;
fn main() {
    let contents = fs::read_to_string("data.txt").unwrap();
    let mut split = contents.trim().split("\n\n");

    let numbers: Vec<i32> = split.next().unwrap().split(',').map(|s| s.parse().unwrap()).collect();
    println!("{:?}", numbers);
    let mut boards: Vec<Bingo> = Vec::new();

    for block in split {
            let mut board: Vec<Vec<i32>> = block
            .split('\n')
            .map(|x| x.split_whitespace().map(|y| y.parse::<i32>().unwrap()).collect())
            .collect();

            let mut tempBoard = Bingo {
                content: &board,
                found: Vec::new()
            };
            boards.push(tempBoard);
    }

    
}

struct Bingo {
    content: Vec<Vec<i32>>,
    found: Vec<Vec<bool>>,

}