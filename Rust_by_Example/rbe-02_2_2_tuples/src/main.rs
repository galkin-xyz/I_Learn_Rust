use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}


#[derive(Debug)]
#[allow(dead_code)]
struct Matrix(f32 ,f32 ,f32 ,f32);

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "")?;
        writeln!(f, "( {}, {} )", self.0, self.1)?;
        writeln!(f, "( {}, {} )", self.2, self.3)
    }
}

fn transpose(mut matrix: Matrix) -> Matrix {
    let tmp = matrix.1;
    matrix.1 = matrix.2;
    matrix.2 = tmp;

    matrix
}

fn main() {
    // элементы могут быть разных типов
    let long_tuple = (1u8, 2u16, 3u32, 4u64, 
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);
    println!("Первое значение длинного кортежа: {}", long_tuple.0);
    println!("Второе значение длинного кортежа: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u64, -1i16), -2i16);
    println!("Кортеж из кортежей {:?}", tuple_of_tuples);

    // максимальное количество элементов кортежа, 
    // которое можно распечатать через трейт Debug {:?} - 12
    
    let pair = (1, true);
    println!("Пара значений: {:?}", pair);
    println!("Обратный порядок пары значений: {:?}", reverse(pair));

    println!("Кортеж из одного элемента: {:?}", (5u32,));
    println!("Просто целое число: {:?}", (5u32));

    // Деструктурирование картежа
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("Матрица: {}", matrix);
    println!("Транспонированная матрица: {}", transpose(matrix));
}
