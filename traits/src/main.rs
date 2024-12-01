use traits::{NewsArticle, Tweet};

fn main() {
    let tweet = Tweet {
        username: "horse_ebooks".to_string(),
        content: "of course, as you probably already know, people".to_string(),
        reply: false,
        retweet: false,
    };
    traits::notify(&tweet);
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };
    traits::notify(&article);
    let pair = traits::conditional::Pair::new(1.0f32, 2.0f32);
    pair.cmp_display();
    // NOTE:
    // Due to trait bound this won't compile since () does not implement std::fmd::Display nor PartialOrd
    // let pair = traits::conditional::Pair::new((), ());
    // pair.cmp_display();
}
