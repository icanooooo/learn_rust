mod error_handling;
use error_handling::panic;
use error_handling::result;

fn main() {
    // Unrecoverable Errors with Panic!
    // panic::panic_example();

    // Recoverable Errors with Result!
    result::result_example();
}
