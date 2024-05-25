use std::fmt;
use std::io;

const MAX_TEAMS_COUNT: usize = 150000 + 1;

struct Team {
    id: u32,
    score: u8,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{} {}", self.id, self.score)
    }
}

fn main() {
    let mut teams_count = String::new();
    io::stdin().read_line(&mut teams_count).expect("Read error");
    let teams_count: u32 = teams_count.trim().parse().expect("Data error");
    let mut results_table: [Team; MAX_TEAMS_COUNT];

    for index in 0..teams_count {
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
    }
}
