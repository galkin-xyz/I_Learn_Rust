use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    let wheelsets_count: u16 = input.trim().parse().expect("Data error");
    let mut wheelsets = HashMap::<u16, u16>::new();
    for _ in 0..wheelsets_count {
        input.clear();
        io::stdin().read_line(&mut input).expect("Read error");
        let diameter: u16 = input.trim().parse().expect("Data error");
        wheelsets
            .entry(diameter)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }
    let mut carriages: u16 = 0;
    for count in wheelsets.values() {
        carriages += count >> 2;
    }

    println!("{carriages}");
}
