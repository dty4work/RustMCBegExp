fn main() {
    // Functions
    basic_fn();
    function_with_inputs("Nouman", 40_000);

    let full_name: &str = "Karman";
    let salary_info = 50_000;
    function_with_inputs(full_name, salary_info);

    let answer = function_with_inputs_outputs(10, 15);
    println!("the answer of multiplication is {}", answer);

    let (prd, sum, diff) = function_with_inputs_multiple_outputs(10, 15);
    println!("product {}, sum {}, diff {}", prd, sum, diff);

    let result = function_with_inputs_multiple_outputs(10, 15);
    println!("product {}, sum {}, diff {}", result.0, result.1, result.2);

    let first_name = "Nouman";
    let last_name = "Azam";

    let full_name = { format!("{} {}", first_name, last_name) };

    println!("My full name is {}", full_name);

    // Inputs from user
    let mut n = String::new();

    std::io::stdin()
        .read_line(&mut n)
        .expect("failed to read input");

    let n: f64 = n.trim().parse().expect("invalid input");
    println!("{:?}", n);
}

fn basic_fn() {
    println!("This is a basic function");
}

fn function_with_inputs(name: &str, salary: i32) {
    println!("The name is {} and the salary is {}", name, salary);
}

fn function_with_inputs_outputs(num1: i32, num2: i32) -> i32 {
    num1 * num2
}

fn function_with_inputs_multiple_outputs(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 * num2, num1 + num2, num1 - num2)
}
