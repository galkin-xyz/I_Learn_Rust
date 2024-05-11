use std::mem;

fn analize_slice(slice: &[i32]) {
    println!("Первый элемент среза: {}", slice[0]);
    println!("Срез состоит из {} элементов", slice.len());
}

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("Первый элемент массива: {}", xs[0]);
    println!("Второй элемент массива: {}", xs[1]);
    println!("Массив состоит из {} элементов", xs.len());
    println!("Массив занимает {} байт", mem::size_of_val(&xs));

    println!("Заимствование всего массива как среза.");
    analize_slice(&xs);
    println!("Заимствование части массива как среза.");
    analize_slice(&ys[1..4]);

    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    // Безопасное обращение к элементам массива с помощью `.get`
    for i in 0..xs.len() + 1 { // Индекс превышает размер массива
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("{}: Индекс за пределами размера массива", i),
        }
    }
}

