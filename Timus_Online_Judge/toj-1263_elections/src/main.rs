use std::io;
const MAX_CANDIDATES: usize = 10000 + 1;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    let mut iter = input.split_whitespace();
    let candidates_count = iter.next().expect("Data error");
    let candidates_count: usize = candidates_count.trim().parse().expect("Data error");
    let voters_count = iter.next().expect("Data error");
    let voters_count: u16 = voters_count.trim().parse().expect("Data error");
    let mut votes_for: [u16; MAX_CANDIDATES] = [0; MAX_CANDIDATES];

    for _ in 0..voters_count {
        let mut candidate = String::new();
        io::stdin().read_line(&mut candidate).expect("Read error");
        let candidate: usize = candidate.trim().parse().expect("Data error");
        votes_for[candidate] += 1;
    }
    for candidate in 1..=candidates_count {
        println!(
            "{:.2}%",
            votes_for[candidate] as f64 / voters_count as f64 * 100.00
        );
    }
}
