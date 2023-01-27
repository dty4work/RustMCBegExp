fn main() {
    // this is the first program in the course
    // This is the second line of comment

    /* This is
    a
    multi line
    comment
    */
    println!("Hello from rust program");

    // learning some basic output commands
    println!("The value of the constant is {}", 10);

    // Learning to print strings
    println!("My first name is {}, my last name is {}", "Nouman", "Azam");

    // Leawrning the print command
    print!("This is a print command");
    print!("This is going to be printed on the same line");
    println!("");

    // Learning to write on multiple line using the print command
    println!(
        "This is going to be 
                printed on multiple 
    line"
    );

    // Learning the use of escape sequences
    println!("\n\n This is going to be printed after two lines. \t This will have a tab");

    // Learning some uses of backslashes
    println!("This will print single quote \' and this will print double quote \"");
    println!("This is going to print one backslach \\");

    print!(
        "this is some text which will be overwritten \r this text will only appear on the screen\n"
    );

    // Learning the positional argument
    println!(
        "\n i am doing {2}, from  {1} years and i {0}",
        "like", 20, "programming"
    );

    // Learning named arguments
    println!(
        "{language} is a system programming language which is coold to {activity} in.",
        language = "Rust",
        activity = "code"
    );

    // learning to print basic maths
    println!("The summation of 25 + 10 = {}", 25 + 10);
}
