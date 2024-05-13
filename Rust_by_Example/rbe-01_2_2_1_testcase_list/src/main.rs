use std::fmt;

// Кортежная структура содержит вектор.
// При реализации трейта fmt::Display необходимо обработать возможную ошибку при выводе каждого
// элемента вектора
// Для этого используется оператор "?" в конце макроса write!
// "?" собирает множество результатов и возвращает ошибку, если возникла хотя бы одна ошибка
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;

        let vec = &self.0;
        for (count, v) in vec.iter().enumerate() {
            if count != 0 {
                write!(f, ", ")?
            };
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}

struct List2(Vec<i32>);

impl fmt::Display for List2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;

        let vec = &self.0;
        for (i, v) in vec.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", i, v)?;
        }
        write!(f, "]")
    }
}

fn main() {
    let vec = List(vec![1, 2, 3]);
    let vec_empty = List(vec![]);
    println!("{}", vec);
    println!("{}", vec_empty);

    let vec2 = List2(vec![1, 2, 3, 4, 5]);
    let vec2_empty = List2(vec![]);
    println!("{}", vec2);
    println!("{}", vec2_empty);
}
