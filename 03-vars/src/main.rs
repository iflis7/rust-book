fn main() {
    // Use *mut* keyword to make the variable mutable
    let mut y = 5;
    println!("The value of x is: {y}");
    y = 6;
    println!("The value of x is: {y}");

    //  Shdowing
    // *1*
    let x = 5;
    let x = x + 1;
    {
        let x = x + 1;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // *2*
    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {spaces}");

}
