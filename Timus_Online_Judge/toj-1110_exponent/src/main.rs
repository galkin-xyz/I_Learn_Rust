use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read errror");
    let mut iter = input.split_whitespace();
    let exponent: usize = iter
        .next()
        .expect("Data error")
        .trim()
        .parse()
        .expect("Data error");
    let divider: usize = iter
        .next()
        .expect("Data error")
        .trim()
        .parse()
        .expect("Data error");
    let expected: u32 = iter
        .next()
        .expect("Data error")
        .trim()
        .parse()
        .expect("Data error");
    let mut is_res_found: bool = false;

    for x in 0..divider {
        let mut product: u32 = 1;
        for _ in 0..exponent {
            product = product * x as u32 % divider as u32;
        }
        if product == expected {
            print!("{} ", x);
            is_res_found = true;
        }
    }
    if !is_res_found {
        println!("-1");
    }
}
