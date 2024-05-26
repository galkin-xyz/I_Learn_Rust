use std::io;

//const MAX_TEAMS_COUNT: usize = 150000 + 1;
const MAX_SCORE: usize = 100 + 1;
const ARRAY_REPEAT_VALUE: Vec<u32> = Vec::new();

fn main() {
    let mut teams_count = String::new();
    io::stdin().read_line(&mut teams_count).expect("Read error");
    let teams_count: usize = teams_count.trim().parse().expect("Data error");
    let mut results_table: [Vec<u32>; MAX_SCORE] = [ARRAY_REPEAT_VALUE; MAX_SCORE];

    for _ in 0..teams_count {
        let mut team_info = String::new();
        io::stdin().read_line(&mut team_info).expect("Read error");
        let mut iter = team_info.split_whitespace();
        let id: u32 = iter
            .next()
            .expect("Data error")
            .trim()
            .parse()
            .expect("Data error");
        let score: usize = iter
            .next()
            .expect("Data error")
            .trim()
            .parse()
            .expect("Data error");
        results_table[score].push(id);
    }

    for i in (0..MAX_SCORE).rev() {
        for j in 0..results_table[i].len() {
            println!("{} {}", results_table[i][j], i);
        }
    }
}
