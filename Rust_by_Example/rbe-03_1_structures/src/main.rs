#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// Единично-подобная структура
struct Unit;

// Кортежная структура
struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

// Структура может использоваться для объявления типа полей другой структуры
#[allow(dead_code)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: left, y: top },
        bottom_right: Point {
            x: right,
            y: bottom,
        },
    } = rect;

    (right - left) * (top - bottom)
}

fn square(top_left: Point, side: f32) -> Rectangle {
    Rectangle {
        bottom_right: Point {
            x: top_left.x + side,
            y: top_left.y - side,
        },
        top_left,
    }
}

fn main() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    println!("{peter:?}");

    let point: Point = Point { x: 10.3, y: 0.4 };
    let another_point: Point = Point { x: 5.2, y: 0.2 };
    println!("Координаты точки: ({}, {})", point.x, point.y);

    let bottom_right = Point {
        x: 5.2,
        ..another_point
    };
    println!(
        "Координаты второй точки: ({}, {})",
        bottom_right.x, bottom_right.y
    );

    // Деструктурирование структуры point
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // Экземпляр структуры является выражением
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    let _unit = Unit;

    let pair = Pair(1, 0.1);
    println!("Пара состоит из {:?} и {:?}", pair.0, pair.1);

    // Деструктурирование кортежной структуры
    let Pair(integer, decimal) = pair;
    println!("Пара состоит из {:?} и {:?}", integer, decimal);

    let rect = Rectangle {
        top_left: Point { x: 1.0, y: 10.0 },
        bottom_right: Point { x: 11.0, y: 5.0 },
    };
    println!("Площадь прямоугольника: {}", rect_area(&rect));

    println!(
        "Площадь квадрата со стороной {} равна {}",
        6.0,
        rect_area(&square(Point { x: 1.0, y: 1.0 }, 6.0))
    );
}
