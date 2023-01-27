fn main() {
    // Initializing multiple variables
    let (first_number, second_number) = (250, 480.32);
    println!(
        "The first number is {} and the second number is {}",
        first_number, second_number
    );

    // Readability of large number
    let large_number = 1_000_000;
    println!("The value of the large number is {}", large_number);

    // Integer overflow
    // let overflow_number: u8 = 256;

    // Decimal numbers in other formats
    let x: i32 = 255;
    println!(
        "The value of variable x in hexadecimal is {:0} and in octal is {:X} and in binary {:b}",
        x, x, x
    );

    // Snake case convention
    let Number: i32 = 45;

    // Operation on number in different formats

    let n1: i32 = 14;
    let n2: f64 = 15.6;
    let n3 = n1 as f64 + n2;

    println!("{}", n3);

    // Shadowing
    // let s = 5;
    // let s = 5 * 5;
    // println!("The value of the variable s = {}", s);

    let mut s = 5;
    let s = 5 * 6;
    println!("The value of the variable s = {}", s);

    let s = 32;
    println!("The value of the variable s = {}", s);

    let s = 'A';
    println!("The value of the variable s = {}", s);

    let s = 64.5;
    println!("The value of the variable s = {}", s);

    let mut s = 65;
    {
        let s = 60;
        println!("The value of the inner variable s = {}", s);
    }

    println!("The value of the outer variable s = {}", s);

    // Constants
    const MAX_SALARY: u32 = 100_000;
    println!("The value of constant {}", MAX_SALARY);
}
