use std::io;

fn main() {
    let mut letters_count = String::new();
    io::stdin()
        .read_line(&mut letters_count)
        .expect("Read error");
    let letters_count: u16 = letters_count.trim().parse().expect("Data error");

    let mut cur_pos: i8 = 0;
    let mut steps_count: u16 = 0;
    for _ in 0..letters_count {
        let mut addressee = String::new();
        io::stdin().read_line(&mut addressee).expect("Read error");
        let next_pos: i8 = match addressee.trim() {
            "Alice" | "Ariel" | "Aurora" | "Phil" | "Peter" | "Olaf" | "Phoebus" | "Ralph"
            | "Robin" => 0,
            "Bambi" | "Belle" | "Bolt" | "Mulan" | "Mowgli" | "Mickey" | "Silver" | "Simba"
            | "Stitch" => 1,
            "Dumbo" | "Genie" | "Jiminy" | "Kuzko" | "Kida" | "Kenai" | "Tarzan" | "Tiana"
            | "Winnie" => 2,
            _ => panic!("Unknown addressee"),
        };
        steps_count += (next_pos - cur_pos).abs() as u16;
        cur_pos = next_pos;
    }
    println!("{steps_count}");
}
