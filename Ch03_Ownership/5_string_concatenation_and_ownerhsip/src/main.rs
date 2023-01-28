fn main() {
    // println!("String concatenation and ownership");
    // // let s1 = String::from("Hello");
    // // let s2 = "world";

    // // let s3 = s1 + s2;

    // // // println!("{}", s1);
    // // println!("{}", s2);

    // let s1 = String::from("Hello");
    // let s2 = String::from("World");

    // let s3 = s1 + &s2;
    // println!("{} {}", s3, s2);

    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let s3 = String::from("Rust");
    let s4 = s1 + &s2 + &s3;

    println!("{} {} {}", s4, s2, s3);
    println!("{}", s1);
}
