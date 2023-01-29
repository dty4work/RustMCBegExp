fn main() {
    println!("Hello, world!");
    demo_for();
}

fn demo_for() {
    // Exact number of time known
    println!("Demo For loop ");

    let mut some_vec = vec![45, 30, 85, 90, 41, 39];

    // for i in some_vec.iter_mut() {
    for i in &mut some_vec {
        // &mut some_vec === some_vec.iter_mut()
        *i += 5;
        println!("{}", i);
    }

    println!("{:?}", some_vec);

    // for i in 0..5 {
    //     println!("{}", some_vec[i]);
    // }

    // for i in some_vec {
    //     println!("{}", i);
    // }

    // for i in some_vec.iter() {
    //     println!("{}", i);
    // }

    // for i in &some_vec {
    //     // &some_vec = some_vec.iter()
    //     println!("{}", i);
    // }

    // println!("{:?}", some_vec);
}
