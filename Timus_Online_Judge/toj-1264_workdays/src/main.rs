use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода.");

    let mut iter = input.split_whitespace();
    let array_size = iter.next().expect("Ошибка данных.");
    let array_size: u16 = array_size.trim().parse().expect("Ошибка данных.");
    let max_value = iter.next().expect("Ошибка данных.");
    let max_value: u16 = max_value.trim().parse().expect("Ошибка данных.");

    println!("{}", array_size as u32 * (max_value as u32 + 1));
}
