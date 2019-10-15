use chrono::Utc;
use rss::extension::itunes::{ITunesChannelExtensionBuilder, NAMESPACE};
use rss::{ChannelBuilder, EnclosureBuilder, Item, ItemBuilder};
use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::path::PathBuf;

pub trait MediaFileLike {
    fn name(&self) -> &str;
    fn stem(&self) -> &str;
    fn extension(&self) -> &str;
    fn len(&self) -> u64;
}

struct MediaFile {
    path: PathBuf,
}

impl MediaFileLike for MediaFile {
    fn name(&self) -> &str {
        self.path.as_path().file_name().unwrap().to_str().unwrap()
    }

    fn stem(&self) -> &str {
        self.path.as_path().file_stem().unwrap().to_str().unwrap()
    }

    fn extension(&self) -> &str {
        self.path.as_path().extension().unwrap().to_str().unwrap()
    }

    fn len(&self) -> u64 {
        std::fs::metadata(&self.path).unwrap().len()
    }
}

pub struct FeedGenerator {
    pub title: String,
    pub base_url: String,
}

impl FeedGenerator {
    pub fn generate_for_dir<W: Write>(&self, files_dir: &str, writer: W) {
        let mut files = Vec::new();
        for dir_entry in fs::read_dir(files_dir).unwrap() {
            let file = MediaFile {
                path: dir_entry.unwrap().path(),
            };
            files.push(file);
        }
        self.generate_for_files(files, writer);
    }

    pub fn generate_for_files<W: Write, M: MediaFileLike>(&self, files: Vec<M>, mut writer: W) {
        let namespaces: HashMap<String, String> = [("itunes".to_string(), NAMESPACE.to_string())]
            .iter()
            .cloned()
            .collect();
        let itunes_ext = ITunesChannelExtensionBuilder::default()
            .block("Yes".to_string())
            .build()
            .unwrap();
        let mut items: Vec<Item> = Default::default();
        let date = Utc::today().and_hms(0, 0, 0); // TODO: change for each file
        for file in files {
            let enclosure = EnclosureBuilder::default()
                .url(format!("{}/{}", self.base_url, file.name()))
                .mime_type(FeedGenerator::mime_type(&file))
                .length(file.len().to_string())
                .build()
                .unwrap();
            let item = ItemBuilder::default()
                .title(Some(String::from(file.stem())))
                .enclosure(Some(enclosure))
                .build()
                .unwrap();
            items.push(item);
        }
        let channel = ChannelBuilder::default()
            .namespaces(namespaces)
            .title(self.title.clone())
            .itunes_ext(itunes_ext)
            .items(items)
            .pub_date(date.to_rfc2822())
            .build()
            .unwrap();
        channel.pretty_write_to(&mut writer, b' ', 2).unwrap();
    }

    fn mime_type(file: &MediaFileLike) -> String {
        match file.extension() {
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

        fn len(&self) -> u64 {
            self.len
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
    fn generates_xml_for_dir() {
        let mut buffer = Vec::new();
        let generator = FeedGenerator {
            title: "Feed Title 1".to_owned(),
            base_url: "https://eg.test".to_owned(),
        };
        generator.generate_for_dir("test_fixtures/dir1/", &mut buffer);
        let feed = String::from_utf8(buffer).unwrap();
        assert_contains!(feed, "<title>Feed Title 1</title>");
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

    #[test]
    fn outputs_correct_mime_type() {
        let mp3 = MockMediaFile { extension: "mp3".to_owned(), ..Default::default() };
        assert_eq!(FeedGenerator::mime_type(&mp3), "audio/mpeg");
        let mp4 = MockMediaFile { extension: "mp4".to_owned(), ..Default::default() };
        assert_eq!(FeedGenerator::mime_type(&mp4), "audio/mp4");
        let aac = MockMediaFile { extension: "aac".to_owned(), ..Default::default() };
        assert_eq!(FeedGenerator::mime_type(&aac), "audio/aac");
        let m4a = MockMediaFile { extension: "m4a".to_owned(), ..Default::default() };
        assert_eq!(FeedGenerator::mime_type(&m4a), "audio/mp4");
    }
}
