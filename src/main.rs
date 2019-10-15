use std::fs::File;

mod feed;
mod upload;

fn main() {
    let out = File::create("feed.xml").unwrap();
    let feed = feed::FeedGenerator {
        title: String::from("Title1"),
        base_url: String::from("http://eg.test"),
    };
    feed.generate_for_dir("test_fixtures/dir1/", out);
}
