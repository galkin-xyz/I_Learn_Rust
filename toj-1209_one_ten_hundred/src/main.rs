use std::io;

const ONES_COUNT: usize = 92682; // Определено экмпериментально

fn main() {
    let mut input = String::new();
    let mut answers_string = String::new();
    let mut one_positions: [u32; ONES_COUNT] = [0; ONES_COUNT];

    fill(&mut one_positions);

    io::stdin().read_line(&mut input).expect("Ошибка ввода.");
    let positions_count: u16 = input.trim().parse().expect("Ошибка данных.");
    for _ in 0..positions_count {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка ввода.");
        let position: u32 = input.trim().parse().expect("Ошибка данных.");
        if is_one(position, &one_positions) {
            answers_string.push('1');
        } else {
            answers_string.push('0');
        }
        answers_string.push(' ');
    }
    println!("{}", answers_string.trim());
}

fn fill(one_positions: &mut [u32; ONES_COUNT]) -> u32 {
    let mut one_position: u32 = 1;
    let mut increment: u32 = 0;
    let mut index: usize = 0;

    while u32::MAX - one_position > increment {
        one_positions[index] = one_position;
        one_position += increment;
        increment +=1;
        index += 1;
    }
    increment 
}

fn is_one(position: u32, one_positions: &[u32; ONES_COUNT]) -> bool {
    let mut index: usize = 0;

    while one_positions[index] < position {
        index += 1;
    }
    position == one_positions[index] 
}
