// ENUMS
#![allow(warnings)]         // Help to ignore warnings

enum MieAyam {              // Enums represents one of multiple possible variants 
    Jawa,                   
    Cina,
    Bangka,
}

enum Bakmi {                // Each variants can contain have their own data
    Jawa(String),
    Bangka(String),
    Cina(String),
}

enum FightOrFriendly {      // Each variants also can have their own data type, in the same Enum
    Punch(i32),
    Dialogue(String), 
}

impl FightOrFriendly {      // Enums also can be implemented with
    fn call(&self) {        // Methods
        match self {        // Biasanya Enums digunakan untuk match
            FightOrFriendly::Punch(Power) => {
                println!("You started a fight by punchig with {} power!", Power);
            }
            FightOrFriendly::Dialogue(text) => {
                println!("Your dialogue: {text}, They don't like it now you are fighting");
            }
        } 
    }
}

fn main() {
    let _mieJaka = MieAyam::Jawa;

    let mieTerEnak = Bakmi::Bangka(String::from("Roxy"));

    match mieTerEnak {
        Bakmi::Bangka(text) => { println!("Aku suka bakmi {}", text)},
        _ => {} // '_' untuk all other variant {} artinya ga ngapa2in
    }

    let GamingExample = vec![
        FightOrFriendly::Dialogue(String::from("Aku Cinta Kamu!")),
        FightOrFriendly::Punch(300),
    ];

    for i in GamingExample {
        i.call();
    }
    option_enum_example();

    match_section();
}

fn option_enum_example() {
    // OPTION ENUM

    // Due to the fact that rust doesn't have a Null Value, we use Option enums
    // Option enums are an item type that indicates whether a thing maybe have a value
    // Below is the option enum that is written in the std library (u don't have to create it)
    /*
    enum Option<T> {
        None,
        Some(T),
    }

    This is way like to represent below
    */

    let x: Option<i32> = Some(32);      // This has a value
    let y: Option<i32> = None;          // This have no value
    
    // While above is completly useless the Option has its usecase like below
  
    let m = [1, 2, 3, 4, 5];
    
    // Function checking if number is.in list
    fn check_number(numbers: &[i32], number: i32) -> Option<i32> {
        if numbers.contains(&number) {
            Some(number)            // return Some(i32)
        } else {
            None                    // return None
        }
    }

    match check_number(&m, 30) {                    // Use match to handle each value
        Some(shit) => println!("Found Number {}", shit),
        None => println!("Nope, not there"),
    };

    // This helps us to handle potentially 'null values' explicitly an not for long

    let x = match check_number(&m, 30) {                    
        Some(shit) => shit,
        None => 0,              // Assign supposedly null into an Int32 
    };

    println!("{x}");
}



fn match_section(){
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {           // Match is about pattern matching 
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => {                      // You also able to run a code block
                println!("wow your rich");
                25},
        }
    }
    let icanos_coin = Coin::Quarter;
    
    println!("ican's coin is worth {} cents", value_in_cents(icanos_coin));
}
