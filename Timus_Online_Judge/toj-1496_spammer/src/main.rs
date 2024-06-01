use std::io;
use std::collections::HashMap;

fn main() {
    let mut submits_count = String::new();
    io::stdin().read_line(&mut submits_count).expect("Read error");
    let submits_count: usize = submits_count.trim().parse().expect("Data error");

    let mut commands = HashMap::new();
    for _ in 0..submits_count {
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Read error");
        name = name.trim().to_string();
        let count: u8 = match commands.get(&name) {
            Some(cur_count) => cur_count + 1,
            None => 1,
        };
        commands.insert(name, count);
    }
    for (name, count) in commands.iter() {
        if *count > 1u8 {
            println!("{}", name);
        }
    }
}
