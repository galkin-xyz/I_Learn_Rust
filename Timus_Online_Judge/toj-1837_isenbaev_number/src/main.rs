use std::collections::HashMap;
use std::collections::HashSet;
use std::io;

const ISENBAEV: &str = "Isenbaev";

fn main() {
    let mut all_persons = HashMap::<String, Option<u16>>::new();
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

    let mut levels = Vec::<HashSet<String>>::new();
    let mut next_level = HashSet::<String>::new();
    next_level.insert(ISENBAEV.to_string());

    while !next_level.is_empty() {
        let processed_level: usize = levels.len();
        levels.push(HashSet::<String>::new());
        for person in &next_level {
            if let Some(level) = all_persons.get_mut(person) {
                *level = Some(processed_level as u16);
            }
            levels[processed_level].insert(person.to_string());
        }
        next_level.clear();

        for team in &mut teams {
            if !team.is_empty() {
                let mut is_found = false;
                for person in &levels[processed_level] {
                    is_found = is_found || team.remove(person);
                }
                if is_found {
                    for person in team.drain() {
                        if all_persons.get(&person).unwrap().is_none() {
                            next_level.insert(person.to_string());
                        }
                    }
                }
            }
        }
    }
    let mut sorted_persons: Vec<_> = all_persons.into_iter().collect();
    sorted_persons.sort_unstable();

    for person in sorted_persons {
        let level = match person.1 {
            None => String::from("undefined"),
            Some(x) => x.to_string(),
        };
        println!("{} {}", person.0, level);
    }
}
