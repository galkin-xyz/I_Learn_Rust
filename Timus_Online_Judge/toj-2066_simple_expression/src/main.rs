use std::cmp::Ordering;
use std::io;

fn main() {
    let mut min_val: i16;
    let mut max_val: i16;
    let mid_val: i16;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода.");
    min_val = input.trim().parse().expect("Ошибка данных.");

    input.clear();
    io::stdin().read_line(&mut input).expect("Ошибка ввода.");
    let val: i16 = input.trim().parse().expect("Ошибка данных.");
    match val.cmp(&min_val) {
        Ordering::Less => {
            max_val = min_val;
            min_val = val;
        }
        _ => max_val = val,
    };

    input.clear();
    io::stdin().read_line(&mut input).expect("Ошибка ввода.");
    let val: i16 = input.trim().parse().expect("Ошибка данных.");
    match val.cmp(&min_val) {
        Ordering::Less => {
            mid_val = min_val;
            min_val = val;
        }
        Ordering::Equal => mid_val = val,
        Ordering::Greater => {
            match val.cmp(&max_val) {
                Ordering::Greater => {
                    mid_val = max_val;
                    max_val = val;
                }
                _ => mid_val = val,
            };
        }
    };

    let res: i16 = if mid_val <= 1 {
        min_val - mid_val - max_val
    } else {
        min_val - mid_val * max_val
    };
    println!("{res}");
}
