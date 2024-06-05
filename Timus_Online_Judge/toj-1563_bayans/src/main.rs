use std::collections::HashSet;
use std::io;

fn main() {
    let mut shops_count = String::new();
    io::stdin().read_line(&mut shops_count).expect("Read error");
    let shops_count: usize = shops_count.trim().parse().expect("Data error");

    let mut shops_visited = HashSet::new();
    for _ in 0..shops_count {
        let mut shop = String::new();
        io::stdin().read_line(&mut shop).expect("Read error");
        shop = shop.trim().to_string();
        shops_visited.insert(shop);
    }

    println!("{}", shops_count - shops_visited.len());
}
