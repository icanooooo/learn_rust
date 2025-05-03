# **Managing Project with Packages, Crates, and Modules**

- **Crates**, a tree of modules that produces a library or executable
- **Packages**, a cargo feature that you build, test, and share crates.
- **Modules** & **Use**, let you control the oranization, scope, and privacy of paths.
- **Paths**, A way of naming an item, suca as a struct, function or module.

## **Crates**

The smallest unit of compilation in Rust. There are two kinds:
- **Binary Crates** are programs you can compile to an executable, each binary crates must have a function called `main` to run the executable. So far we have only use these kinds.
- **Library Crates** dont' have `main` functions. They're intended as a support to define functions to be shareds to multiple projects. Most of the time when we talk about *crates* is this one. It's somewhat the same as the general programming concepts of a *library*. 
