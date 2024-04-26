use std::io;

fn main() {
    let (pn1_count, pn1) = get_proper_numbers();
    println!("{pn1_count}");
}

fn get_proper_numbers() -> (usize, [u32; 4000]) {
    let mut pn: [u32; 4000] = [0; 4000];
    let mut pn_count = String::new();
    io::stdin().read_line(&mut pn_count).expect("Ошибка ввода.");
    let pn_count = pn_count.trim().parse().expect("ошибка данных.");
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers).expect("Ошибка ввода.");
    let mut index: usize = 0;
    for number in numbers.split_whitespace() {
        pn[index] = number.trim().parse().expect("Ошибка данных.");
        index += 1;
    }

    (pn_count, pn)
}
