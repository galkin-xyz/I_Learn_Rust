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
    const STEAK_SIDES: u16 = 2;
    const FRYING_TIME: u16 = 1;
    const MIN_DURATION: u16 = STEAK_SIDES * FRYING_TIME;

    let steaks_count = params[0];
    let pan_capacity = params[1];

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
