use rss::extension::itunes::{ITunesChannelExtensionBuilder, NAMESPACE};
use rss::{ChannelBuilder, EnclosureBuilder, Item, ItemBuilder};
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use chrono::Utc;

pub struct FeedGenerator {
    pub base_url: String,
}

impl FeedGenerator {
    pub fn generate_for_dir<W: Write>(&self, files_dir: &str, mut writer: W) {
        let namespaces: HashMap<String, String> = [("itunes".to_string(), NAMESPACE.to_string())]
            .iter()
            .cloned()
            .collect();
        let itunes_ext = ITunesChannelExtensionBuilder::default()
            .block("Yes".to_string())
            .build()
            .unwrap();
        let mut items: Vec<Item> = Default::default();
        let paths = fs::read_dir(files_dir).unwrap();
        let date = Utc::today().and_hms(0, 0, 0); // TODO: change for each file
        for path in paths {
            let p = path.unwrap().path();
            let file_name = p.file_name().unwrap().to_str().unwrap();
            let file_stem = p.file_stem().unwrap().to_str().unwrap();
            let file_meta = std::fs::metadata(&p).unwrap();
            let enclosure = EnclosureBuilder::default()
                .url(format!("{}/{}", self.base_url, file_name))
                .mime_type("audio/mpeg") // TODO: handle other mime types
                .length(file_meta.len().to_string())
                .build()
                .unwrap();
            let item = ItemBuilder::default()
                .title(Some(String::from(file_stem)))
                .enclosure(Some(enclosure))
                .build()
                .unwrap();
            items.push(item);
        }
        let channel = ChannelBuilder::default()
            .namespaces(namespaces)
            .itunes_ext(itunes_ext)
            .items(items)
            .pub_date(date.to_rfc2822())
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
        let generator = FeedGenerator {
            base_url: "https://eg.test".to_owned(),
        };
        generator.generate_for_dir("test_fixtures/dir1/", &mut buffer);
        let feed = String::from_utf8(buffer).unwrap();
        assert_contains!(feed, "xmlns:itunes");
        assert_contains!(feed, "<itunes:block>Yes</itunes:block>");
        assert_contains!(
            feed,
            "url=\"https://eg.test/file1.mp3\" length=\"6\" type=\"audio/mpeg\""
        );
        assert_contains!(feed, "<title>file1</title>");
        let pub_date = Utc::today().and_hms(0, 0, 0).to_rfc2822();
        assert_contains!(feed, format!("<pubDate>{}</pubDate>", pub_date).as_str());
    }
}
