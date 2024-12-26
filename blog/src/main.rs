use blog::rs_state_pattern::{Approval, Post};

fn main() {
    let mut post = Post::new();

    post.add_text("I hurt myself today");

    let post = post.request_review();

    let mut post = post.approve();
    while let Approval::Pending(pending) = post {
        post = pending.approve();
    }

    assert!(matches!(post, Approval::Confirmed(_)));
    match post {
        Approval::Confirmed(post) => assert_eq!("I hurt myself today", post.content()),
        _ => (),
    };
}
