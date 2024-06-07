use std::io;

fn main() {
    let mut encryption = String::new();
    io::stdin().read_line(&mut encryption).expect("Read error");

    let mut decryption = String::new();
    let mut last_ch: char = '.';
    for (pos, ch) in encryption.chars().enumerate() {
        if pos == 0 {
            decryption.push(ch);
            last_ch = ch;
        } else {
            if ch == last_ch {
                decryption.pop().expect("Data error");
                last_ch = match decryption.chars().last() {
                    None => '.',
                    Some(ch) => ch,
                }
            } else {
                decryption.push(ch);
                last_ch = ch;
            }
        }
    }
    println!("{}", decryption);
}
