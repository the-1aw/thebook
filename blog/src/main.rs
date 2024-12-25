use blog::state_pattern;

fn main() {
    let mut post = state_pattern::Post::new();

    post.add_text("I hurt myself today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I hurt myself today", post.content());
}
