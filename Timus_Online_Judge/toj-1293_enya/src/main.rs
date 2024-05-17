use std::io;

fn main() {
    const SIDE_COUNT: u32 = 2;
    const EXP: u32 = 1;
    let mut input = String::new();
    let mut params: [u8; 3] = [0, 0, 0];

    io::stdin().read_line(&mut input).expect("Ошибка ввода.");
    for (index, word) in input.split_whitespace().enumerate() {
        params[index] = word.parse().expect("Ошибка ввода.");
    }
    println!(
        "{}",
        params[0] as u32 * params[1] as u32 * params[2] as u32 * SIDE_COUNT * EXP
    );
}
