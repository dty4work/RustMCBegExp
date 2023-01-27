fn main() {
    // Tuples
    let my_information = ("Salary", 40_000);
    println!("{} is equal to {}", my_information.0, my_information.1);

    let (salary, salary_value) = my_information;
    println!(
        "The value of the individual variables are,
    {} and {}",
        &salary, salary_value
    );

    let salary = my_information.0;
    let salary_value = my_information.1;
    println!(
        "The value of the individual variables are,
    {} and {}",
        &salary, salary_value
    );

    let nested_tuple = (4, 5.0, (3, 2), "Hello");
    let element = nested_tuple.2 .0;
    println!("The value of the element is {}", element);

    let empty_tuple = ();

    // Arrays
    let mut number_array: [i32; 5] = [4, 5, 6, 8, 9];

    println!("{}", number_array[0]);
    println!("{:?}", number_array);

    number_array[4] = 5;
    println!("{:?}", number_array);

    let array_with_same_elements: [i32; 10] = [0; 10];
    println!("{:?}", array_with_same_elements);

    let mut string_array_1 = ["apple", "tomato", "grapes"];
    let string_array_2 = ["Unknown"; 6];

    string_array_1[0] = "Kamran Azam";

    println!("{:?}\n{:?}", string_array_1, string_array_2);

    let char_array = ['a', 'p', 'p', 'l', 'e'];

    let mut number_array_1: [i32; 5] = [4, 5, 6, 8, 9];

    println!("The original array is {:?}", number_array_1);
    println!("Elements in the array are {}", number_array_1.len());
    println!(
        "The array is occupying {} bytes",
        std::mem::size_of_val(&number_array_1)
    );

    let subset_array: &[i32] = &number_array[0..3];
    println!(
        "The subset of the values of the array are {:?}",
        subset_array
    );

    println!("Elements in the array are {}", subset_array.len());
    println!(
        "The array is occupying {} bytes",
        std::mem::size_of_val(subset_array)
    );

    // number_array_1[10] = 5;

    let check_indeex = number_array_1.get(2);
    println!("{:?}", check_indeex);
}
