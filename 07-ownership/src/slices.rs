#[allow(unused)]

pub fn slicing() {
    {
        println!("\n *** Slicing ***");

        let mut s = String::from("hello world");

        let word = first_word(&s); // word will get the value 5

        // word still has the value 5 here, but there's no more string that
        // we could meaningfully use the value 5 with. word is now totally invalid!

        println!("the first word is: {word}");
        s.clear(); // this empties the String, making it equal to ""
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    println!("Here are the bytes: {:?}", bytes);

    for (i, &item) in bytes.iter().enumerate() {
        println!("Here are the 'i': {i} and the 'item' {item}");
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

/*
Key Concepts:
1- Ownership and Borrowing: A slice (&[start..end]) doesn't take ownership, it borrows a part of the data.
2- String slices: &str is the type for slices of strings. It's used for string literals as well as parts of String objects.
3- Immutable Reference Lifetimes: The slice ties the returned part of the string to the original data, preventing invalid references if the data changes.

Safer Code:
If you try to change the string while a slice exists, Rust will throw a compile-time error. This is because you can't have a mutable and an immutable reference at the same time:
rust
    let word = first_word(&s);
    s.clear(); // error!

More Flexible:
You can make the function more general by changing its parameter type to &str, which allows it to accept both string slices and references to String:
rust

    fn first_word(s: &str) -> &str {
        // same implementation
    }

Beyond Strings:
Slices work on other collections, like arrays:
rust

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // Refers to the subarray [2, 3]
*/