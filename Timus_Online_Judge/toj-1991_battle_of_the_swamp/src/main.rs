use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    let mut iter = input.split_whitespace();
    let groups_count: usize = iter
        .next()
        .expect("Data error")
        .trim()
        .parse()
        .expect("Data error");
    let droids_count: i32 = iter
        .next()
        .expect("Data error")
        .trim()
        .parse()
        .expect("Data error");
    input.clear();
    let mut bum_bums_allocation: Vec<i32> = Vec::new();
    io::stdin().read_line(&mut input).expect("Read error");
    for word in input.split_whitespace() {
        bum_bums_allocation.push(word.trim().parse().expect("Data error"));
    }
    let mut bum_bums_balance: u64 = 0;
    let mut droids_balance: u64 = 0;
    for i in 0..groups_count {
        bum_bums_balance += 0.max(bum_bums_allocation[i] - droids_count) as u64;
        droids_balance += 0.max(droids_count - bum_bums_allocation[i]) as u64;
    }
    println!("{bum_bums_balance} {droids_balance}");
}
