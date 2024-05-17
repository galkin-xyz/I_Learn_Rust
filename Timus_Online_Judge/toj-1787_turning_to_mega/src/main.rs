use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода.");
    let mut iter = input.split_whitespace();
    let traffic_light_capacity = iter.next().expect("Ошибка данных.");
    let traffic_light_capacity: u8 = traffic_light_capacity.trim().parse().expect("Ошибка данных.");

    let mut cars_left: u16 = 0;
    input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода.");

    for word in input.split_whitespace() {
        let cars_in: u16 = word.trim().parse().expect("Ошибка данных.");
        cars_left = if cars_left + cars_in > traffic_light_capacity as u16 {
            cars_left + cars_in - traffic_light_capacity as u16
        } else {
            0
        }
    }
    println!("{cars_left}");
}
