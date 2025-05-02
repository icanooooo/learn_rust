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

    if_else_let_section();
 
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

    enum Status {
        Laper,
        Kenyang,
    }

    let ican_status = Status::Laper;

    match ican_status {
        Status::Laper => { println!("Tambah lagi nasinya") },
        Status::Kenyang => { println!("Gabakal lagi gua makan selamanya") },
    }
}

fn if_else_let_section () {                     // Helps us to us match on specific variant values
    // Withouf if let
    
    let config_max = Some(3u8);
    //    EXPRESSION
    match config_max {
    //  PATTERN 
        Some(max) => {println!("The {max}")},
        _ => (),
    }

    // Is equal to (using if let)

    //     PATTERN     EXPRESSION
    if let Some(max) = config_max {             // Helps us to write match on only specific
                                                // variants
        println!("The true {max}");
    }

    // the use of 'else'

    let config_min: Option<u32> = None;

    if let Some(someshit) = config_min {             
       println!("The true {someshit}");
    } else {                                        // You can also use 
        println!("this is the same as using '_'");  // 'else if let PATTERN = VALUE for another'
                                                    // specific variant
    }

    let ican_coin = Coin::Quarter(UsState::Alabama);

    let coin_status = describe_state_quarter(ican_coin); 

    if let Some(result) = coin_status { println!("{}", result); }
}

// FOR LAST EXAMPLE
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),       // You can use an enum inside an enum
}

impl UsState {
    fn existed_in(&self, year: u32) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >=1959,
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    // Ingin menarik UsState dari Coin::Quarter
    let Coin::Quarter(state) = coin else {          //if exist, assign UsState into a 'state'
       return None;                                 //variable in the parenthases 
    };

    /* above is the sama as below
    
    let state = if let Coin::Quarter(state) = coin {
        state
    } else {
        return None
    };

    */

    if state.existed_in(1900) {
        Some(format!("{state:?} is pretty old"))
    } else {
        Some(format!("{state:?} is relativly new"))
    } 
}
