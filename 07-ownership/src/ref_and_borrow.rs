#[allow(unused)]

pub fn ref_and_borrow() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
    println!();

    /* You can't change the value when passed as a ref
    Just as variables are immutable by default, so are references.
    We’re not allowed to modify something we have a reference to.
    */
    // {
    //     let s = String::from("hello");
    //     change(&s);
    // }

    {
        let mut s = String::from("hello");
        change(&mut s);
        println!("Here is the 's' string after being passed to the 'change' function: {s}")
    }

    /*
    Mutable references have one big restriction: if you have a mutable reference to a value,
    you can have no other references to that value. This code that attempts to create
    two mutable references to s will fail:
     */
    // {
    //     let mut s = String::from("hello");
    //     let r1 = &mut s;
    //     let r2 = &mut s;
    //     println!("{}, {}", r1, r2);
    // }

    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

/* You can't change the value when passed as a ref */
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
