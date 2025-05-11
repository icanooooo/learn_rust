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

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
    fn summarize(&self) -> String {
        format!("{}: {}", self.author, self.content)
    }
}

pub fn notify(media: &impl Summary) {
    println!("Breaking news! {}", media.summarize());
}
