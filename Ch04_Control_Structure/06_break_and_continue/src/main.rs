fn main() {
    println!("Hello, world!");
    demo_break();
    demo_continue();
}

fn demo_break() {
    println!("Demo - Break for stopping a loop");

    let mut var = 100;
    loop {
        var = var - 1;
        if var % 13 == 0 {
            break;
        }
    }
    println!(
        "The higest number less than the given number divisible by 13 is {}",
        var
    );
}

fn demo_continue() {
    let mut var = 0;
    let mut count = 0;

    loop {
        var = var + 1;
        if var % 5 == 0 && var % 3 == 0 {
            println!("The number is divisible by both 3 and 5 is {}", var);
            count = count + 1;
            if count == 3 {
                break;
            } else {
                continue;
            }
        }
        println!("{}", var);
    }
}
