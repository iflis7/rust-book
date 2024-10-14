pub fn scalar_type() {
    /* Scalar Types
    A scalar type represents a single value. Rust has four primary scalar types: 
    integers, floating-point numbers, Booleans, and characters. */

    // let guess: u32 = "42".parse().expect("Not a number!");

    // Integer Types
    /*
        Length  |   Signed	|   Unsigned
        8-bit	    i8	        u8
        16-bit	    i16	        u16
        32-bit	    i32	        u32
        64-bit	    i64	        u64
        128-bit	    i128	    u128
        arch	    isize	    usize
        ------------------------
        ------------------------
        Number literals |   Example
        Decimal	            98_222
        Hex	                0xff
        Octal	            0o77
        Binary	            0b1111_0000
        Byte (u8 only)	    b'A'
    */
    // Floating-Point Types
    {
        let x = 2.0; // f64
        println!("This is an f64 float: {:.1}", x);
        let y: f32 = 3.0; // f32
        println!("This is an f32 float: {:^10.2}", y);
        println!();
    }

    // Numeric Operations
    {
        // addition
        let sum = 5 + 10;
        println!("This is an addition ==> 5 + 10 = {:.1}", sum);

        // subtraction
        let difference = 95.5 - 4.3;
        println!("This is a subtraction ==> 95.5 = - 4.3 {:.2}", difference);

        // multiplication
        let product = 4 * 30;
        println!("This is a multiplication ==> 4 * 30 = {}", product);

        // division
        let quotient = 56.7 / 32.2;
        let truncated = -5 / 3; // Results in -1
        println!("This is a division (quotient) ==> 56.7 / 32.2 = {:.1}", quotient);
        println!("This is a division (truncated) ==> -5 / 3 = {}", truncated);

        // remainder
        let remainder = 43 % 5;
        println!("This is a remainder ==> 43 % 5 = {}", remainder);
        println!();
    }

    // The Boolean Type
    {
        let t = true;
        let f: bool = false; // with explicit type annotation
        println!("This is a Boolean (t = true) ==> {}", t);
        println!("This is a Boolean (f: bool) ==> false = {}", f);
    }

    // The Character Type
    {
        let c = 'z';
        println!("This is a Character (c = 'z') ==> {}", c);
        let z: char = 'ℤ'; // with explicit type annotation
        println!("This is a Character (z: char = 'ℤ') ==> {}", z);
        let heart_eyed_cat = '😻';
        println!("This is a Character (heart_eyed_cat = '😻') ==> {}", heart_eyed_cat);
    }
}
