use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода");

    let input: i32 = input.trim().parse().expect("Ошибка данных");
    let res = if (1..=4).contains(&input) {
        "few"
    } else if (5..=9).contains(&input) {
        "several"
    } else if (10..=19).contains(&input) {
        "pack"
    } else if (20..=49).contains(&input) {
        "lots"
    } else if (50..=99).contains(&input) {
        "horde"
    } else if (100..=249).contains(&input) {
        "throng"
    } else if (250..=499).contains(&input) {
        "swarm"
    } else if (500..=999).contains(&input) {
        "zounds"
    } else {
        "legion"
    };

    println!("{res}");
}
