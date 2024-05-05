use std::io;

fn main() {
    let mut positions: [(u8, u8); 64] = [(0, 0); 64];
    let mut tests_count = String::new();
    io::stdin().read_line(&mut tests_count).expect("Ошибка ввода.");
    let tests_count: usize = tests_count.trim().parse().expect("Ошибка данных.");

    for index in 0..tests_count {
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
            _ => return
        };
        let hor_pos: u8 = input[1..2].parse().expect("Ошибка данных");
        if !(1..=8).contains(&hor_pos) {
            return
        }
        positions[index] = (vert_pos, hor_pos);
    }

    for index in 0..tests_count {
        let (vert_pos, hor_pos) = positions[index];
        let mov_count: u8 = if hor_pos == 8 || hor_pos == 1 {
            get_vert_mov_count(vert_pos, 1) + get_vert_mov_count(vert_pos, 2) 
        } else if hor_pos == 7 || hor_pos == 2 {
            get_vert_mov_count(vert_pos, 1) + 2 * get_vert_mov_count(vert_pos, 2)
        } else {
            2 * get_vert_mov_count(vert_pos, 1) + 2 * get_vert_mov_count(vert_pos, 2)
        };
        println!("{mov_count}");
    };
}

fn get_vert_mov_count(vert_pos: u8, to_be_moved: u8) -> u8 {
    if vert_pos == 8 || vert_pos == 1 {
        1
    } else if (vert_pos == 7 || vert_pos == 2) && to_be_moved == 2 {
        1
    } else {
        2
    }
}
