use std::io;
const MAX_N: usize = 100;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Ошибка ввода.");
    let n: usize = n.trim().parse().expect("Ошибка данных.");

    let mut image: [[u8; MAX_N]; MAX_N] = [[0; MAX_N]; MAX_N];

    for i in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка ввода.");
        for (j, word) in input.split_whitespace().enumerate() {
            image[i][j] = word.trim().parse().expect("Ошибка данных.");
        }
    }

    for max_index in 0..n + 1 {
        for i in (0..max_index).rev() {
            print!("{} ", image[i][max_index - 1 - i]);
        }
    }
    for min_index in 1..n {
        for i in (min_index..n).rev() {
            print!("{} ", image[i][n + min_index - i - 1]);
        }
    }
    println!("");
}
