use chrono::{Duration, Utc};
use percent_encoding::{utf8_percent_encode, AsciiSet, NON_ALPHANUMERIC};
use rss::extension::itunes::{ITunesChannelExtensionBuilder, NAMESPACE};
use rss::{ChannelBuilder, EnclosureBuilder, Item, ItemBuilder};
use std::collections::HashMap;
use std::io::prelude::*;
use std::io::Error;
use std::path::PathBuf;

const ESCAPE_CHAR_SET: &AsciiSet = &NON_ALPHANUMERIC.remove(b'.').remove(b'_');

pub trait MediaFileLike {
    fn name(&self) -> &str;
    fn stem(&self) -> &str;
    fn extension(&self) -> &str;
    fn len(&self) -> Result<u64, Error>;
}

pub struct MediaFile<'a> {
    pub path: &'a PathBuf,
}

impl<'a> MediaFileLike for MediaFile<'a> {
    fn name(&self) -> &str {
        self.path.file_name().unwrap().to_str().unwrap()
    }

    fn stem(&self) -> &str {
        self.path.file_stem().unwrap().to_str().unwrap()
    }

    fn extension(&self) -> &str {
        self.path.extension().unwrap().to_str().unwrap()
    }

    fn len(&self) -> Result<u64, Error> {
        Ok(std::fs::metadata(&self.path)?.len())
    }
}

pub struct FeedGenerator {
    pub title: String,
    pub base_url: String,
}

impl FeedGenerator {
    pub fn generate_for_files<W: Write, M: MediaFileLike>(
        &self,
        files: Vec<M>,
        mut writer: W,
    ) -> Result<(), Error> {
        let namespaces: HashMap<String, String> = [("itunes".to_string(), NAMESPACE.to_string())]
            .iter()
            .cloned()
            .collect();
        let itunes_ext = ITunesChannelExtensionBuilder::default()
            .block("Yes".to_string())
            .build()
            .unwrap();
        let mut items: Vec<Item> = Default::default();
        let today = Utc::today().and_hms(0, 0, 0);
        for (i, file) in files.iter().enumerate() {
            let pub_date = (today - Duration::days(i as i64)).to_rfc2822();
            let escaped_name = utf8_percent_encode(file.name(), ESCAPE_CHAR_SET);
            let enclosure = EnclosureBuilder::default()
                .url(format!("{}/{}", self.base_url, escaped_name))
                .mime_type(FeedGenerator::mime_type(file.extension()))
                .length(file.len()?.to_string())
                .build()
                .unwrap();
            let item = ItemBuilder::default()
                .title(Some(file.stem().replace("_", " ").to_owned()))
                .enclosure(Some(enclosure))
                .pub_date(pub_date)
                .build()
                .unwrap();
            items.push(item);
        }
        let channel = ChannelBuilder::default()
            .namespaces(namespaces)
            .title(self.title.clone())
            .itunes_ext(itunes_ext)
            .items(items)
            .build()
            .unwrap();
        channel.pretty_write_to(&mut writer, b' ', 2).unwrap();
        Ok(())
    }

    fn mime_type(extension: &str) -> String {
        match extension {
            "aac" => "audio/aac".to_owned(),
            "m4a" => "audio/mp4".to_owned(),
            "mp3" => "audio/mpeg".to_owned(),
            "mp4" => "audio/mp4".to_owned(),
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use roxmltree::{Document, Node};
    use std::path::Path;

    fn get_child_node_text<'a>(parent: &'a Node, child_tag: &str) -> &'a str {
        parent
            .descendants()
            .find(|n| n.tag_name().name() == child_tag)
            .unwrap()
            .text()
            .unwrap()
    }

    struct MockMediaFile {
        name: String,
        stem: String,
        extension: String,
        len: u64,
    }

    impl Default for MockMediaFile {
        fn default() -> Self {
            Self {
                name: "name1.mp3".to_owned(),
                stem: "name1".to_owned(),
                extension: "mp3".to_owned(),
                len: 123,
            }
        }
    }

    impl MediaFileLike for MockMediaFile {
        fn name(&self) -> &str {
            &self.name
        }

        fn stem(&self) -> &str {
            &self.stem
        }

        fn extension(&self) -> &str {
            &self.extension
        }

        fn len(&self) -> Result<u64, Error> {
            Ok(self.len)
        }
    }

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
    fn generates_xml_for_files() {
        let path = Path::new("test_fixtures/dir1/file1.mp3").to_path_buf();
        let file = MediaFile { path: &path };
        let generator = FeedGenerator {
            title: "Feed Title 1".to_owned(),
            base_url: "https://eg.test".to_owned(),
        };
        let mut buffer = Vec::new();
        let result = generator.generate_for_files(vec![file], &mut buffer);
        assert!(result.is_ok(), "expected generate_for_files to return ok");
        let feed = String::from_utf8(buffer).unwrap();
        assert_contains!(feed, "<title>Feed Title 1</title>");
        assert_contains!(feed, "xmlns:itunes");
        assert_contains!(feed, "<itunes:block>Yes</itunes:block>");
        assert_contains!(
            feed,
            "url=\"https://eg.test/file1.mp3\" length=\"6\" type=\"audio/mpeg\""
        );
        assert_contains!(feed, "<title>file1</title>");
    }

    #[test]
    fn returns_error_if_file_does_not_exist() {
        let path = Path::new("invalid-file-1.mp3").to_path_buf();
        let file = MediaFile { path: &path };
        let generator = FeedGenerator {
            title: "Feed Title 1".to_owned(),
            base_url: "https://eg.test".to_owned(),
        };
        let mut buffer = Vec::new();
        let result = generator.generate_for_files(vec![file], &mut buffer);
        assert!(
            result.is_err(),
            "expected generate_for_files to return an error"
        );
    }

    #[test]
    fn outputs_correct_mime_type() {
        assert_eq!(FeedGenerator::mime_type("mp3"), "audio/mpeg");
        assert_eq!(FeedGenerator::mime_type("mp4"), "audio/mp4");
        assert_eq!(FeedGenerator::mime_type("aac"), "audio/aac");
        assert_eq!(FeedGenerator::mime_type("m4a"), "audio/mp4");
    }

    #[test]
    fn pub_dates_go_in_reverse() {
        let files = vec![
            MockMediaFile {
                stem: "file1".to_owned(),
                ..Default::default()
            },
            MockMediaFile {
                stem: "file2".to_owned(),
                ..Default::default()
            },
            MockMediaFile {
                stem: "file3".to_owned(),
                ..Default::default()
            },
        ];
        let generator = FeedGenerator {
            title: "Feed Title 1".to_owned(),
            base_url: "https://eg.test".to_owned(),
        };
        let mut buffer = Vec::new();
        generator.generate_for_files(files, &mut buffer).unwrap();
        let feed = String::from_utf8(buffer).unwrap();
        let doc = Document::parse(&feed).unwrap();
        let items: Vec<Node> = doc
            .descendants()
            .filter(|n| n.tag_name().name() == "item")
            .collect();
        assert_eq!(items.len(), 3);
        let today = Utc::today().and_hms(0, 0, 0);
        let item = items.get(0).unwrap();
        assert_eq!(get_child_node_text(item, "title"), "file1");
        assert_eq!(
            get_child_node_text(item, "pubDate"),
            (today - Duration::days(0)).to_rfc2822()
        );
        let item = items.get(1).unwrap();
        assert_eq!(get_child_node_text(item, "title"), "file2");
        assert_eq!(
            get_child_node_text(item, "pubDate"),
            (today - Duration::days(1)).to_rfc2822()
        );
        let item = items.get(2).unwrap();
        assert_eq!(get_child_node_text(item, "title"), "file3");
        assert_eq!(
            get_child_node_text(item, "pubDate"),
            (today - Duration::days(2)).to_rfc2822()
        );
    }

    #[test]
    fn handles_special_chars_in_filenames() {
        let files = vec![MockMediaFile {
            name: "a+b c&d.mp3".to_owned(),
            stem: "a+b c&d".to_owned(),
            ..Default::default()
        }];
        let generator = FeedGenerator {
            title: "Feed Title 1".to_owned(),
            base_url: "https://eg.test".to_owned(),
        };
        let mut buffer = Vec::new();
        generator.generate_for_files(files, &mut buffer).unwrap();
        let feed = String::from_utf8(buffer).unwrap();
        let doc = Document::parse(&feed).unwrap();
        let item = doc
            .descendants()
            .find(|n| n.tag_name().name() == "item")
            .unwrap();
        assert_eq!(get_child_node_text(&item, "title"), "a+b c&d");
        let enclosure = doc
            .descendants()
            .find(|n| n.tag_name().name() == "enclosure")
            .unwrap();
        assert_eq!(
            enclosure.attribute("url"),
            Some("https://eg.test/a%2Bb%20c%26d.mp3")
        );
    }

    #[test]
    fn replaces_underscores_with_spaces_in_title() {
        let files = vec![MockMediaFile {
            name: "ab_cd.mp3".to_owned(),
            stem: "ab_cd".to_owned(),
            ..Default::default()
        }];
        let generator = FeedGenerator {
            title: "Feed Title 1".to_owned(),
            base_url: "https://eg.test".to_owned(),
        };
        let mut buffer = Vec::new();
        generator.generate_for_files(files, &mut buffer).unwrap();
        let feed = String::from_utf8(buffer).unwrap();
        let doc = Document::parse(&feed).unwrap();
        let item = doc
            .descendants()
            .find(|n| n.tag_name().name() == "item")
            .unwrap();
        assert_eq!(get_child_node_text(&item, "title"), "ab cd");
        let enclosure = doc
            .descendants()
            .find(|n| n.tag_name().name() == "enclosure")
            .unwrap();
        assert_eq!(
            enclosure.attribute("url"),
            Some("https://eg.test/ab_cd.mp3")
        );
    }
}
