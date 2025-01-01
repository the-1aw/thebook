pub mod advanced;
pub mod conditional;

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

#[derive(Debug)]
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.clone()
    }
}

#[derive(Debug)]
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

    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn notify_syntax_sugar(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn notify_with_where<T>(item: &T)
where
    T: Summary + std::fmt::Debug,
{
    notify_syntax_sugar(item);
}

pub fn notify<T: Summary + std::fmt::Debug>(item: &T) {
    notify_with_where(item)
}
