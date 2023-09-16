pub struct Post {
    state: Option<Box<dyn State>>,
    content: String
}

impl Post {
    pub fn new() -> Post {
        return Post {
            state: Some(Box::new(Draft {})),
            content: String::new()
        }
    }

    // This should only add text when in Draft mode - rely on trait object
    pub fn add_text(&mut self, text: &str) {
        // let text_to_push = todo!();
        // self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        return self.state.as_ref().unwrap().content(self);
    }

    // Requsting a review changes the Post state
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() { //take() is to consume the Option
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

// Behavior SIGNATURE for all possible states a Post must have 
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str { // Default implementaiton of content() for this trait. That means that only the Published state will have to override it. 
        return "";
    }

    // Adding a reject() method to change PendingReview back to Draft
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn add_text(self: Box<Self>, text: &str) {
        // self.content.push_str(text);
    }
} 

// TODO  Fix grammar at end of first par here: https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html#defining-post-and-creating-a-new-instance-in-the-draft-state

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        return self; 
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        return self;
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        return Box::new(NearPublished {});
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        return Box::new(Draft {});
    }
}

struct NearPublished {}

impl State for NearPublished {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        return Box::new(Published {});
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        return Box::new(Draft {});
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        return &post.content;
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        return self;
    }
}
