use std::io;

fn main() {
    const ATTEMPT_PENALTY_TIME: u16 = 20;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    let mut iter = input.split_whitespace();
    let qxx_penalty_time: u16 = iter
        .next()
        .expect("Data error")
        .parse()
        .expect("Data error");
    let zzz_penalty_time: u16 = iter
        .next()
        .expect("Data error")
        .parse()
        .expect("Data error");
    input.clear();

    io::stdin().read_line(&mut input).expect("Read error");
    let attempts = input
        .split_whitespace()
        .map(|x| x.parse().expect("Data error"))
        .collect::<Vec<u16>>();

    let mut attempts_total: u16 = 0;
    for count in attempts {
        attempts_total += count;
    }

    println!(
        "{}",
        if zzz_penalty_time - attempts_total * ATTEMPT_PENALTY_TIME < qxx_penalty_time {
            "Dirty debug :("
        } else {
            "No chance."
        }
    );
}
