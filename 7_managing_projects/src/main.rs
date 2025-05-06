use crate::garden::vegetables::Asparagus;
use rand::Rng;                              // Importing rand and using rng trait
use std::collections::HashMap;              // Even using std library you have to use `use`
// use std::{cmp::Ordering,                    // Nested paths
//          io};
use std::io::{self, Write};                 // it's importing both std::io and std::io::Write
use std::collections::*;                    // Using '*' or the glob operator to bring all public
                                            // items defined in path into this scope
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I am an {plant:?}!");

    let randNum = rand::rng().random_range(1..=100); // The function is `thread_rng()`
    println!("I am an {randNum}");
}

