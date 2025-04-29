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

    // to remove warning
    println!("{}, {}, {}, {}, {}, {}", black.0,black.1,black.2,
             origin.0, origin.1, origin.2);
    
    // UNIT-LIKE STRUCTURE
    let _admin = AlwaysEqual;        // Used for implementing trait (More on Chapter 10) 
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

// NOTES
// OWNERSHIP OF STRUCT DATA
// We used 'String' rather than '&str' because in this case we want each instance to own their own
// data rather than refrencing others. It's also possible to reference other but it requires
// Lifetimes, more on chapter 10


