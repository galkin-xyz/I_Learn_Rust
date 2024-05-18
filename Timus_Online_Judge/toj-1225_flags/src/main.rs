use std::io;

fn main() {
    let mut strip_count = String::new();
    io::stdin().read_line(&mut strip_count).expect("Read error");
    let strip_count: u8 = strip_count.trim().parse().expect("Data error");

    println!(
        "{}",
        match strip_count {
            0 => 0, // не может быть по условию задачи
            1..=2 => 2,
            _ => {
                let mut prev_prev: u32 = 2;
                let mut prev: u32 = 2;
                let mut flags_count = prev_prev + prev;
                for _ in 4..=strip_count {
                    prev_prev = prev;
                    prev = flags_count;
                    flags_count = prev_prev + prev;
                }
                flags_count
            }
        }
    );
}
