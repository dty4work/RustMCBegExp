fn main() {
    println!("Rust Ownership");
    println!("- Each value in Rust has a variable that's called its owner");
    println!("- There can be only one owner at at time");
    println!("- When the owner goes out of scope, the value will be dropped");

    println!();

    println!("Ownership and functions");
    let stack_num: i32 = 32;
    let mut heap_num: Vec<i32> = vec![4, 5, 6];

    stack_function(stack_num);
    println!(
        "The stack variable is copied and the original value was {}",
        stack_num
    );

    heap_function(&heap_num);
    println!(
        "The value of the vector inside the function is {:?}",
        heap_num
    );

    // Quiz - Who has the ownership?
    //  Ans: ref1

    let mut my_heap_num = vec![4, 5, 6, 7];
    let ref1 = my_heap_num;
    let ref2 = &ref1;

    // When will refences will be handy
    let large_data1 = String::from("This is the first long string");
    let large_data2 = String::from("This is the second long string");

    let huge_data = vec![&large_data1, &large_data2];
}

fn stack_function(mut var: i32) {
    var = 56;
    println!("The copied value of variable has been updated to {}", var);
}

fn heap_function(var: &Vec<i32>) {
    println!("The value of the vector inside the function is {:?}", var);
}
