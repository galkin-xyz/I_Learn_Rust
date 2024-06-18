use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    let len: usize = input.trim().parse().expect("Data error");

    let mut hyper_depth = Vec::<i16>::new();
    for _ in 0..len {
        input.clear();
        io::stdin().read_line(&mut input).expect("Read error");
        hyper_depth.push(input.trim().parse().expect("Data error"));
    }
    let mut max_potential: i32 = 0;
    for end in 0..len {
        for start in 0..=end {
            let mut potential: i32 = 0;
            for i in start..=end {
                potential += hyper_depth[i] as i32;
            }
            max_potential = max_potential.max(potential);
        }
    }

    println!("{max_potential}");
}
