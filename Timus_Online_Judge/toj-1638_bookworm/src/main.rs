use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    let mut iter = input
        .split_whitespace()
        .map(|x| x.parse().expect("Data error"));
    let thickness_vol: u8 = iter.next().expect("Data error");
    let thickness_binding: u8 = iter.next().expect("Data error");
    let start_vol: u8 = iter.next().expect("Data error");
    let end_vol: u8 = iter.next().expect("Data error");

    println!("{thickness_vol} {thickness_binding} {start_vol} {end_vol}");
}
