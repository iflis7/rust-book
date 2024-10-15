pub fn looping() {
    println!("Loops");

    println!("==> Returning Values from Loops");
    {
        let mut counter = 0;

        let result = loop {
            counter += 1;

            if counter == 10 {
                break counter * 2;
            }
        };

        println!("The result is {result}");
        println!("*****************************");
    }

    println!("==> Loop Labels to Disambiguate Between Multiple Loops");
    {
        let mut count = 0;
        'counting_up: loop {
            println!("count = {count}");
            let mut remaining = 10;

            loop {
                println!("remaining = {remaining}");
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
                remaining -= 1;
            }

            count += 1;
        }
        println!("End count = {count}");
        println!("*****************************");
    }

    println!("==> Conditional Loops with while");
    {
        let mut number = 3;
        while number != 0 {
            println!("{number}!");

            number -= 1;
        }

        println!("LIFTOFF!!!");
        println!("*****************************");
    }

    println!("==> Looping Through a Collection with while");
    {
        let a = [10, 20, 30, 40, 50];
        println!("Printing the a array: {:?}", a);
        let mut index = 0;

        while index < 5 {
            println!("the value is: {}", a[index]);

            index += 1;
        }
        println!("*****************************");
    }

    println!("==> Looping Through a Collection with for");
    {
        let a = [10, 20, 30, 40, 50];
        println!("Printing the a array: {:?}", a);

        for element in a {
            println!("the value is: {element}");
        }
    }

    println!("==> Looping through elements of a collection with for");
    {
        for number in (1..5).rev() {
            println!("{number}!");
        }
        println!("LIFTOFF!!!");
        println!("*****************************");
    }
}
