use std::collections::HashSet;
use std::io;

fn main() {
    let mut teacher_dates_count = String::new();
    io::stdin()
        .read_line(&mut teacher_dates_count)
        .expect("Read error");
    let teacher_dates_count: usize = teacher_dates_count.trim().parse().expect("Data error");

    let mut teacher_dates = HashSet::new();
    for _ in 0..teacher_dates_count {
        let mut date = String::new();
        io::stdin().read_line(&mut date).expect("Read error");
        date = date.trim().to_string();
        teacher_dates.insert(date);
    }

    let mut stud_dates_count = String::new();
    io::stdin()
        .read_line(&mut stud_dates_count)
        .expect("Read error");
    let stud_dates_count: usize = stud_dates_count.trim().parse().expect("Data error");
    let mut coincidence_count: u32 = 0;
    for _ in 0..stud_dates_count {
        let mut date = String::new();
        io::stdin().read_line(&mut date).expect("Read error");
        date = date.trim().to_string();
        if teacher_dates.contains(&date) {
            coincidence_count += 1;
        }
    }

    println!("{coincidence_count }");
}
