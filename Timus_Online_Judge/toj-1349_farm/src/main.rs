use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Read error");
    let n: u8 = n.trim().parse().expect("Data error");
    match n {
        1 => println!("1 2 3"),
        2 => println!("3 4 5"),
        _ => println!("-1"),
    }
}
