// Наследуем трейт fmt::Debug для обеспечения возможности печати через {:?}
#[derive(Debug)]
// Разрешаем "Dead code" т.к значение поля структыры далее не используется.
#[allow(dead_code)]
struct Structure(i32);

#[derive(Debug)]
#[allow(dead_code)]
struct Deep(Structure);

#[derive(Debug)]
#[allow(dead_code)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

fn main() {
    println!("Месяцев в году - {:?}.", 12);
    println!(
        "{1:?} {0:?} имя {actor:?}.",
        "Slater",
        "Christian",
        actor = "актёра"
    );

    println!("Теперь {:?} можно распечатать.", Structure(3));
    println!("Теперь {:?} можно распечатать.", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    // "Прелестная" распечатка с помощью {:#?}
    println!("{:#?}", peter);
}
