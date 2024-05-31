use std::io;

fn is_happiness(ticket: u32) -> bool {
    ticket % 1000000 / 100000 + ticket % 100000 / 10000 + ticket % 10000 / 1000
        == ticket % 1000 / 100 + ticket % 100 / 10 + ticket % 10 / 1
}

fn main() {
    let mut ticket = String::new();
    io::stdin().read_line(&mut ticket).expect("Read error");
    let cur_ticket: u32 = ticket.trim().parse().expect("Data error");
    let next_ticket: u32 = cur_ticket + 1 % 1000000;
    let prev_ticket: u32 = if cur_ticket > 0 {
        cur_ticket - 1
    } else {
        999999
    };
    let is_one_step_from_happiness: bool = is_happiness(next_ticket) || is_happiness(prev_ticket);
    println!(
        "{}",
        if is_one_step_from_happiness {
            "Yes"
        } else {
            "No"
        }
    );
}
