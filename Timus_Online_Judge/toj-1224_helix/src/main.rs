use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    let mut iter = input
        .split_whitespace()
        .map(|x| x.parse::<u32>().expect("Data error"));
    let n: u32 = iter.next().expect("Data error");
    let m: u32 = iter.next().expect("Data error");
    println!("{n} {m}");
}
