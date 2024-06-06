use std::io;

fn main() {
    let mut seat = String::new();
    io::stdin().read_line(&mut seat).expect("Read error");
    seat = seat.trim().to_string();
    let row: u8 = seat[..seat.len() - 1].parse().expect("r Data error");
    let letter: char = seat[seat.len() - 1..].chars().next().expect("l Data error");

    println!(
        "{}",
        match row {
            1..=2 => match letter {
                'A' | 'D' => "window",
                'B' | 'C' => "aisle",
                _ => "neither",
            },
            3..=20 => match letter {
                'A' | 'F' => "window",
                'B' | 'C' | 'D' | 'E' => "aisle",
                _ => "neither",
            },
            21..=65 => match letter {
                'A' | 'K' => "window",
                'C' | 'D' | 'G' | 'H' => "aisle",
                _ => "neither",
            },
            _ => "",
        }
    );
}
