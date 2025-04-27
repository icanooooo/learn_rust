# Notess

```use std::io;```

to code above lets us to import the `io`, the input/output library into this code. The `io` library comes from the standard library know as `std` library.

```let mut guess = String::new();```

we use the `let` statement to decleare a variable, in this case the `guess` variable. In rust there's two variables are immutable by default, therefore once we declared the variable, the value can't change (immutable). To make variable be able to change, or mutable, we add the `mut` statement. In the otherside of the declaration we use the `String::new` that return a new instance of `String`. to this point we just declared the variable, we havent taken the input yet.


```
io::stdin()
    .read_line(&mut guess)
    .expect("if failed output");

```

Then we use `io::stdin()` from the `io` library to take the 'stdin()' function. Then we use the `read_line(&mut guess)`to take the input, by reading the line (`read_line()`) assigning it to the `guess` variable`. In using rust the `&` indindicates a reference to another part of our code, rather than copy that data multiple times. References are also immutable by default, therefore rather than passing on the `&guess` we pass the argument as `&mut guess` to make it still mutable. Lastly, we use the `expect()` to help us handle potential failures.

The `read_line` function puts whatever string the user enters into a string we pass to it. But not only that, it also return a `Result` value. `Result` is an enumaration, often called `enum` which is a type that can be in multiple possible states, we call each possible state a *variant.*. This will be discussed more in *Chapter 6*.

The aim of the `Result` types is to help error-handling by providing its variant of `Ok` and `Err`. The `Ok` mean it's succesful and contains the succesfully generated value. The `Err` means operation failed and it contain information about how the operation failed. The code will still compiled if there's no `expect` but Cargo will warn you. The right way to surpress the warning is ackhtually writing error-handling code, but in this we just want to crash the program if it fails therefore we use `expect`.

```
println!("you guess: {}", guess);
```

In this instance we use the value of the guess and put it in the string using the place holder of `{}` and the guess, but we could actually could write as `"you guessed: {guess}". But remember you can't do the operations inside of the pinchers, you have to do it like this:

```
println!("{}", x+y);
```

Let's make it more fun making it a guessing random number game. To do this we must use the `rand` crate to generate a random number. To do this, we have to modified the the `Cargo.toml` to add in the `[dependencies]` section. 

*Write about Cargo.lock and its command later*

To use generate the random number we create the variable of `secret_number`. Because the number will not change we will set it as an immutable variable, creating it without the `mut` statement. 

First we add the `use rand::Rng;`. The `Rng` trait defines methods that random numbers implement, and this trait must be in scope to use those methods. *Further reading chapter 3*.

Next we use the `rand` library to use the `rng()` function, with the `random_range(start..=end)` to create number random number between 1 and 100.

After also have to transform our input into a number, therefore we declare it by using `u32`, an unnasigned 32-bit number. There are many more such `i32`, `i64`, etc. We decleare by using the `.trim()` method to remove any white spaces, use `.parse()` to change it into another data type, and lastly use `expect()` to help use handle the error.

After getting our random number and changing our `guess` data type into a number, we then can compared it using `cmp`. Again we need to import it using our standard library `std::cmp::Ordering;`. The `cmp` method helps us to compare two number that return an `Ordering` enum, and then use the `match` experession to get the output of each `Ordering` variant.

We wrap this in a loop like below:

```
loop {
    // getting input, parse guess, compare

}
```

And then add a `break` statement based on which output that we want to stop at. In this case exactly right.
