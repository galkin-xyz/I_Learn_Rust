static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;

    println!("Это язык {}", LANGUAGE);
    println!("Пороговое значение {}", THRESHOLD);
    println!(
        "{} - {}",
        n,
        if is_big(n) {
            "большое"
        } else {
            "маленькое"
        }
    );
}
