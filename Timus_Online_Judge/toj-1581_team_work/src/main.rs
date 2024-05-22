use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Read error");
    let n: usize = n.trim().parse().expect("Data error");

    let mut str_of_num = String::new();
    io::stdin().read_line(&mut str_of_num).expect("Read error");
    let mut prev_num: u8 = 0;
    let mut num_count: u16 = 0;
    for (i, word) in str_of_num.split_whitespace().enumerate() {
        if i == n {
            break;
        }
        let cur_num: u8 = word.trim().parse().expect("Data error");
        if cur_num == prev_num {
            num_count += 1;
        } else {
            if prev_num != 0 {
                print!("{num_count} {prev_num} ");
            }
            prev_num = cur_num;
            num_count = 1;
        }
    }
    print!("{num_count} {prev_num} ");
}
