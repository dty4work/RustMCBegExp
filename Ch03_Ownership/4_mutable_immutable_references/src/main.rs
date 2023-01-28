// Reference Rules

fn main() {
    println!("Reference Rules");
    println!("  - One mutable reference in a scope");
    println!("  - Many immutable references");
    println!("  - mutable and immutable can not coexist");
    println!("  - scope of a reference");
    println!("  - Data should not change when immutable references are in scope");

    // println!("Cannot be borrowed ");
    // let mut heap_num: Vec<i32> = vec![4, 5, 6, 7];
    // let ref1 = &mut heap_num;

    // println!(
    //     "The original is {:?} heap_num and the first reference is {:?}",
    //     heap_num, ref1
    // );

    // demo_borrow_more_than_once_mutable();

    // demo_both_borrowed_mutable_and_immutable();

    // demo_scope_of_a_reference

    // demo_data_not_changed_with_immutable
}

fn demo_data_not_changed_with_immutable() {
    let mut heap_num = vec![4,5,6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    // heap_num.push(86);       // Not compile
    println!("Immutable references are {:?} and {:?}", ref1, ref2);
    // hap_num.push(86);        // Works with this
}

fn demo_scope_of_a_reference() {
    println!("demo scope of a reference");
    let mut heap_num = vec![4,5,6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    println!("Immutable references are {:?} and {:?}", ref1, ref2);

    let ref3 = &mut heap_num;
}

fn demo_scope_of_a_reference() {
    let mut heap_num = vec![4,5,6];
    let ref1 
}

fn demo_borrow_more_than_once_mutable() {
    println!("Cannot borrow mutable more than once");
    let mut heap_num: Vec<i32> = vec![4, 5, 6, 7];
    let ref1 = &mut heap_num;
    let ref2 = &mut heap_num; // No data access

    println!(
        "The first reference is {:?} and the second reference is {:?}",
        ref1, ref2
    );
}

fn demo_both_borrowed_mutable_and_immutable() {
    println!("Both mutable and immutable reference cannot exist at the same time");
    let mut heap_num = vec![4, 5, 6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    let ref3 = &mut heap_num;
    println!(
        "Immutable references are {:?} and {:?} and the mutable reference is {:?}",
        ref1, ref2, ref3
    );
}
