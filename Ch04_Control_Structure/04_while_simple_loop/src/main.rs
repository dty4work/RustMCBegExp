use std::io::Read;

fn main() {
    println!("Hello, world!");

    // demo_loop_forever();
    // demo_while_cond();
    demo_while_divisible_by_2_5();
}

fn demo_while_divisible_by_2_5() {
    println!(
        "Enter a number and I will tell you the next number after 
    your number which is divisible by both 2 and 5"
    );

    let mut number = String::new();
    std::io::stdin()
        .read_line(&mut number)
        .expect("Failed to read input");
    let mut number: i32 = number.trim().parse().expect("Invalid input");

    while (number % 2 == 0 && number % 5 == 0) != true {
        number = number + 1;
    }

    println!(
        "The number after your number which is divisible by both 2 and 5 is {}",
        number
    );
}

fn demo_while_cond() {
    println!("Demo - While Cond");

    let my_number = 13;
    let mut guess = false;
    println!("Guess my number which is between 1 and 20");
    while guess != true {
        let mut number = String::new();
        std::io::stdin()
            .read_line(&mut number)
            .expect("failed to read input");
        let number: i32 = number.trim().parse().expect("Invalid input");

        if my_number == number {
            println!("You guess the number correctly");
            guess = true;
        } else {
            println!("Please try again");
        }
    }
}

fn demo_loop_forever() {
    println!("Demo - Loop");

    // Forever Loop
    loop {
        println!("This is a forever loop");
    }
}
