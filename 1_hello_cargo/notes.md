# Notes

So cargo is a project is rust's build system and package manager. We use this to manage our project. U will use this most of the time when you're learning rust. To start, run the following:

```cargo new nama_project```

it will create a new directory where you'll write your project. It also initialize git if you're not already. Inside the project directory you'll see a file `Cargo.toml` and a directory `src`.

The `Cargo.toml` is the Cargo configuration. The first line `[package]` are statements to configure our package. the last line `[dependencies]` is the start of a section for you to list any of the project dependencies. In Rust, Packages of code are reffered as `crates`, which we will further go in depth in `Chapter 2`.

The `src` directory is the directory where we will put our source file. The top level directory is mostly for configuration and readme files, nothing related to your code.

## Building and Running a Cargo Project

to build:

```cargo build```

this would compile the `main.rs` inside of our `src` directory and outputs an executable in the `./target/debug/nama_project`. We can run this directly, but cargo provide a way for us to compiled our code and directly run it using:

```cargo run```

it is most convenient to use this. But if u don't want to run it yet just check periodically we can use:

```cargo check```
