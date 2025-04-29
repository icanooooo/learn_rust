struct User {
        active: bool,                               // This is a struct
        username: String,                           // A collection of related data
        email: String,                              // Building a key:value pair
        sign_in_count: u64,
}

struct Color(i32, i32, i32);    // you can also create tuple structs, without name fields
struct Point(i32, i32, i32);    // each struct we define is its own type

struct AlwaysEqual;     // Unit-like struct, a struct without any fields

fn main() {
    // NORMAL STRUCTS
    let ican_user = build_user("muhihsan0.jkt@gmail.com".to_string(), "icanooo".to_string());

    println!("{}", ican_user.username);          // How to access the values
    println!("{}", ican_user.active);            // instance.key
    println!("{}", ican_user.sign_in_count);

    let mut abe = build_user(ican_user.email, "abe".to_string()); // creating an instance
                                                                  // from another instance
                                                                  // Tadinya emailnya sama 
    println!("email abe: {}", abe.email);

    abe.email = String::from("abe.tungtung@sahur.com");       // how to change value in an
                                                              // instance
    println!("email abe: {}", abe.email);

    let joppy = User {
        username: String::from("hubertdelacrose"),
        ..abe // Taking in values other than ^ from another instance
    };

    println!("email joppy: {}", joppy.email);

    // TUPLE STRUCTS
    let black = Color(0,0,0);       // Each is their own type
    let origin = Point(0, 0, 0);

    println!("{}, {}, {}, {}, {}, {}", black.0,black.1,black.2,
             origin.0, origin.1, origin.2);
    
    // UNIT-LIKE STRUCTURE
    let _admin = AlwaysEqual;        // Used for implementing trait (More on Chapter 10) 

    // EXAMPLE PROGRAM USING STRUCTS
    example()
}

// BUILDING A STRUCT BUILDER FUNCTION 
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// EXAMPLE SECTION

#[derive(Debug)]            // Deriving trait from debug, enable us to use rect:? and rect:#?
struct Rectangle {          // The Rectangle Struct have the Trait now
    height: u32,
    width: u32,
}

// USING METHODS

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height

    }

    fn can_hold(&self, other: &Rectangle) -> bool {     // Borrowing from other rectangles
        self.width > other.width && self.height > other.height
    }
}           // we can have multiple impl for one Struct, the use case for that is in chapter 10

fn example() {
    {
        let width1 = 30;            // Rather than doing this, we could use tuples 
        let height1 = 50;

        println!("the area is {}",area(width1, height1));
    }
    
    // REFACTORING WITH TUPLES
    let rect = (30, 50);                        // Much more readble
    println!("the area is {}", area2(rect));    // using tuples directly to the function 

    // REFACTORING WITH STRUCTS
    let rect = Rectangle { height: 30, width: 50 }; // More understandable  
    println!("the area is {}", area3(&rect)); 

    // DERIVING TRAIT FROM DEBUG
    println!("{rect:#?}");      // We ara able to use rect:? and rect:#? for debugging
    dbg!(&rect);                // and we are able to use dbg!
                                // There are number of traits we can derive from

    println!("the area is {}", rect.area());    // Implementing methods given to the struct

    let rect2 = Rectangle { height: 40, width: 70 };

    println!("Can rect2 hold rect? {}", rect2.can_hold(&rect));

}

fn area(width: u32, height:u32) -> u32{
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32{    // more readable using tuple structs
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {    // much more readable labels
    rectangle.width * rectangle.height
}

// NOTES
// OWNERSHIP OF STRUCT DATA
// We used 'String' rather than '&str' because in this case we want each instance to own their own
// data rather than refrencing others. It's also possible to reference other but it requires
// Lifetimes, more on chapter 10


