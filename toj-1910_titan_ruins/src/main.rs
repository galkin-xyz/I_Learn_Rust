use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода.");
    let sect_count: u16 = input.trim().parse().expect("Ошибка данных.");

    let mut cur_sect_num: u16 = 0;
    let mut mid_sect_num: u16 = 0;
    let mut prev_sect_power: u32 = 0;
    let mut cur_sect_power: u32 = 0;
    let mut max_three_sect_power: u32 = 0;

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода.");
    for word in input.split_whitespace() {
        cur_sect_num += 1;
        let prev_prev_sect_power: u32 = prev_sect_power;
        prev_sect_power = cur_sect_power;
        cur_sect_power = word.trim().parse().expect("Ошибка данных.");
        let cur_three_sect_power: u32 = prev_prev_sect_power + prev_sect_power + cur_sect_power;
        if cur_three_sect_power > max_three_sect_power {
            mid_sect_num = cur_sect_num - 1;
            max_three_sect_power = cur_three_sect_power;
        }
    }

    println!("{max_three_sect_power} {mid_sect_num}");
}
