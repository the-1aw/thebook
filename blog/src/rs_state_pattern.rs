pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
    nb_approval: usize,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, txt: &str) {
        self.content.push_str(txt);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            nb_approval: 0,
        }
    }
}

pub enum Approval {
    Confirmed(Post),
    Pending(PendingReviewPost),
}

impl PendingReviewPost {
    pub fn approve(mut self) -> Approval {
        self.nb_approval += 1;
        if self.nb_approval < 2 {
            Approval::Pending(PendingReviewPost {
                content: self.content,
                nb_approval: self.nb_approval,
            })
        } else {
            Approval::Confirmed(Post {
                content: self.content,
            })
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
