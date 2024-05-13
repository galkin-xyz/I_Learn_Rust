use std::io;

fn main() {
    println!("Представление температуры в градусах Цельсия и Фаренгейта.");
    println!("0 - Выход из программы");
    println!("1 - Перевод температуры из градусов Цельсия в Фаренгейта");
    println!("2 - Перевод температуры из градусов Фаренгейта в Цельсия");
    let temp_type = ["Цельсия", "Фаренгейта"];

    loop {
        println!("Укажите требуемую операцию [0, 1, 2]:");
        let mut code_oper = String::new();
        io::stdin()
            .read_line(&mut code_oper)
            .expect("Ошибка чтения");
        let code_oper: usize = match code_oper.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ожидается ввод значения [0, 1, 2].");
                continue;
            }
        };
        if code_oper > 2 {
            println!("Ожидается ввод значения [0, 1, 2].");
            continue;
        }
        if code_oper == 0 {
            break;
        }
        println!(
            "Введите температуру в градусах {}",
            temp_type[code_oper - 1]
        );
        let mut temp_val = String::new();
        io::stdin().read_line(&mut temp_val).expect("Ошибка чтения");

        let temp_val: f64 = match temp_val.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Ожидается ввод числа с плавающей точкой.");
                continue;
            }
        };
        let conv_temp_val: f64;
        if code_oper == 1 {
            conv_temp_val = cel2fah(temp_val);
        } else if code_oper == 2 {
            conv_temp_val = fah2cel(temp_val);
        } else {
            println!("Ожидается ввод значения [0, 1, 2].");
            continue;
        }
        println!(
            "Температура в градусах {} равна {}",
            temp_type[code_oper % 2],
            conv_temp_val
        );
    }
}

fn cel2fah(cel_val: f64) -> f64 {
    (cel_val * 1.8) + 32.0
}

fn fah2cel(fah_val: f64) -> f64 {
    (fah_val - 32.0) / 1.8
}
