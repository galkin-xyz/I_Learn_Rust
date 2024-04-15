use std::io;

fn main() {
    let mut input = String::new();
    let mut params: [u8; 2] = [0, 0];
    let mut index: usize = 0;

    io::stdin().read_line(&mut input).expect("Ошибка ввода.");
    for word in input.split_whitespace() {
        params[index] = word.parse().expect("Ошибка ввода.");
        index += 1;
    }
    println!("{} {}", params[1] - 1, params[0] - 1);
}
