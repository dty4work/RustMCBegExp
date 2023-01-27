use core::num;

fn main() {
    // Strings
    //      - String slices (&str) - Fixed length strings

    let some_string: &str = "Fixed Length String";
    println!("The text inside the string is \"{}\"", some_string);

    // Strings
    //      - Variable Length strings
    //      - adding removing characters

    let mut growable_string: String = String::from("This string will grow");
    println!("The text inside the string is \"{}\"", growable_string);

    growable_string.push('s');
    println!("The text inside the string is \"{}\"", growable_string);

    growable_string.pop();
    println!("The text inside the string is \"{}\"", growable_string);

    growable_string.push_str(" which i will use");
    println!("The text inside the string is \"{}\"", growable_string);

    println!(
        "I am going to tell you some basic things about the strings,
    Is the string empty {},
    The length of the string is {},
    The string has {} bytes,
    Does the string contains the word 'use' {}",
        growable_string.is_empty(),
        growable_string.len(),
        growable_string.capacity(),
        growable_string.contains("use")
    );

    growable_string.push_str("    ");
    println!(
        "The length of the string before the trim is {},
the string after the trim is {}",
        growable_string.len(),
        growable_string.trim().len()
    );

    let number = 6;
    println!(
        "The value of the number in string is {}",
        number.to_string()
    );

    println!(
        "is the number really a string {}",
        number.to_string() == "6"
    );

    let some_cahr = 'a';
    println!(
        "The value of the some_cahr in string is {}",
        some_cahr.to_string()
    );

    println!(
        "is the some_cahr really a string {}",
        some_cahr.to_string() == "a"
    );

    let my_name = "Nouman azam".to_string();
    println!("The string contains my name {}", my_name);
    // my_name.push_str(" is that it");

    let empty_string = String::new();
    println!("length is {}", empty_string.len());

    println!("Hello, world!");

    let s_1 = "Nouman".to_string();
    let s_2 = "Azam".to_string();
    let s_3 = format!("My first name is {} and my last name is {}", s_1, s_2);

    let string_1 = String::from("Nouman");
    let string_2 = String::from("Azam");
    let string_3 = format!(
        "My first name is {} and my last name is {}",
        string_1, string_2
    );

    println!("s_1 equals string_1 is {}", s_1 == string_1);
}
