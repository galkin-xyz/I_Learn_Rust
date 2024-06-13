use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    let n: u8 = input.trim().parse().expect("Data error");

    println!("{n}");
}
