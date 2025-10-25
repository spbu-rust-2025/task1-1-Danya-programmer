use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<f64> = input
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();

    println!("{}", nums[0] + nums[1]);
}
