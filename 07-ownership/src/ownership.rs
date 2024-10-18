/*
Ownership Rules
    - Each value in Rust has an owner.
    - There can only be one owner at a time.
    - When the owner goes out of scope, the value will be dropped.
*/
#[allow(unused)]

pub fn ownership_fn() {
    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward
        println!("Print string *s-literal*: {s}"); // do stuff with s
    } // this scope is now over, and s is no longer valid

    {
        let mut s = String::from("hello");

        s.push_str(", world!"); // push_str() appends a literal to a String

        println!("Print a s string *s-string type*: {s}"); // This will print `hello, world!`
        println!();
    }

    /*
        To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer 
        valid. Therefore, Rust doesn’t need to free anything when s1 goes out of scope.
        Check out what happens when you try to use s1 after s2 is created; it won’t work:
    */
    // {
    //     let s1 = String::from("hello");
    //     let s2 = s1;
    //     println!("{s1}, world!");
    // }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {s1}, s2 = {s2}");
        println!();
    }

    {
        let s1 = gives_ownership(); // gives_ownership moves its return value into s1

        let s2 = String::from("hello"); // s2 comes into scope

        let s3 = takes_and_gives_back(s2); // s2 is moved into
        // takes_and_gives_back, which also moves its return value into s3
    } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing happens. s1 goes out of scope and is dropped.
}

fn gives_ownership() -> String {
    // gives_ownership will move its return value into the function that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope

    a_string // a_string is returned and moves out to the calling function
}
