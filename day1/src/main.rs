use std::fs;

fn main() {
    let data = fs::read_to_string("./src/data.txt").unwrap();

    let data: Vec<&str> = data.split_terminator("\n\n").collect();
    let data: Vec<Vec<i32>> = data
        .iter()
        .map(|f| f.split_whitespace().map(|s| s.parse().unwrap()).collect())
        .collect();
    let mut data: Vec<i32> = data
        .into_iter()
        .map(|x| x.into_iter().reduce(|accum, value| accum + value).unwrap())
        .collect();

    data.sort();

    let data: i32 = data.iter().rev().take(3).sum();

    println!("{:?}", data);
}
