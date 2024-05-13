#![allow(dead_code)]

// Элементы перечисления неявно нумеруются с 0
enum Number {
    Zero,
    One,
    Two,
}

// Можно задать явное числовое значение для элементов перечисления
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // Элементы перечисления могут быть преобразованы в число
    println!("Ноль {}", Number::Zero as i32);
    println!("Один {}", Number::One as i32);

    println!("Цвет розы #{:06x}", Color::Red as i32);
    println!("Цвет фиалки #{:06x}", Color::Blue as i32);
}
