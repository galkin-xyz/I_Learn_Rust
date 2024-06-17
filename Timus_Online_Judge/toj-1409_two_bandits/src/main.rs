use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");

    let mut iter = input
        .split_whitespace()
        .map(|x| x.parse().expect("Data error"));
    let harry: u8 = iter.next().expect("Data error");
    let larry: u8 = iter.next().expect("Data error");
    println!("{} {}", larry - 1, harry - 1);
}
