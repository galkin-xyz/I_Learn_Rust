use std::io;

fn main() {
    let mut items_count = String::new();
    io::stdin().read_line(&mut items_count).expect("Read error");
    let mods: [u8; 6] = [1, 3, 2, 6, 4, 5];
    let mut sum: u32 = 0;
    for (index, digit) in items_count.trim().chars().rev().enumerate() {
        let digit: u8 = String::from(digit).parse().expect("Data error");
        sum += (digit * mods[index % 6]) as u32;
    }
    print!("{}", sum % 7);
}
