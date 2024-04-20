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
    let traffic_light_capacity: &u16 = &params[0];
//    let duration: &u16 = &params[1];

    let mut cars_left: u16 = 0;
    input = String::new();    
    io::stdin().read_line(&mut input).expect("Ошибка ввода.");
    
    for word in input.split_whitespace() {
        let cars_in: u16 = word.trim().parse().expect("Ошибка данных.");
        cars_left = if cars_left + cars_in > *traffic_light_capacity {
            cars_left + cars_in - traffic_light_capacity
        } else {
            0
        }
    }
    println!("{cars_left}");
}
