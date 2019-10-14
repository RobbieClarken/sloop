use rss::ChannelBuilder;
use std::io::prelude::*;

pub struct FeedGenerator;

impl FeedGenerator {
    pub fn generate_for_dir<W: Write>(&self, _dir: &str, mut writer: W) {
        let channel = ChannelBuilder::default().build().unwrap();
        channel.pretty_write_to(&mut writer, b' ', 2).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_xml_for_dir() {
        let mut buffer = Vec::new();
        (FeedGenerator {}).generate_for_dir("test_fixtures/dir1/", &mut buffer);
        let feed = String::from_utf8(buffer).unwrap();
        assert!(
            feed.starts_with("<rss"),
            format!("expected feed to start with \"<rss\", got:\n\n{}\n\n", feed)
        );
    }
}
