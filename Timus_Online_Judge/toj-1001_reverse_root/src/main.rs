use std::io;

fn main() {
    let stdin = io::read_to_string(io::stdin()).expect("Read error");
    let mut input: Vec<u64> = Vec::new();

    for word in stdin.split_whitespace() {
        let num: u64 = match word.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        input.push(num);
    }
    for num in input.into_iter().rev() {
        println!("{:.4}", (num as f64).sqrt());
    }
}
