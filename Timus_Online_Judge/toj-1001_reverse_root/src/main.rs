use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).expect("Read error");
    let mut flow: Vec<u64> = Vec::new();

    for word in input.split_whitespace() {
        let num: u64 = match word.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        flow.push(num);
    }
    for num in flow.into_iter().rev() {
        println!("{:.4}", (num as f64).sqrt());
    }
}
