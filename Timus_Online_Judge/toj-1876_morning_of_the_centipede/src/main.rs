use std::io;

const FOOTS: u8 = 40;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    let mut iter = input.split_whitespace();
    let left_slipper: u8 = iter
        .next()
        .expect("Data error")
        .trim()
        .parse()
        .expect("Data error");
    let right_slipper: u8 = iter
        .next()
        .expect("Data error")
        .trim()
        .parse()
        .expect("Data error");
    let case1: u8 = right_slipper * 2 + FOOTS;
    let case2: u8 = left_slipper * 2 + FOOTS - 1;
    println!("{}", case1.max(case2));
}
