use std::fmt::Display;

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, announcement: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", announcement);
    longest(x, y)
}

struct LifetimeGeneric<'a> {
    slice: &'a str,
}

impl<'a> LifetimeGeneric<'a> {
    fn level(&self) -> i32 {
        3
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");
    let result = longest_with_announcement(
        string1.as_str(),
        string2.as_str(),
        "I am required to have the display trait",
    );
    println!("The longest string is {result}");
    let lg = LifetimeGeneric {
        slice: string2.as_str(),
    };
    println!("lg contains slice {}", lg.slice);
    println!("lg is level {}", lg.level());
}
