use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода.");
    let mut iter = input.split_whitespace();
    let m = iter.next().expect("Ошибка данных.");
    let m: u8 = m.trim().parse().expect("Ошибка данных.");
    let n = iter.next().expect("Ошибка данных.");
    let n: u8 = n.trim().parse().expect("Ошибка данных.");

    let breaks_count: u16 = (m as u16 - 1) + (n as u16 - 1) * m as u16;
    let strategy = match breaks_count % 2 {
        1 => "[:=[first]",
        _ => "[second]=:]",
    };
    println!("{strategy}");
}
