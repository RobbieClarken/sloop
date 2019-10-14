use rss::extension::itunes::{ITunesChannelExtensionBuilder, NAMESPACE};
use rss::ChannelBuilder;
use std::collections::HashMap;
use std::io::prelude::*;

pub struct FeedGenerator;

impl FeedGenerator {
    pub fn generate_for_dir<W: Write>(&self, _dir: &str, mut writer: W) {
        let namespaces: HashMap<String, String> = [("itunes".to_string(), NAMESPACE.to_string())]
            .iter()
            .cloned()
            .collect();
        let itunes_ext = ITunesChannelExtensionBuilder::default()
            .block("Yes".to_string())
            .build()
            .unwrap();
        let channel = ChannelBuilder::default()
            .namespaces(namespaces)
            .itunes_ext(itunes_ext)
            .build()
            .unwrap();
        channel.pretty_write_to(&mut writer, b' ', 2).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[macro_export]
    macro_rules! assert_contains {
        ($haystack:expr, $needle:expr) => {{
            assert!(
                $haystack.contains($needle),
                format!(
                    "expected string to contain \"{}\", got:\n\n{}\n\n",
                    $needle, $haystack
                )
            );
        }};
    }

    #[test]
    fn generates_xml_for_dir() {
        let mut buffer = Vec::new();
        (FeedGenerator {}).generate_for_dir("test_fixtures/dir1/", &mut buffer);
        let feed = String::from_utf8(buffer).unwrap();
        assert_contains!(feed, "xmlns:itunes");
        assert_contains!(feed, "<itunes:block>Yes</itunes:block>");
    }
}
