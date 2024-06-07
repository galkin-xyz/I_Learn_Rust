use std::io;

fn main() {
    let mut encryption = String::new();
    io::stdin().read_line(&mut encryption).expect("Read error");

    let mut decryption = String::new();
    let mut last_ch: Option<char> = None;
    for ch in encryption.chars() {
        if last_ch == None || last_ch != Some(ch) {
            decryption.push(ch);
            last_ch = Some(ch);
        } else {
            decryption.pop().expect("Data error");
            last_ch = decryption.chars().last();
        }
    }
    println!("{}", decryption);
}
