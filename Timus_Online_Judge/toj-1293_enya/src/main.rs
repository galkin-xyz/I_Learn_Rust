use std::io;

fn main() {
    let mut input = String::new();
    let mut params: [u32; 3] = [0, 0, 0];
    let mut index: usize = 0;

    io::stdin().read_line(&mut input).expect("Ошибка ввода.");
    for word in input.split_whitespace() {
        params[index] = word.parse().expect("Ошибка ввода.");
        index += 1;
    }
    let res: u32 = 2 * params[0] * params[1] * params[2];
    println!("{}", res);
}
