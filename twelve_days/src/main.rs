fn main() {
    println!("Двенадцать дней Рождества Twelve Days of Christmas");
    println!("(Английская народная песня-считалка-скороговорка)");
    println!("");

    let days = ["", "В первый", "Во второй", "На третий", "На четвёртый", "На пятый", "На шестой",
    "На седьмой", "На восьмой", "На девятый", "В десятый", "В одиннадцатый", "В двенадцатый"];

    let gifts = ["", "серую куропатку на грушевом дереве", "двух горлиц", 
    "трёх фаверолей, куриц французских", "четырех птиц говорящих", "пять колец золотых", 
    "шесть гусынь, яйца несущих", "семь плавающих лебедей", "восемь молодых доярок", 
    "девять танцующих леди", "десять прыгающих лордов", "одиннадцать трубящих трубачей", 
    "двенадцать барабанящих барабанщиков"];

    for current_day in 1..=12 {
        println!("{} день Рождества", days[current_day]);
        let clarification = if current_day > 7 {
                "на Святки " 
            } else {
                ""
            };
        println!("послала мне {}любовь моя верная", {clarification});
        let prefix_for_first_gift = if current_day == 1 {
            ""
        } else {
            "и "
        };
        for gift_index in (1..=current_day).rev() {
            let postfix = match gift_index {
                1 => ".",
                2 => "",
                _ => ","
            };
            if gift_index == 1 {
                println!("{}{}{}", prefix_for_first_gift, gifts[gift_index], postfix);
            } else {
                println!("{}{}", gifts[gift_index], postfix);

            }
        }
        println!("");
    }
}
