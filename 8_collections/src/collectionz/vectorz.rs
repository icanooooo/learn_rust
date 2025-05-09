use std::fmt;

// pub fn test() -> String {
//     "This is a test".to_string()    
// }

pub fn vectors_example() {
    // VECTORS
    let mut v: Vec<i32> = Vec::new();           // Creating an empty vector that hold i32 type

    v.push(5);                                  // Adding value to a vector
    v.push(6);
    v.pop();                                    // Popping out from a vector

    println!("{}", v[0]);
    let mut v = vec![1, 2, 3, 4];               // Create a vector with a value in it
  
    // Get an Element in a VECTOR
    println!("{}", v[1]);

    let third: &i32 = &v[2];                    // Using a reference to contain it in a variable
    println!("{}", third);

    let third: Option<&i32> = v.get(2);         // Using a get method, returning an enum of a
                                                // referenc if exist
    match third {
        Some(third) => { println!("{}", third) },
        None => { println!("Theres is no value in this") }
    }

    let onehundred: Option<&i32> = v.get(99);

    let onehunny = match onehundred {           // Using match to create a value
        Some(onehundred) => { onehundred },     // Borrows from the 'v'
        None => { &0 },                         // Borrows from a constant literal '0'
    };
    println!("{}", onehunny);

    let first = & v[0];                         
    println!("{}",  first);
                                                // Mutability and reference same as anyother
                                                // variable
    v.push(5);                                  // variable 'first' ends here

    for i in &mut v {                           // iterating over a vector 
        *i += 1;                                // Using the derefrence operator to mutate each
                                                // value
        println!("{i}");
    }



    // Because of Vectors only can contain the same type, we use enums to solve this
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    // Enabling the enum 'Spreadshell', to be printed. more on this later. 
    impl fmt::Display for SpreadsheetCell {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                SpreadsheetCell::Int(val) => write!(f, "{}", val),
                SpreadsheetCell::Float(val) => write!(f, "{}", val),
                SpreadsheetCell::Text(val) => write!(f, "{}", val),
            }
        }
    }

    // a vector of a SpreadsheetCell enum type
    let row = vec![
        SpreadsheetCell::Int(28),
        SpreadsheetCell::Text(String::from("Halo Aku Ican")),
        SpreadsheetCell::Float(14.1),
    ];

    for i in &row {
        println!("{i}");
    }
    // Like any struct, a vector are freed when goes out of scope
}
