//use std::io;
//const MAX_N: usize = 100;
const MAX_N: usize = 4;

fn main() {
//    let mut n = String::new();
//    io::stdin().read_line(&mut n).expect("Ошибка ввода.");
//    let n: u8 = n.trim().parse().expect("Ошибка данных.");
    let n = MAX_N;

    let image: [[u8; MAX_N]; MAX_N] = [
        [1, 3, 6, 10],
        [2, 5, 9, 13],
        [4, 8, 12, 15],
        [7, 11, 14, 16]
    ];
    for max_index in 0..n + 1 {
        for i in (0..max_index).rev() {
//            print!("[{},{}] ", i, max_index - 1 - i);
            print!("a[{}][{}]={} ", i, max_index - 1 - i, image[i][max_index - 1 - i]);
        }
        println!("");
    }
    for min_index in (0..n) {
        for i in (min_index..n).rev() {
            print!("[{},{}] ", i, i);
//            print!("a[{}][{}]={} ", i, min_index - i + 1, image[i][min_index - i + 1]);
        }
        println!("");
    }
}
