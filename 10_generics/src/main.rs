mod generics;
use generics::types;
use generics::traits::{Tweet, Summary, NewsArticle, notify};    
                                                        // to use the trait, we must also bring the
                                                        // trait with the struct
use generics::lifetimes;

fn main() {
    types::type_example();

    traits_example() 

    // lifetimes::LifetimesExample();
}

fn traits_example() {
    // Using TRAITS
    let tweet = Tweet {
        author: String::from("@icanooo"),
        content: String::from("i like Wish You Were Here by Pink Floyd"),
        reply: false,
        retweet: false,
    };

    println!("{}", tweet.summarize());

    // Using default implementations for TRAITS (summarize is not defined for NewsArticle but
    // implemented Summary trait, using it's default trait)
    let article = NewsArticle {
        headline: String::from("Seulgi Comeback dengan Tune Britney-Pop ala 2000-an dengan Baby not Baby"),
        location: String::from("Seoul, Korea Selatan"),
        author: String::from("Prisqia Hanifa"),
        content: String::from(
            "CANTIK BANGET SEULGI ALALALA LILIILILI",
            ),
    };

    println!("{}", article.summarize());

    notify(&article);
    notify(&tweet);
}
