use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");

    let mut iter = input
        .split_whitespace()
        .take(2)
        .map(|x| x.trim().parse::<u32>().expect("Data error"));
    let array_size: u32 = iter.next().expect("Data error");
    let max_value: u32 = iter.next().expect("Data error");

    println!("{}", array_size * (max_value + 1));
}
