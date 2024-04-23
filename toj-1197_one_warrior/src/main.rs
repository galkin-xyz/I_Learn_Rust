use std::io;

fn main() {
    let mut tests_count = String::new();
    io::stdin().read_line(&mut tests_count).expect("Ошибка ввода.");
    let tests_count: usize = tests_count.trim().parse().expect("Ошибка данных.");

    for _ in 0..tests_count {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Ошибка ввода.");
        let vert_pos: u8 = match input.chars().next() {
            Some('a') => 1,
            Some('b') => 2,
            Some('c') => 3,
            Some('d') => 4,
            Some('e') => 5,
            Some('f') => 6,
            Some('g') => 7,
            Some('h') => 8,
            _ => 0,
        };
        println!("{} - {}", vert_pos, &input[1..2]);
    }
}

fn get_vert_movements_count(cur_vert_pos: u8, to_be_moved: u8) -> u8 {
    if cur_vert_pos == 8 || cur_vert_pos == 1 {
        1
    } else if (cur_vert_pos == 7 || cur_vert_pos == 2) && to_be_moved == 2 {
        1
    } else {
        2
    }
}
