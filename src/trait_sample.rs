struct Post {
    title: String,
    content: String,
}

impl Post {
    fn new(title: String, content: String) -> Self {
        Self {
            title,
            content
        }
    }
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("{}\n{}", self.title, self.content)
    }
}

trait Summary {
    fn summarize(&self) -> String;
}

fn echo_summary<T: Summary>(article: &T) {
    let summary = article.summarize();
    println!("{}", summary);
}

fn main() {
    let post = Post::new("title".to_owned(), "sample".to_owned());
    echo_summary(&post);
}
