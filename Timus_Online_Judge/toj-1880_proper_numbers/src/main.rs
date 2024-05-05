use std::io;

fn main() {
    let (pn1_count, pn1) = get_proper_numbers();
    let (pn2_count, pn2) = get_proper_numbers();
    let (pn3_count, pn3) = get_proper_numbers();
    let mut pn_command_count: u16 = 0;
    let mut index2: usize = 0;
    let mut index3: usize = 0;

    'main_loop: for index1 in 0..pn1_count {
        let candidate = pn1[index1];

        while pn2[index2] < candidate {
            index2 += 1;
            if index2 == pn2_count {
                break 'main_loop;
            }
        }
        if pn2[index2] > candidate {
            continue;
        }
        while pn3[index3] < candidate {
            index3 += 1;
            if index3 == pn3_count {
                break 'main_loop;
            }
        }
        if pn3[index3] > candidate {
            continue;
        }
        pn_command_count += 1;
    }
    println!("{pn_command_count}");
}

fn get_proper_numbers() -> (usize, [i32; 4000]) {
    let mut pn: [i32; 4000] = [0; 4000];
    let mut pn_count = String::new();
    io::stdin().read_line(&mut pn_count).expect("Ошибка ввода.");
    let pn_count = pn_count.trim().parse().expect("ошибка данных.");
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers).expect("Ошибка ввода.");
    let mut index: usize = 0;
    for number in numbers.split_whitespace() {
        pn[index] = number.trim().parse().expect("Ошибка данных.");
        index += 1;
    }

    (pn_count, pn)
}
