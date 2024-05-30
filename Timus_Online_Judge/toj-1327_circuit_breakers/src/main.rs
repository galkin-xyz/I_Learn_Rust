use std::io;

fn main() {
    let mut start_day = String::new();
    io::stdin().read_line(&mut start_day).expect("Read error");
    let start_day: u16 = start_day.trim().parse().expect("Data error");
    let mut end_day = String::new();
    io::stdin().read_line(&mut end_day).expect("Read error");
    let end_day: u16 = end_day.trim().parse().expect("Data error");
    println!(
        "{}",
        (end_day + end_day % 2 - (start_day - start_day % 2)) / 2
    );
}
