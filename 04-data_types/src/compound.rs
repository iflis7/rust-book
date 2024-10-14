#[allow(unused)]

pub fn compound (){
    
    // Tuples 
    println!("***Tuples***");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("This is a tuple ==> {:?}", tup);

    let (x, y, z) = tup;
    println!("The value of y is: {y}");
    
    let first = tup.0;
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", tup.1);
    println!("The value of third is: {}", tup.2);
    println!();
    
    // Array
    println!("***Arrays***");
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];
    
    println!("The value of the a array is: {:?}", a);
    println!("The value of the b array is: {:?}", b);
    println!("The value of the c array is: {:?}", c);
    println!("The value of the first index of the a array is: {:?}", a[0]);
    println!("The value of the months array is: {:?}", months);
    println!("The value of the first month is: {:}", months[0]);
    // println!("The value of the first month is: {:}", months[12]); //Index out of bound error
    println!();
}

