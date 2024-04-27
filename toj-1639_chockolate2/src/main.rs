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
    let m = params[0];
    let n = params[1];
    let breaks_count = (m - 1) + (n - 1) * m;
    let strategy = match breaks_count % 2 {
        1 => "[:=[first]",
        _ => "[second]=:]",
    };
    println!("{strategy}");
}
