use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ошибка ввода.");
    let input: u16 = input.trim().parse().expect("Ошибка данных.");

    const TASKS_COUNT: u16 = 12;
    const DURATION_OF_COMPETITION_H: u16 = 5;
    const DURATION_OF_FIRST_PART_H: u16 = 1;
    const REMAINING_TIME_M: u16 = (DURATION_OF_COMPETITION_H - DURATION_OF_FIRST_PART_H) * 60;
    const ONE_TASK_SOLVE_TIME_M: u16 = 45;
    let res = if (TASKS_COUNT - input) * ONE_TASK_SOLVE_TIME_M <= REMAINING_TIME_M {
        "YES"
    } else {
        "NO"
    };

    println!("{res}");
}
