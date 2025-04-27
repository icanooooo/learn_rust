fn main() {
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
}
