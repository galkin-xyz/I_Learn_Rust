use std::io;

fn main() {
    let mut input = String::new();
    let mut params: [u32; 2] = [0, 0];
    let mut index: usize = 0;

    io::stdin().read_line(&mut input).expect("Ошибка ввода.");
    for word in input.split_whitespace() {
        params[index] = word.trim().parse().expect("Ошибка данных.");
        index += 1;
    }
    let array_size: &u32 = &params[0];
    let max_value: &u32 = &params[1];

    let res = array_size * (max_value + 1);

    println!("{res}");
}
