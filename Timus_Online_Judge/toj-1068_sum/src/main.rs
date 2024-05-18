use std::cmp::Ordering;
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Read error");
    let n: i16 = n.trim().parse().expect("Data error");

    println!(
        "{}",
        match n.cmp(&0) {
            Ordering::Equal => 1,
            Ordering::Less => -n as i32 * (n as i32 - 1) / 2 + 1,
            Ordering::Greater => n as i32 * (n as i32 + 1) / 2,
        }
    );
}
