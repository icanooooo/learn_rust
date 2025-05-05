# **Managing Project with Packages, Crates, and Modules**

- **Crates**, a tree of modules that produces a library or executable
- **Packages**, a cargo feature that you build, test, and share crates.
- **Modules** & **Use**, let you control the oranization, scope, and privacy of paths.
- **Paths**, A way of naming an item, such as a struct, function or module.

## **Crates**

The smallest unit of compilation in Rust. There are two kinds:
- **Binary Crates** are programs you can compile to an executable, each binary crates must have a function called `main` to run the executable. So far we have only use these kinds.
- **Library Crates** dont' have `main` functions. They're intended as a support to define functions to be shareds to multiple projects. Most of the time when we talk about *crates* is this one. It's somewhat the same as the general programming concepts of a *library*. 

The *Crate root* is a source file that the Rust compiler starts from and makes up the root module of your crate.

## **Packages**

A bundle of one or more crates that provides a functionality. It could have multiple *binary crates* but at most only have **one** *library crate*. A package contains a *Cargo.toml* file that describes how to build those crates. Cargo is actually a package that contains binary crate for the comman-line tool you've been using to build our code (it's a command line app used for use to build our projects!). Essentially it's a bundle of one or more crates that build our project.

When we use `cargo new a-project`, it builds us a directory for our project. Within that directory and the again within `src` directory, contains our `main.rs`. Cargo recognize this file, in the `src/main.rs`, as their *crate root* we talked about in the previous section. If it's a `lib.rs` file, it will then use it as the *crate root*. It passes each of them or both to rustc to create our binary file.

If there's both of them, cargo will then create for each crate their own *crate root*. If we want to build multiple *Binary Crates* we can create it within the `src/bin` directory:

```
project/
├── Cargo.toml
├── src/
│   ├── lib.rs            # crate root for the library crate
│   └── main.rs           # crate root for default binary crate
└── src/bin/
    ├── tool1.rs          # crate root for binary "tool1"
    └── tool2.rs          # crate root for binary "tool2"
```

## **Modules**

Modules are a way to organize code into namespace, making large program more manageable and avoid naming conflicts. Modules allow you to group related code blocks (Function, Structs, Enums, Constants, etc) together.

### Cheat Sheet

Before we go further, it's good to show how the compiler work and process in a project and how most developers organize their code.

- The compiler first looks in the ***Crate Root*** for the code to compile, either *src/lib.rs* or *src.main.rs*.
- The in the *Crate Root* File, we can declare new modules (example, declaring a garden modul with `mod garden;`. The compiler then will look for the modul in this places:
    - Inline, in the curly brackets `{}` that replace the semicolon after declaring `mod garden`. (if written).
    - In the file *src/garden.rs*
    - In the file *src/garden/mod.rs*

