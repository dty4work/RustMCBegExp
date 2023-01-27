fn main() {
    // Vectors
    let mut number_vec: Vec<i32> = vec![4, 5, 6, 8, 9, 10, 11, 12, 15, 16, 12, 10];
    println!("{}", number_vec[0]);

    number_vec[4] = 5;
    println!("{:?}", number_vec);

    let array_with_same_element: Vec<i32> = vec![0; 10];
    let mut string_array_1: Vec<&str> = vec!["apple", "tomato", "grapes"];
    string_array_1[0] = "Kamran Azam";

    let string_array_2: Vec<&str> = vec!["Unknown"; 6];

    let char_vec: Vec<char> = vec!['a', 'p', 'p', 'l', 'e'];
    let subset_vec = &number_vec[0..3];
    println!("The subset of the values of the array are {:?}", subset_vec);
    println!("Elements in the array are {}", number_vec.len());

    let check_index = number_vec.get(2);
    println!("{:?}", check_index);

    number_vec.push(30);
    number_vec.push(40);
    println!("The values of the array are {:?}", number_vec);
    println!("Elements in the array are {}", number_vec.len());

    number_vec.remove(5);
    println!("The values of the array are {:?}", number_vec);
    println!("Elements in the array are {}", number_vec.len());

    println!(
        "The value of 10 exits in the array{}",
        number_vec.contains(&10)
    );

    // println!("The subset of the values of the array are {:?}", subset_vec);
    // println!("Elements in the array are {}", number_vec.len());
}
