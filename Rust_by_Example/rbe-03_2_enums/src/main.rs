enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("страница загружена"),
        WebEvent::PageUnload => println!("страница выгружена"),
        WebEvent::KeyPress(c) => println!("нажата клавиша '{}'.", c),
        WebEvent::Paste(s) => println!("вставлена строка \"{}\".", s),
        WebEvent::Click { x, y } => {
            println!("нажатие в точке x = {}, y = {}.", x, y);
        },
    }
}

#[allow(dead_code)]
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Создание псевдонима для перечисления
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers; 

#[allow(dead_code)]
impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("мой текст".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    let _x = Operations::Add;
}
