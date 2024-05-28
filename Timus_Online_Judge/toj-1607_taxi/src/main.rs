use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");

    let mut iter = input.split_whitespace();
    let mut petya_price: u16 = iter
        .next()
        .expect("Data error")
        .trim()
        .parse()
        .expect("Data error");
    let petya_discount: u16 = iter
        .next()
        .expect("Data error")
        .trim()
        .parse()
        .expect("Data error");
    let mut taxi_price: u16 = iter
        .next()
        .expect("Data error")
        .trim()
        .parse()
        .expect("Data error");
    let taxi_discount: u16 = iter
        .next()
        .expect("Data error")
        .trim()
        .parse()
        .expect("Data error");
    while petya_price < taxi_price {
        petya_price = taxi_price.min(petya_price + petya_discount);
        taxi_price = petya_price.max(taxi_price - taxi_discount);
    }
    println!("{}", petya_price.max(taxi_price));
}
