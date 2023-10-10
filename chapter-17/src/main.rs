use chapter_17::Post;

fn main() {
    let mut post = Post::new(); // Post now in Draft

//  Ver 1 - with State
    post.add_text("I ate a salad for breakfast. "); // Post still in Draft
    assert_eq!("", post.content());

    post.request_review(); // Post now in PendingReview
    assert_eq!("", post.content());

    post.reject(); // Post now in Draft
    assert_eq!("", post.content());

    post.add_text("I also had coffee. ");
    assert_eq!("", post.content());

    post.request_review(); // Post now in PendingReview
    assert_eq!("", post.content());

    post.approve(); // Post now in NearPublished
    assert_eq!("", post.content());

    post.add_text("Finally, I ate a human child. "); 

    post.approve(); // Post now in Published
    assert_eq!("I ate a salad for breakfast. I also had coffee. Finally, I ate a human child. ", post.content())

//  Ver 2 - with types
    // post.add_text("I ate a salad for lunch today"); 

    // let post = post.request_review(); // Different types
    // let post = post.approve(); // Different types

    // assert_eq!("I ate a salad for lunch today", post.content()); // Only Post has the content method
}
