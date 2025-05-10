use std::fs::File;                          // Module for opening file, will return a result enum
use std::io::ErrorKind;

pub fn result_example() {
    let greeting_file_result = File::open("lesgo.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            },
        }
    };
}
