use std::fs::{self, File};                          // Module for opening file, will return a result enum
use std::io::{self, Read, ErrorKind};

pub fn result_example() {
    let greeting_file_result = File::open("lesgo.txt");         // This module will return a result
                                                                // enum. With either T (returned if
                                                                // Ok, and E represent type of
                                                                // error.
    // Propagating Example
    let username = read_username_from_file();
    match username {
        Ok(name) => println!("{}", name),
        Err(e) => panic!("Nope can do: {e:?}"),
    }
    
    // Propagating using '?' example
    let username = read_username_from_file2();
    match username {
        Ok(name) => println!("{}", name),
        Err(e) => panic!("Nope can do: {e:?}"),
    }
    // Chaining '?'
    let username = read_username_from_file3();
    match username {
        Ok(name) => println!("{}", name),
        Err(e) => panic!("Nope can do: {e:?}"),
    }
    // using fs::read_to_string
    let username = read_username_from_file4();
    match username {
        Ok(name) => println!("{}", name),
        Err(e) => panic!("Nope can do: {e:?}"),
    }

    // SHORCUT USING UNWRAP & EXPECT
    // let greeting_file = File::open("tylersssecretalbum.txt").unwrap();       // using .unwrap()
    let greeting_file = File::open("tylersssecretalbum.txt")
        .expect("there is no secret album");                                // using .expect()

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {          // File::open alway return an 'Err' variant.
                                                    // We can call '.kind()' that return will
                                                    // an enum of 'ErrorKind' which have multiple
                                                    // different kinds of errors.
            // We use 'ErrorKind::NotFound' in this match case
            ErrorKind::NotFound => match File::create("hello.txt") { // Creating a file if 
                Ok(fc) => fc,                                        // 'NotFound'
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            // We can use any place holder for other errors
            anyplaceholderforerror => {
                panic!("Problem opening the file: {anyplaceholderforerror:?}");
            },
        }
    };
}

// PROPAGATING ERRORS
// Rather than handling the error inside the function, we should return the error itself
// (Propagating the error)
fn read_username_from_file() -> Result<String, io::Error> {     // Return a result enum of either
    // ^function returning username written in file             // string or error
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {        // Getting the file
        Ok(file) => file,
        Err(e) => return Err(e),                                // username_file will be error
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {         // write file to into username 
        Ok(_) => Ok(username),                                  // return content of the file
        Err(e) => Err(e),                                       // return Err(e)
    }
}

// Short version using '?'
// same as above function but shorter using '?'
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;           // will return Err if result is Err
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;               // same as above
    Ok(username)
}

// Chaining '?' for shorter write of above
fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// Using fs::read_to_string to make it shorter
fn read_username_from_file4() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

// The '?' can be used when a scope return value is Result<T, E> and Option, in Option<T> it will
// return a None variant early if value is None.
//
// You can use it in main by 'fn main() -> Result<(), Box<dyn Error>> { ..code block ..}'
// More on Box<dyn Error> later
