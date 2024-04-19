use std::io;

fn main() {
    let mut weightings: [[u16; 2]; 4] = [[0; 2]; 4];
    const MATH_A: usize = 0;
    const MATH_B: usize = 1;

    for weighing_num in 1..=3 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка ввода.");
        let mut math: usize = MATH_A;
        for word in input.split_whitespace() {
            weightings[weighing_num][math] = word.trim().parse().expect("Ошибка данных.");
            math += 1;
        }
    }
    let math_a_berries_weight = weightings[1][MATH_A] - weightings[3][MATH_A];
    let math_b_berries_weight = weightings[1][MATH_B] - weightings[2][MATH_B];
    println!("{math_a_berries_weight} {math_b_berries_weight}");
}
