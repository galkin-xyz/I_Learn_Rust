use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    let mut iter = input.split_whitespace();
    let lines_per_page: u8 = iter
        .next()
        .expect("Data error")
        .parse()
        .expect("Data error");
    let symbols_per_line: u8 = iter
        .next()
        .expect("Data error")
        .parse()
        .expect("Data error");
    let words_in_text: usize = iter
        .next()
        .expect("Data error")
        .parse()
        .expect("Data error");

    let mut len_of_words = Vec::<u8>::new();
    for _ in 0..words_in_text {
        input.clear();
        io::stdin().read_line(&mut input).expect("Read error");
        len_of_words.push(input.trim().to_string().len() as u8);
    }

    // check input
    println!("{lines_per_page} {symbols_per_line} {words_in_text}");
    for len in &len_of_words {
        println!("{len}");
    }
}
