use std::collections::HashMap;
use std::io;

fn main() {
    let mut hier_count = String::new();
    io::stdin().read_line(&mut hier_count).expect("Read error");
    let hier_count: u16 = hier_count.trim().parse().expect("Data error");
    let mut hieroglyphs: HashMap<char, Vec<char>> = HashMap::new();

    for _ in 0..hier_count {
        let mut hier = String::new();
        io::stdin().read_line(&mut hier).expect("Read error");
        let mut iter = hier.chars();
        let key: char = iter.next().expect("Data error");
        let val: char = iter.next().expect("Data error");
        if !hieroglyphs.contains_key(&key) {
            hieroglyphs.insert(key, Vec::<char>::new());
        }
        let tail = hieroglyphs.get_mut(&key).expect("Data error");
        tail.push(val);
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    let key: char = input.trim().parse().expect("Data error");
    if hieroglyphs.contains_key(&key) {
        for val in hieroglyphs[&key].iter() {
            println!("{}{}", key, val);
        }
    }
}
