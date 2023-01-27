fn main() {
    println!("Rust Ownership");
    println!("- Each value in rust has a variable that's called its owner");
    println!("- There can be only one owner at a time");
    println!("- When the owner goes out of scope, the value will be dropped");
    println!();
    println!("Copy and Move");
    println!("Primitives and non-primitives");
    println!("move = leads to change of ownership");
    println!("borrow = does not change ownership");

    // let mut x = 32;
    // let mut y = x;
    // println!("The value of x = {} and the value of y = {}", x, y);

    // let s1 = String::from("abc");
    // let s2 = &s1;
    // println!("The value of s1 = {} and the value of s2 = {}", s1, s2);

    // let num_vec1: Vec<i32> = vec![5, 6, 9, 8, 7];
    // let num_vec2: &Vec<i32> = &num_vec1;
    // println!(
    //     "The first vector is {:?} and the second vector is {:?}",
    //     num_vec1, num_vec2
    // );

    // let num_vec2 = num_vec1.clone();
    // println!(
    //     "The first vector is {:?} and the second vector is {:?}",
    //     num_vec1, num_vec2
    // );

    {
        let my_name = String::from("Nouman Azam");
    }
    println!("My name is {}", my_name);
}
