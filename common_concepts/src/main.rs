use std::io;

fn main() {

    // VARIABLES AND MUTABILITY
    let x = 5;
    // x = 6; // you can't do this, because this is immutable
    println!("The value of x is {x}");

    let mut y = 5;
    println!("The value of y is {y}");
    y = 10;
    println!("you can change y into {y}");

    // const, is like immutable types, but cannot but mutable in any case
    // but unlike variables, constant only can be constant experssions with real values
    // the value must be already known at compile time
    
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 3 * 60;
    println!("{}", THREE_HOURS_IN_SECONDS); 
    // const Y_HOURS: u32 = 60 * y;

    // naming convention is to all caps
    // because it directly know at compile time, it avoid memmory allocation or lookups
   
    // Shadowing: reiterating new values to a variable using `let`
    let x = x + 10; 
    println!("you can reiterate x with `let` to {x}");

    // Once shadowed previous acces cannot be accessed again execpt if its within a scope, it only
    // is available within the scope
    {
        let x = x * 2;
        println!("in this scope x is {x}");
    }

    println!("it's still {x} in this scope");
   
    // Difference between `mut` and shadowing is, that shadowing creating an entirely new variable
   
    let spaces = 30;
    println!("yulisa umurnya {spaces}");
    let spaces = "Efek Rumah Kaca";
    println!("aku suka {spaces}");

    let mut king = "Biden";
    //king = 15; //gabisa karena they expect a string
    println!("{}", king);
    king = "Trump"; 
    println!("{}", king);

    // DATA TYPES
        // Rust is a Statically Typed Language
        // it must know all the data types on compile time
        // There are two types of Data Types in rust, Scalar and Compound
        // for not used variables use '_' in the left of the variable name to not trigger warning 
    
    // SCALAR
    
    let days_of_the_week: u8 = 7; //unassigned 8-bit scalar
    let _weekend: i16 = (days_of_the_week - 5).into(); //signed (interger) 16 bit scalar
    
    let _x = 2.1; // f64 by default
    let _y: f32 = 3.4;

    // Mathematical Operations
    
    let _sum = 3 + 2;
    let _difference = 3.44 - 2.12;
    let _product = 4 * 30; //multiplication
    let _division = 56.4 / 32.2; // akan bukan pecahan
    let _truncated = -5 / 3; // Results in -1
    let _remainder = 43 % 5;

    // Booleans
    
    let _t = true;
    let _f: bool = false; //with explicit type annotation

    // Character type
    
    let _c = 'z'; // we use single quotes
    let _z: char = 'Z'; // with explicit type annotations

        // Char in the rus definition represent a Univode Scalar value and four bytes in size

    // COMPOUND
        // Group multiple values into one type

    // Tuples
        // They can have different types
        // once declared they cannot shrink and grow in size

    let tup: (i32, f64, u8) = (500, 6.4, 32);
    
    let (_x, y, _z) = tup; // destructuring, breaking the tuple into many parts

    println!("Value of y is {y}");
    println!("{}", tup.0); // get the first value of the tuple

    // Array
        // fixed length, can only have the same data types  

    let _a = [1, 2, 3, 4, 5];

        // good if you know the number of elements won't change like months
    let months = ["Januari", "Februari", "Maret", "udah ya capek"];

    println!("{}", months[3]);
   
    let _a: [i32; 5] = [1, 2, 3, 4, 5]; // explicit type annotation for element type and size

    let _a = [3; 5]; // the value will [3, 3, 3, 3, 3]
    
    // Indexing out of range example 
    { 
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Cannot readline");

        let input: usize = input.trim().parse().expect("please insert a number");
        // for indexing we must use usize

        println!("{}", months[input]); 
    }   // this will compile succesfully
        // but will go runtime error if input out of range 

    // Vector
        // Similar datatype provided by the standard library

    let _b: Vec<i32> = Vec::new(); // declaring a variable as vector
    let mut b = vec![1, 2, 3, 4, 5];
    b.push(10);

    println!("{}", b[5]);
    

}
