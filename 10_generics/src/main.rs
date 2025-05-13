mod generics;
use generics::types;
use generics::traits::{Tweet, Summary, NewsArticle, Pair};    
                                                        // to use the trait, we must also bring the
                                                        // trait with the struct
use generics::lifetimes;
use std::fmt::{Display, Debug};


fn main() {
    types::type_example();

    traits_example(); 

    lifetimes::lifetimes_example();
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
    advertisement(&tweet);

    let article2 = NewsArticle {
        headline: String::from("Seulgi Comeback dengan Tune Britney-Pop ala 2000-an dengan Baby not Baby"),
        location: String::from("Seoul, Korea Selatan"),
        author: String::from("Herman"),
        content: String::from(
            "Cara menjadi orang baik?",
            ),
    };

    // Walaupun tetap typenya harus sama
    recommendation(&article, &article2);

    let dg_tweet = return_summarizable();

    notify(&dg_tweet);

    conditional_traits_for_methods();
}

fn conditional_traits_for_methods() {
    let r = Pair { x: 12, y: 10 };

    r.cmp_display();

    let m = Pair { x: String::from("china"), y: String::from("is great") };
    let mx = m.get_x();

    println!("{}", mx);

    m.cmp_display();
}

// Creating a function with a Trait in mind
fn notify(item: &impl Summary) {                    // Only type with 'Summary' as trait can be used
    println!("Breaking News! {}", item.summarize());
}

// '&impl' above^ is syntax sugare for below:
fn advertisement<T: Summary>(item: &T) {
    println!("Check out this article! {}", item.summarize());
}

// you can shorten multiple trait as paramaters using this:
fn recommendation<T: Summary>(item1: &T, item2: &T) {
    println!("You like {}? You'll probably also like writings from {}", item1.summarize_author(),
    item2.summarize_author());
}   // rather than using "item1: &impl summary, item2: &impl summary"

// Using multiple traits for a function
// Fungsi ini hanya menerima item yang memiliki trait summary dan display
fn multiple_traits_ex<T: Summary + Display>(item: &T) {
    println!("20");
}   //atau juga bisa '(item: &(impl Summary + Display))'

// Using multiple Trait with Multiple different paramaters
fn multiple_traits_mul_par<T, U>(t: &T, u: &U)
    where
        T: Display + Summary,
        U: Summary + Debug,
    {
        println!("fungsinya disini");
    }

// return a struct with a Trait 
fn return_summarizable() -> impl Summary {
    Tweet { // typenya harus sudah ada di scope 
        author: String::from("david_gilmour"),
        content: String::from("WISH YOU WERE HEREE... "),
        reply: true,
        retweet: true,
    }
} // namun hanya dapat dilakukan untuk 1 type

