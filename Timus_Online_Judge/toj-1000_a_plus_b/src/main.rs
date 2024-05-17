use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода.");

    let mut sum: i32 = 0;
    for word in input.split_whitespace() {
        sum += word.parse::<i32>().expect("Ошибка данных.");
    }

    println!("{sum}");
}
