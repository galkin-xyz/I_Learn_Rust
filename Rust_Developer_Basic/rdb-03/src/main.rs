fn main() {
    println!("Hello, world!");
}

fn double_int32(int32: i32) -> i32 {
    int32 * 2
    // todo: overflow check
}

fn double_int64(int64: i64) -> i64 {
    int64 * 2
    // todo: overflow check
}

fn double_float32(float32: f32) -> f32 {
    float32 * 2.0
    // todo: overflow check
}

fn double_float64(float64: f64) -> f64 {
    float64 * 2.0
    // todo: overflow check
}

fn int_plus_float_to_float(int: i32, float: f32) -> f64 {
    int as f64 + float as f64
}

fn int_plus_float_to_int(int: i32, float: f32) -> i64 {
    int as i64 + float.round() as i64
}

fn tuple_sum(tuple: (i32, i32)) -> i64 {
    tuple.0 as i64 + tuple.1 as i64
}

fn array_sum(array: [i32; 3]) -> i64 {
    array[0] as i64 + array[1] as i64 + array[2] as i64
}
