use std::io;

fn main() {
    let mut tests_count = String::new();
    io::stdin().read_line(&mut tests_count).expect("Ошибка ввода.");
    let tests_count: usize = tests_count.trim().parse().expect("Ошибка данных.");

    for _ in 0..tests_count {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка ввода.");
        println!("{input}");
    }
}
