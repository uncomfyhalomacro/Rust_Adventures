use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

// impl Summary for NewsArticle {
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }
impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// Trait as Parameters
// pub fn notify(item: &impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// Trait bound syntax
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

// Specifying Multiple Trait Bounds with the +Syntax

pub fn notify1(item: &(impl Summary + Display)) {}

pub fn notify2<T: Summary + Display>(item: &T) {}

// Clearer Trait Bounds with where Clauses
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    10
}

fn return_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably know, people"),
        reply: false,
        retweet: false,
    }
}
