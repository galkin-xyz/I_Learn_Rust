use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    let mut iter = input
        .split_whitespace()
        .map(|x| x.parse().expect("Data error"));
    let thickness_vol: i16 = iter.next().expect("Data error");
    let thickness_binding: i16 = iter.next().expect("Data error");
    let start_vol: i16 = iter.next().expect("Data error");
    let end_vol: i16 = iter.next().expect("Data error");
    println!(
        "{}",
        ((end_vol - start_vol) * (thickness_vol + 2 * thickness_binding) - thickness_vol).abs()
    );
}
