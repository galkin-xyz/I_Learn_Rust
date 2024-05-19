use std::io;

fn main() {
    let mut records_count = String::new();
    io::stdin()
        .read_line(&mut records_count)
        .expect("Read error");
    let records_count: u16 = records_count.trim().parse().expect("Data error");

    let mut emperor_penguin_count: u16 = 0;
    let mut little_penguin_count: u16 = 0;
    let mut macaroni_penguin_count: u16 = 0;
    for _ in 0..records_count {
        let mut penguin = String::new();
        io::stdin().read_line(&mut penguin).expect("Read error");
        match penguin.trim() {
            "Emperor Penguin" => emperor_penguin_count += 1,
            "Little Penguin" => little_penguin_count += 1,
            "Macaroni Penguin" => macaroni_penguin_count += 1,
            _ => panic!("Unknown penguin type"),
        };
    }
    if emperor_penguin_count > little_penguin_count
        && emperor_penguin_count > macaroni_penguin_count
    {
        println!("Emperor Penguin");
    } else if little_penguin_count > emperor_penguin_count
        && little_penguin_count > macaroni_penguin_count
    {
        println!("Little Penguin");
    } else {
        println!("Macaroni Penguin");
    }
}
