use std::fmt::Display;
use std::cmp::PartialOrd;

// Building a Trait
pub trait Summary {
    // ???
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("Read more from {}... ", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// Implementing the Trait for a Struct
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
}

pub struct Tweet {
    pub author: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// Again implementing a trait fro the Tweet Struct
// Tweet and NewsArticle share the same struct
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.author, self.content)
    }
}

pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

// Can be used if T is any type
impl<T> Pair<T> {
    pub fn get_x(&self) -> &T {
        &self.x
    }
}

// Only can be use used if T is Display or PartialOrd
impl <T: Display + PartialOrd> Pair <T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("{}", self.x)
        } else {
            println!("{}", self.y)
        }
    }
}


