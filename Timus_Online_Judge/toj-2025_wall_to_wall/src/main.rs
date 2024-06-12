use std::io;
const MAX_TESTS_COUNT: usize = 10;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    let tests_count: usize = input.trim().parse().expect("Data error");

    let mut tests: [(u32, u32); MAX_TESTS_COUNT] = [(0, 0); MAX_TESTS_COUNT];
    for i in 0..tests_count {
        input.clear();
        io::stdin().read_line(&mut input).expect("Read error");
        let mut iter = input.split_whitespace();
        tests[i] = (
            iter.next()
                .expect("Data error")
                .parse()
                .expect("Data error"),
            iter.next()
                .expect("Data error")
                .parse()
                .expect("Data error"),
        );
    }
    for i in 0..tests_count {
        let (fighters, teams) = tests[i];
        let line_up = fighters / teams;
        let teams_plus_one = fighters % teams;

        let fights = (line_up * (fighters - line_up) * (teams - teams_plus_one)
            + (line_up + 1) * (fighters - (line_up + 1)) * teams_plus_one)
            / 2;
        println!("{fights}");
    }
}
