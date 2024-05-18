use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Read error");
    let n: u8 = n.trim().parse().expect("Data error");

    println!(
        "{}",
        match n % 4 {
            1 | 2 => "grimy",
            0 | 3 => "black",
            _ => "",
        }
    );
}
