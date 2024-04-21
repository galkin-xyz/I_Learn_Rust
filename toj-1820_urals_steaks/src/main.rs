use std::io;

fn main() {
    let mut input = String::new();
    let mut params: [u16; 2] = [0; 2];
    let mut index: usize = 0;

    io::stdin().read_line(&mut input).expect("Ошибка ввода.");
    for word in input.split_whitespace() {
        params[index] = word.trim().parse().expect("Ошибка данных.");
        index += 1;
    }
//    println!("{}");
}
