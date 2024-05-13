#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    use crate::Status::{Poor, Rich};
    use crate::Work::*;
    
    // Эквивалентно Status::Poor
    let status = Poor;
    // Эквивалентно Work::Civilian
    let work = Civilian;
    
    match status {
        Rich => println!("Статус \"Богатый\""),
        Poor => println!("Статус \"Бедный\""),
    }

    match work {
        Civilian => println!("Гражданский"),
        Soldier => println!("Военный"),
    }
}
