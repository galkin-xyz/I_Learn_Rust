use std::io;

fn main() {
    println!("Генерирование n-го числа Фибоначчи.");
    println!("Формула для вычисления: F0 = 0, F1 = 1, Fn = Fn-1 + Fn-2 для n > 1.");

    loop {
        println!("Введите номер члена последовательности или <Enter> для выхода.");
        let mut seq_number = String::new();
        io::stdin()
            .read_line(&mut seq_number)
            .expect("Ошибка при чтении данных!");
        let seq_number: u16 = match seq_number.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        println!(
            "Значение {} члена последовательности Фибоначчи равно {}",
            seq_number,
            fibonacci_number(seq_number)
        );
    }
}

fn fibonacci_number(n: u16) -> u32 {
    let mut fn_2: u32 = 0;
    let mut fn_1: u32 = 1;
    let mut r#fn: u32 = fn_1 + fn_2;

    match n {
        0 => fn_2,
        1 => fn_1,
        2 => r#fn,
        _ => {
            for _ in 3..=n {
                fn_2 = fn_1;
                fn_1 = r#fn;
                r#fn = fn_2 + fn_1;
            }
            r#fn
        }
    }
}
