use std::io;

fn main() {
    let mut chant = String::new();
    io::stdin().read_line(&mut chant).expect("Read error");

    let mut cost: u16 = 0;
    for ch in chant.chars() {
        cost += match ch {
            'a' | 'd' | 'g' | 'j' | 'm' | 'p' | 's' | 'v' | '.' | 'y' | ' ' => 1,
            'b' | 'e' | 'h' | 'k' | 'n' | 'q' | 't' | 'w' | ',' | 'z' => 2,
            'c' | 'f' | 'i' | 'l' | 'o' | 'r' | 'u' | 'x' | '!' => 3,
            _ => 0,
        };
    }
    println!("{}", cost);
}
