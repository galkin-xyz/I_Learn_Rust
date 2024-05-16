//use std::io;
//const MAX_N: usize = 100;
const MAX_N: usize = 4;

fn main() {
    //    let mut n = String::new();
    //    io::stdin().read_line(&mut n).expect("Ошибка ввода.");
    //    let n: u8 = n.trim().parse().expect("Ошибка данных.");
    let mut n = MAX_N;

    let image0: [[u8; MAX_N]; MAX_N] = [
        [1, 3, 6, 10],
        [2, 5, 9, 13],
        [4, 8, 12, 15],
        [7, 11, 14, 16],
    ];
    let image1: [[u8; 5]; 5] = [
        [1, 3, 6, 10, 15],
        [2, 5, 9, 14, 19],
        [4, 8, 13, 18, 22],
        [7, 12, 17, 21, 24],
        [11, 16, 20, 23, 25],
    ];

    for max_index in 0..n + 1 {
        for i in (0..max_index).rev() {
            //            print!("[{},{}] ", i, max_index - 1 - i);
            print!(
                "a[{}][{}]={} ",
                i,
                max_index - 1 - i,
                image0[i][max_index - 1 - i]
            );
        }
        println!("");
    }
    for min_index in 1..n {
        for i in (min_index..n).rev() {
            //            print!("min_index={}, i={}, j={}| ", min_index, i, n - i + min_index - 1);
            print!(
                "a[{}][{}]={} ",
                i,
                n + min_index - i - 1,
                image0[i][n + min_index - i - 1]
            );
        }
        println!("");
    }
    // нечётное число
    n = 5;
    for max_index in 0..n + 1 {
        for i in (0..max_index).rev() {
            //            print!("[{},{}] ", i, max_index - 1 - i);
            print!(
                "a[{}][{}]={} ",
                i,
                max_index - 1 - i,
                image1[i][max_index - 1 - i]
            );
        }
        println!("");
    }
    for min_index in 1..n {
        for i in (min_index..n).rev() {
            //            print!("min_index={}, i={}, j={}| ", min_index, i, n - i + min_index - 1);
            print!(
                "a[{}][{}]={} ",
                i,
                n + min_index - i - 1,
                image1[i][n + min_index - i - 1]
            );
        }
        println!("");
    }
}
