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

    borrow();

    let s = no_dangle();
    println!("{s}");

    let r = first_char(&s);
    println!("{r}");

    let string_new = String::from("Let's go to the mall today");
    let a = slices(&string_new);
    println!("{a}");

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

fn borrow() {
    let r = String::from("Kau ambil hatiku!");
    
    let r_size = calculate_length(&r); // using 'r' as a reference, it allows to borrow the value
                                       // without taking ownership of it
    println!("The size of '{r}' is {r_size}");

    let mut lirik = String::from("Tapi aku tak pernah mati, tak akan");

    finish_lirik(&mut lirik); //you can change the variable by '&mut', which means a mutable
                              //reference, the borrower can mutate it
    println!("{lirik}");
                              //But having multiple mutable references to one values is a no no
    /*
    let mut s = String::from("test");
    let r = &mut s;
    let x = &mut s; // this is a nono
    */

    // but u can borrow multiple times if it's not mutable
    
    let mut kendrick = String::from("MUSTRAARRD");

    let duckworth = &kendrick;
    let lamar = &kendrick;
    // let mut diddy = &mut kendrick; //ini gabisa karena mutable reference gabisa sharing

    println!("{kendrick} {lamar} {duckworth}"); //gini bisa
                                                //                                               //
    kendrick.push_str("dj"); // Ketika sudah dimutate 'duckworth' and 'lamar' udah gabisa diborrow
                             // lagi. Karena sudah di immutable borow sama argument `push_str`.

    println!("{kendrick}"); // jika menggunakan 'duckworth' and 'lamar', akan error. 

    let jcole = &kendrick;

    println!("{jcole}"); // 'jcole' bisa digunakan karena mutable reference sudah dibuang karena
                         // diluar scope functionnya.
    
    finish_lirik(&mut kendrick); // disini 'jcole' berhenti bisa digunakan karena 'kendrick'
                                 // diborrow dengan mutable reference
    println!("{kendrick}");

    // So in short we can use multiple reference towards a variable, but we reference it with
    // mutability the previous reference cannot be used again.
    // It's okay to mutable reference if it's in different scope, because once the scope is
    // finished it will be dropped
}


fn calculate_length(s: &String) -> usize { // Rather than taking the ownership 's' borrows it
    s.len()// return the length of the string
}

fn finish_lirik(s: &mut String) { // Borrowing the arguments and mutate it need the '&mut'
                                  // statement
    s.push_str(" berhenti");    
}

// Dangling reference is a reference of a memory that is not used anymores
// so if you returning something from a function don't let it be a reference if the variable is
// dropped out of the scope
//
// it's better to return directly the Variable

fn no_dangle() -> String{
    String::from("hellow") // don't return a reference except the variable of the reference is
                           // still ther
}

fn first_char(s: &String) -> &str { //this is okay as because it points to a variable that is still
                                    //exis from out of this scope
                                    //this is a slice
    &s[0..1]
}

// Slices are references that borrow a portion of a collection without taking ownership,
// Internally a pointer to the start and length
fn slices(s: &String) -> &str { // This is a slice
    &s[1..5] // this is okay to return because it points to an address
             // starting at position number 1 with 5 length
} 
