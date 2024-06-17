use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");

    let sum: i32 = input
        .split_whitespace()
        .take(2)
        .map(|x| x.parse::<i32>().expect("Data error"))
        .sum();
    println!("{sum}");
}
