use std::io;

const MAX_TEAMS_COUNT: usize = 150000 + 1;

fn main() {
    let mut teams_count = String::new();
    io::stdin().read_line(&mut teams_count).expect("Read error");
    let teams_count: usize = teams_count.trim().parse().expect("Data error");
    let mut results_table: [(u32, u8); MAX_TEAMS_COUNT] = [(0, 0); MAX_TEAMS_COUNT];

    for i in 0..teams_count {
        let mut team_info = String::new();
        io::stdin().read_line(&mut team_info).expect("Read error");
        let mut iter = team_info.split_whitespace();
        let id: u32 = iter
            .next()
            .expect("Data error")
            .trim()
            .parse()
            .expect("Data error");
        let score: u8 = iter
            .next()
            .expect("Data error")
            .trim()
            .parse()
            .expect("Data error");
        results_table[i] = (id, score);
    }
    // сортировка методом пузырька. Решение не проходит "Time limit exceeded"
    //    for j in 1..teams_count {
    //        for i in 0..teams_count - j {
    //            if results_table[i].1 < results_table[i + 1].1 {
    //                let tmp = results_table[i + 1];
    //                results_table[i + 1] = results_table[i];
    //                results_table[i] = tmp;
    //            }
    //        }
    //    }
    // сортировка методом вставки. Решение не проходит "Time limit exceeded"
    //    for j in 1..teams_count {
    //        let mut i = j;
    //        while i > 0 && results_table[i].1 > results_table[i - 1].1 {
    //                let tmp = results_table[i - 1];
    //                results_table[i - 1] = results_table[i];
    //                results_table[i] = tmp;
    //                i -= 1;
    //        }
    //    }

    for i in 0..teams_count {
        println!("{} {}", results_table[i].0, results_table[i].1);
    }
}
