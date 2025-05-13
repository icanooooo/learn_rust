use std::fmt::Display;

// Lifetimes can also be use for structs
struct importantExcerpt<'a> {
    part: &'a str, // indicating the section will live as long as the variable to the reference
                   // will live    
}

// Creating a methoed that implement lifetimes
impl <'a> importantExcerpt<'a> {
    fn announce_excerpt (&self) {
        println!("Annoucement: {}", self.part);
    }
}

// using STATIC
static NAMA: &str = "Muhammad Ihsan";
// this will live as long as the the program lives

pub fn lifetimes_example() {
    let x = String::from("shakedown 1979");
    let y = String::from("coolkids never have the time");

    println!("{}", largest_string(&x, &y));

    // Even though struct is defined outside of the scope, it's availability of is only within the
    // scope it's declared
    {
        let r = String::from("I won't scatter your sorrow to the heartless sea. I will always be with you.");
        let excerpt = r.split('.').next().unwrap();

        let important_sentence = importantExcerpt {
            part: excerpt,
        };

        println!("{}", important_sentence.part);
    
        important_sentence.announce_excerpt();
    }

    // important_sentence is not available anymore

    let a; // declare a variable to clear
    {
        let m = String::from("thank god");

        let (temp_a, b) = multiple_lifetimes_example(&x, &m);
    
        a = temp_a; // Change ownership to a variable already declared
    }

    println!("{} still exist here", a);     // we can't do this with b becaus2e lifetime is only
                                            // within the same scope with 'm'

    println!("{}", NAMA);

    last_function("nama saya muhammad ihsan", "saya bangga dengan sidik", 3);
}

// THE LIFETIME PARAMATER ('a)
// The symbol indicates that the reference will live as long as the most short-lived parameter
fn largest_string<'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }

    // Because the function can return either x or y, the compiler must ensure that both
    // references live long enough for the return value. By using the lifetime parameter 
    // 'a, we tell the compiler that the return reference must be valid as the most short-lived
    // parameter we put it into.
    //
    // In short: both x and y (references) will live as long as their most short-lived parameter. 
}

fn multiple_lifetimes_example<'a, 'b> (inner: &'a str, outer: &'b str) -> (&'a str, &'b str) {
    (inner, outer)
    // In this case, we have multiple lifetime annotations a', b'. It means each return value will
    // live as long as their each parameter lifetimes
}

// All concepts written together
fn last_function<'a, T> (
    x: &'a str,
    y: &'a str,
    z: T,
)
where 
    T: Display,
{
    if x.len() > y.len() {
        println!("Announcement: {}", x);
    } else {
        println!("Announcement: {}", y);
    }
}

