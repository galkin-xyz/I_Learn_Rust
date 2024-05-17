use std::io;

fn main() {
    const STEAK_SIDES: u16 = 2;
    const FRYING_TIME: u16 = 1;
    const MIN_DURATION: u16 = STEAK_SIDES * FRYING_TIME;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода.");
    let mut iter = input.split_whitespace();
    let steaks_count = iter.next().expect("Ошибка данных.");
    let steaks_count: u16 = steaks_count.trim().parse().expect("Ошибка данных."); 
    let pan_capacity = iter.next().expect("Ошибка данных.");
    let pan_capacity: u16 = pan_capacity.trim().parse().expect("Ошибка данных."); 

    if steaks_count <= pan_capacity {
        println!("{MIN_DURATION}");
        return;
    }

    let work_vol = steaks_count * STEAK_SIDES * FRYING_TIME;
    let res = if work_vol % pan_capacity == 0 {
        work_vol / pan_capacity
    } else {
        work_vol / pan_capacity + 1
    };
    println!("{res}");
}
