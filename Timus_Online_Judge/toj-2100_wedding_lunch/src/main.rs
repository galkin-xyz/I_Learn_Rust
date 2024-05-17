use std::io;

fn main() {
    let mut invitations_count = String::new();
    io::stdin()
        .read_line(&mut invitations_count)
        .expect("Ошибка ввода.");
    let invitations_count: usize = invitations_count.trim().parse().expect("Ошибка данных.");

    let mut guests_count: u8 = 2;
    for _ in 0..invitations_count {
        let mut invitation = String::new();
        io::stdin()
            .read_line(&mut invitation)
            .expect("Ошибка ввода.");
        let trimmed_inv = invitation.trim();
        if (trimmed_inv.len() > 4) && (&trimmed_inv[trimmed_inv.len() - 4..] == "+one") {
            guests_count += 2;
        } else {
            guests_count += 1;
        }
    }
    if guests_count == 13 {
        guests_count += 1;
    }
    let wedding_lunch_cost: u16 = guests_count as u16 * 100;
    println!("{wedding_lunch_cost}");
}
