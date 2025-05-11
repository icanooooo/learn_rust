# When to use `panic!`

- Use it when writing Examples, Prototype Code, and Test.
- Cases in you which have more information than the compiler
- Guidelines for error handling
- Creating custome type for validation

## Summary

Use `panic!` macro to signals that a program is in a state it can't handle. Using `Result` enum should indicate whether if something fails, the code should still continute or recover despite the failure.
