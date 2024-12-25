pub struct Post {
    state: State,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: State::Draft,
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        match self.state {
            State::Published => &self.content,
            _ => "",
        }
    }

    pub fn add_text(&mut self, txt: &str) {
        self.content.push_str(txt)
    }

    pub fn request_review(&mut self) {
        match self.state {
            State::Draft => self.state = State::PendingReview,
            _ => (),
        }
    }

    pub fn approve(&mut self) {
        match self.state {
            State::PendingReview => self.state = State::Published,
            _ => (),
        }
    }
}

enum State {
    Draft,
    PendingReview,
    Published,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_lifecycle() {
        let mut post = Post::new();

        post.add_text("I hurt myself today");
        assert_eq!("", post.content());

        post.request_review();
        assert_eq!("", post.content());

        post.approve();
        assert_eq!("I hurt myself today", post.content());
    }
}
