use std::fmt;

struct Structure(i32);

// Чтобы обеспечить возможность распечатывать пользовательский тип через "{}",
// а не только через "{:?}", необходимо реализовать трейт fmt::Display для этого типа
impl fmt::Display for Structure {
    // Сигнатура, которая используется
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // синтаксис макроса write! похож на println!
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}, z: {}", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {
    // Теперь значение структуры Struct можно распечатать
    println!("{}", Structure(5));

    let minmax = MinMax(0, 14);
    println!("Вывод через fmt::Debug: {:?}", minmax);
    println!("Вывод через fmt::Display: {}", minmax);

    let small_range = MinMax(-3, 3);
    let big_range = MinMax(-300, 300);
    println!("Маленький диапазон {small_range}, большой диапазон {big_range}.");

    let point = Point3D {x: 3.5, y: 7.4, z: 0.8};
    println!("Вывод через fmt::Debug: {:?}", point);
    println!("Вывод через fmt::Display: {}", point);

    let complex = Complex {real: 3.3, imag: 7.2};
    println!("Вывод через fmt::Debug: {:?}", complex);
    println!("Вывод через fmt::Display: {}", complex);
}
