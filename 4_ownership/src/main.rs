use std::io;

fn main() {
    {   // s i not yet valid because it haven't been declared yet
        let s = "hello"; // s is valid onwards
                         // s is a literal, hardcoded as binary in the execution file
        println!("well, {} there, the S var is still valid", s);
    }   // s is now valid no more

    let _s = String::from("this is immutable"); //This is a string not a literal
                                               //Stored in the heap, release memory when it's out
                                               //of scope
                                               //Good for unknown sizes, like API Call result that
                                               //you don't want to mutate
    println!("who's your crush?");
    let mut crush_name = String::new();

    io::stdin()
        .read_line(&mut crush_name)
        .expect("failed to read line");

    let mut hi = String::from("Hello, ");   // Mutable string type
                                            // stored in the heap, will release memory when it's
                                            // out of scope
    hi.push_str(&crush_name);

    println!("{hi}");
      
    let s1 = String::from("hello");
    let s2 = s1; // s2 takes ownership of the s1 pointer
                 // no copying data from the heap, cleans up the heap from s1

    println!("{s2}");

    let mut erk = String::from("tapi, aku, tak pernah mati");
    println!("{erk}");
    erk = String::from("Kusuka dirimu!");

    println!("{erk}"); // The first value dropped, freeing up the memory in heap

    let s3 = s2.clone(); // if you want to copy from the stack
    println!("{s2} ada, dan {s3} juga ada");

    let x = 5;
    let y = x; // y is making a copy of 'x' directly because it's stored in the stack

    println!("{x} ada, dan {y} juga ada");

    let s = String::from("hello");
    nice_function(s.clone()); // must write .clone() after variable ownership was moved from 's' to
                              // some_string in the function
                              // if not 's' is lost forever

    println!("{s}");

    let n: u32 = 28;
    nicer_function(n);      // no need to use copy because it's in the stack

    println!("{n}");

    let a = gives_ownership();
    println!("{a}");
    
    let b = give_n_take(a); // a doesn't own it anymore
                            // "I love you" string ownership:
                            // gives_ownership() -> a -> give_n_take(a) -> b
    println!("{b}");

    let (_c, _c_length) = get_string_n_lenght(b);

}   // every variable in main() is dropped

fn nice_function(some_string: String) { // Borrowed the string
    println!("{some_string}"); 
} //borrowed string will be lost forever if not cloned

fn nicer_function(some_number: u32) {
    println!("{some_number}");
}

// Something returned also gives the ownership
fn gives_ownership() -> String {
    let some_string = String::from("I love you");

    some_string
}
fn give_n_take(some_string: String) -> String {
    some_string
}

fn get_string_n_lenght(s: String) -> (String, usize) {
    let length = s.len(); // returns length of a string

    (s, length)
}
