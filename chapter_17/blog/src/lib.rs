pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingReviewPost {
    content: String,
    number_of_approves: i32,
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
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            number_of_approves: 0,
        }
    }
}

impl PendingReviewPost {
    pub fn approve(mut self) -> Post {
        self.number_of_approves += 1;
        if self.number_of_approves == 2 {
                Post {
                    content: self.content,
                }
        } else {
            return self
        }
    }
}