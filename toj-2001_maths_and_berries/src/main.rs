use std::io;

fn main() {
    let mut params: [[u16; 2]; 3] = [[0; 2]; 3];

    for weighing_num in 0..3 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка ввода.");
        let mut math_num: usize = 0;
        for word in input.split_whitespace() {
            params[weighing_num][math_num] = word.trim().parse().expect("Ошибка данных.");
            math_num += 1;
        }
    }
}
