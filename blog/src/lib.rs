pub mod enums;
pub mod state_pattern;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ensure_compatibility() {
        let mut e_post = enums::Post::new();
        let mut sp_post = state_pattern::Post::new();

        e_post.add_text("I hurt myself today");
        sp_post.add_text("I hurt myself today");
        assert_eq!(e_post.content(), sp_post.content());

        e_post.request_review();
        sp_post.request_review();
        assert_eq!(e_post.content(), sp_post.content());

        e_post.reject();
        sp_post.reject();
        assert_eq!(e_post.content(), sp_post.content());

        e_post.request_review();
        sp_post.request_review();
        assert_eq!(e_post.content(), sp_post.content());

        e_post.approve();
        sp_post.approve();
        assert_eq!(e_post.content(), sp_post.content());
    }
}
