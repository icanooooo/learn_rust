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
- Then in the *Crate Root* File, we can declare new modules (example, declaring a garden modul with `mod garden;`. The compiler then will look for the modul in this places:
    - Inline, in the curly brackets `{}` that replace the semicolon after declaring `mod garden`. (if written).
    - In the file *src/garden.rs*
    - In the file *src/garden/mod.rs*
- We can also write a ***Submodules***, (example: `mod vegetables;`) in *src/garden.rs*, the compiler will then look it in the
    - Inline, in the curly brackets `{}` that replace the semicolon after declaring `mod vegetables`. (if written).
    - In the file *src/garden/vegetables.rs*
    - In the file *src/garden/vegetables/mod.rs*
- Once the module is part of our crate, we can **get the piece of code we want** anywhere in the same crate (as long privacy rules allow). For example, we want to acces an `Asparagus` type in the garden vegetables module, we can get it using `crate::garden::vegetables::Asparagus`.
- We can use the **`use` keyword** to shorten what we want to refer, rather than using the previous code to refer, we can write it like `use crate::garden::vegetables::Asparagus;` once and then we only need to write `Asparagus` so on.
- To control the privacy of the modules, we can declare it with **`pub mod`** to make it *public*, and **`mod`** to make it privated. Please note the module will follow the privacy of the the parent module by default. To make the items withe the module also public, use the `pub` keyword before the declaration.

Please see code and project for example.

So in the project, using the example above we create an item (`Asparagus`) in the submodule of `vegetables` in the module of `gardes`. Create the file directory as below:

```
project/
├── Cargo.toml
└── src/
    ├──Garden/
    |   └── vegetables.rs       # struct Vegetables {} 
    ├── main.rs                 # running our main
    └── garden.rs               # importing submodule for Vegetables 
```


