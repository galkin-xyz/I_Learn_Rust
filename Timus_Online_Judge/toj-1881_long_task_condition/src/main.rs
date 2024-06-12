use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Read error");
    let mut iter = input.split_whitespace();
    let lines_per_page: u16 = iter
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
    let mut lines_count: u16 = 0;
    let mut index: usize = 0;
    while index < len_of_words.len() {
        let mut line_len: u8 = 0;
        loop {
            line_len += len_of_words[index] + 1;
            if line_len - 1 > symbols_per_line {
                break;
            } else {
                index += 1;
            }
            if index == len_of_words.len() {
                break;
            }
        }
        lines_count += 1;
    }
    println!(
        "{}",
        match lines_count % lines_per_page {
            0 => lines_count / lines_per_page,
            _ => lines_count / lines_per_page + 1,
        }
    );
}
