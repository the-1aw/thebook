use blog::rs_state_pattern;

fn main() {
    let mut post = rs_state_pattern::Post::new();

    post.add_text("I hurt myself today");

    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I hurt myself today", post.content());
}
