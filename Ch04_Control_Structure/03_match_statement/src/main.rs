fn main() {
    println!("Hello, world!");
    demo_match_1();
    demo_match_2();
    demo_match_let();
}

fn demo_match_1() {
    println!("Demo match 1");
    let some_number = 100;
    match some_number {
        1 => println!("The number is 1"),
        2 | 3 => println!("The number is either 2 or 3"),
        4..=100 => println!("The number is between 4 and 100 inclusive"),
        _ => println!("The number is greater than 100"),
    }
}

fn demo_match_2() {
    println!("Demo match 2");
    let marks = 50;
    let mut grade = 'N';
    match marks {
        90..=100 => grade = 'A',
        80..=89 => grade = 'B',
        70..=79 => grade = 'C',
        60..=69 => grade = 'D',
        _ => grade = 'F',
    }

    println!("The grade is {}", grade);
}

fn demo_match_let() {
    println!("Demo match let");
    let marks = 75;
    let mut grade = 'N';
    let grade = match marks {
        90..=100 => 'A',
        80..=89 => 'B',
        70..=79 => 'C',
        60..=69 => 'D',
        _ => 'F',
    };

    println!("The grade is {}", grade);
}
