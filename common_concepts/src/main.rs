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

    // FUNCTIONS
   
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Cannot readline");

    let test: i32 = another_function(&input); // jangan lupa '&' untuk reference ke var
    println!("{}", test);

    let m = {
        let x = 2;
        x
    };

    println!("{}", m);

    let num = times_two_hundred(26);

    println!("{}", num);


    // CONTROL FLOWS
    flow(3);
    
    looper(2121);

    looper2();

    looper3();

    while_looper(10);

    for_looper2();
}

    // FUNCTION

fn another_function(name: &str) -> i32 { // To return value we must stat type ex. "-> i32"
    println!("Ku suka dirimu, {name}");
    
    // Statments: Instructions to perform action that does not return a value
    let _y = 6;

    // Expression: Evaluate to resultant value
    _y + 1 // the last line without the ';' will be the return type
}

fn times_two_hundred(x: i32) -> i32 {
    x * 200
}

    // FLOW & LOOPS
fn flow (x: i32) {
    if x < 2 { // condition must be expliticitly a bool
        println!("less than 2");  
    } else if x == 3 {
        println!("it's three");
    } else{
        println!("more or the same as 2");
    }

    let condition = true;
    let number = if condition {5} else {6}; // must be same type

    println!("{}", number);
}

fn looper(mut x: i32) {
    loop {
        println!("i am infinity");
        x = x -1 ;

        if x < 0 {
            break;
        }
    }
}

fn looper2 () {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn looper3() {
    let mut counter = 0;

    'outside_counter: loop { //kita bisa namain loop kita
        let mut inside_counter = 0;
        println!("outer iteration: {}", counter);
        'inside_counter: loop {
            inside_counter += 1;
            
            if inside_counter == 20 {break 'inside_counter;}
            if counter == 30 {break 'outside_counter;} // Terus spesifik break terhadap loop
                                                       // tertentu
        
            println!("{}", inside_counter);
        }

        counter +=1;
    }
}

fn while_looper(mut x: i32) {
    while x!= 0 {
        println!("{x}");

        x -= 1;
    }

    println!("LIFT OFF!!");
}
/*
fn for_looper() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("{}", a[index]);
    
        index += 1
    }
}
*/
    // tapi yang diatas bisa ditulis kaya gini
fn for_looper2() {
    let a = [10, 20, 30, 40, 50];

    for x in a {
        println!("element: {x}")
    }
}
