use std::io;

fn main() {
    const SIDE_COUNT: u32 = 2;
    const EXP: u32 = 1;
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Ошибка ввода.");
    let amount: u32 = input
        .split_whitespace()
        .take(3)
        .map(|x| x.parse::<u32>().expect("Data error"))
        .product::<u32>()
        * SIDE_COUNT
        * EXP;
    println!("{amount}");
}
