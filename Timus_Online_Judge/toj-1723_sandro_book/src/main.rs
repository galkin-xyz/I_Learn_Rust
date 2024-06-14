use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    let mut letters = HashMap::<char, u8>::new();
    for ch in input.trim().chars() {
        letters
            .entry(ch)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    let mut max_freq: u8 = 0;
    let mut max_letter: Option<char> = None;
    for (letter, freq) in letters {
        if freq > max_freq {
            max_freq = freq;
            max_letter = Some(letter);
        }
    }

    println!("{}", max_letter.unwrap());
}
