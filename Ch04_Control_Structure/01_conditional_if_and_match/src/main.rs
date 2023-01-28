fn main() {
    println!("Hello, Condition IF and MATCH");

    demo_if();
    demo_if_with_mult_cond();
    demo_if_else();
}

fn demo_if_else() {
    println!(" Demo - IF ... ELSE ...");
    let marks = 95;
    let mut grade = 'N';

    if marks >= 90 {
        grade = 'A';
    } else if marks >= 80 {
        grade = 'B';
    } else if marks >= 70 {
        grade = 'C';
    } else if marks >= 60 {
        grade = 'D';
    } else {
        grade = 'F';
    }

    println!("The obtained grade is {}", grade);
}

fn demo_if_with_mult_cond() {
    println!(" Demo - IF with Multi Cond");

    let marks = 65;
    if marks >= 60 && marks <= 70 {
        println!("The grade is satisfactory");
    }

    let flag_1 = true;
    let flag_2 = false;
    if flag_1 == true || flag_2 == true {
        println!("flag_1 = {flag_1}, flag_2 = {flag_2} ");
        println!("One of the condition is true here");
    }

    let flag_1 = true;
    if flag_1 != false {
        println!("This will execute when flag_1 = true and when flag_1 is not true");
    }

    let flag_1 = true;
    let flag_2 = false;
    let number = 60;
    if (flag_1 == true && flag_2 == false) || number < 50 {
        println!("This part will execute based on the conditions above");
    }
}

fn demo_if() {
    println!(" Demo - Condition IF");
    let some_number = 55;
    if some_number < 50 {
        println!("The number is less than 50");
    }

    println!("The line will execute irrespective the condition above");
}
