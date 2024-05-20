use std::io;

const MAX_N: usize = 100;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Read error");
    let n: usize = n.trim().parse().expect("Data error");

    let mut signboard: [[u16; MAX_N]; MAX_N] = [[0; MAX_N]; MAX_N];
    let mut number: u16 = 1;

    for shift in 0..=n {
        for i in 0..shift {
            let j = n - shift + i;
            signboard[i][j] = number;
            number += 1;
        }
    }
    for shift in (0..n).rev() {
        for j in 0..shift {
            let i = n - shift + j;
            signboard[i][j] = number;
            number += 1;
        }
    }

    for i in 0..n {
        for j in 0..n {
            print!("{} ", signboard[i][j]);
        }
        println!("");
    }
}
