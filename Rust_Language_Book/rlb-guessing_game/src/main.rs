use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Игра угадай число.");

    let secret_number: u32 = rand::thread_rng().gen_range(0..=100);
    //    println!("Загадано число {secret_number}.");

    loop {
        println!("Введите вашу догадку.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Не удалось получить значение.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ожидается ввод числа!");
                continue;
            }
        };
        println!("Получено число {guess}.");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Введнное число меньше"),
            Ordering::Greater => println!("Введённое число больше."),
            Ordering::Equal => {
                println!("Вы угадали!");
                break;
            }
        }
    }
}
