use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода.");

    let mut params: [u8; 2] = [0, 0];
    for (index, word) in input.split_whitespace().enumerate() {
        params[index] = word.parse().expect("Ошибка ввода.");
    }
    println!("{} {}", params[1] - 1, params[0] - 1);
}
