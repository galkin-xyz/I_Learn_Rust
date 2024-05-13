use std::io;

fn main() {
    let mut code1 = String::new();
    io::stdin().read_line(&mut code1).expect("Ошибка ввода.");
    let code1: u16 = code1.trim().parse().expect("Ошибка данных.");

    let mut code2 = String::new();
    io::stdin().read_line(&mut code2).expect("Ошибка ввода.");
    let code2: u16 = code2.trim().parse().expect("Ошибка данных.");

    let res = if (code1 % 2 == 1) && (code2 % 2 == 0) {
        "no"
    } else {
        "yes"
    };
    println!("{res}");
}
