use std::fmt;
use std::io;

const MAX_TEAMS_COUNT: usize = 150000 + 1;

#[derive(Clone, Copy)]
struct Team {
    id: u32,
    score: u8,
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.id, self.score)
    }
}

fn main() {
    let mut teams_count = String::new();
    io::stdin().read_line(&mut teams_count).expect("Read error");
    let teams_count: usize = teams_count.trim().parse().expect("Data error");
    let mut results_table: [Team; MAX_TEAMS_COUNT] = [Team { id: 0, score: 0 }; MAX_TEAMS_COUNT];

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
        results_table[index] = Team {
            id: id,
            score: score,
        };
    }
    for j in 1..teams_count {
        for i in 0..teams_count - j {
            if results_table[i].score < results_table[i + 1].score {
                let tmp = results_table[i + 1];
                results_table[i + 1] = results_table[i];
                results_table[i] = tmp;
            }
        }
    }
    for index in 0..teams_count {
        println!("{}", results_table[index]);
    }
}
