use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

const ISENBAEV: &str = "Isenbaev";

fn main() {
    let mut all_persons = HashMap::<String, Option<u16>>::new();
    let mut numbers = Vec::<HashSet<String>>::new();
    let mut teams = Vec::<HashSet<String>>::new();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    let teams_count: usize = input.trim().parse().expect("Data error");

    for _ in 0..teams_count {
        let mut team = HashSet::<String>::new();
        input.clear();
        io::stdin().read_line(&mut input).expect("Read error");
        for person in input.split_whitespace() {
            team.insert(person.to_string());
            all_persons.insert(person.to_string(), None);
        }
        teams.push(team);
    }

    println!("{:#?}", teams);
}
