use std::io;
const MAX_TEAMS_COUNT: usize = 150000 + 1;
struct Team {
    id: u32,
    score: u8,
}

fn main() {
    let mut teams_count = String::new();
    io::stdin().read_line(&mut teams_count).expect("Read error");
    let teams_count: u32 = teams_count.trim().parse().expect("Data error");

    for index in 0..teams_count {
        let mut team_result = String::new();
        io::stdin().read_line(&mut team_result).expect("Read error");
        let mut iter = team_result.split_whitespace();
    }
}
