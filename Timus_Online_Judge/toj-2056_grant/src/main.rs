use std::io;

fn main() {
    let mut exam_count = String::new();
    io::stdin().read_line(&mut exam_count).expect("Read error");
    let exam_count: usize = exam_count.trim().parse().expect("Data error");

    let mut is_named: bool = true;
    let mut is_none: bool = false;
    let mut grade_total: u8 = 0;
    for _ in 0..exam_count {
        let mut grade = String::new();
        io::stdin().read_line(&mut grade).expect("Read error");
        let grade: u8 = grade.trim().parse().expect("Data error");

        is_named = is_named && (grade == 5);
        is_none = is_none || (grade == 3);
        grade_total += grade;
    }
    let res = if is_named {
        "Named"
    } else if is_none {
        "None"
    } else if grade_total as f64 / exam_count as f64 >= 4.5 {
        "High"
    } else {
        "Common"
    };
    println!("{res}");
}
