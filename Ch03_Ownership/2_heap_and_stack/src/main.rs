const MAX_VALUE: i32 = 40_000;

fn main() {
    let (x, y) = (2, 4);
    let sum_value = square_sum(x, y);
    println!("The value of the Square of sum = {}", sum_value);
}

fn square_sum(num1: i32, num2: i32) -> i32 {
    let result = square(num1 + num2);
    result
}

fn square(num: i32) -> i32 {
    num * num
}
