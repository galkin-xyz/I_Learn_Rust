use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода.");
    let sections_count: u16 = input.trim().parse().expect("Ошибка данных.");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода.");
    for word in input.split_whitespace() {
        let section_power: u32 = word.trim().parse().expect("Ошибка данных.");
    }

    println!("{input}");
}
