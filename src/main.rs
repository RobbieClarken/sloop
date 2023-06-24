use std::fs::File;
use std::path::PathBuf;
use std::process;
use structopt::StructOpt;

mod feed;
mod upload;

#[derive(Debug, StructOpt)]
#[structopt(about = "audiobook to podcast tool")]
enum Opt {
    Feed {
        #[structopt(long)]
        title: String,
        #[structopt(long)]
        image: Option<PathBuf>,
        #[structopt(long)]
        region: String,
        #[structopt(long)]
        bucket: String,
        #[structopt(short, long)]
        out: PathBuf,
        #[structopt(long)]
        upload: bool,
        #[structopt(parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    Upload {
        #[structopt(long)]
        region: String,
        #[structopt(long)]
        bucket: String,
        #[structopt(parse(from_os_str))]
        files: Vec<PathBuf>,
    },
}

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Feed {
            title,
            image,
            region,
            bucket,
            out,
            upload,
            files,
        } => {
            let uploader = upload::S3Uploader::new(&region, &bucket).unwrap();
            let feed = feed::FeedGenerator {
                title,
                base_url: uploader.base_url(),
                image: image.clone().map(|path| feed::Image { path }),
            };
            let media_files = files.iter().map(|path| feed::MediaFile { path }).collect();
            if let Err(e) = feed.generate_for_files(media_files, File::create(&out).unwrap()) {
                eprintln!("Failed to create feed: {}", e);
                process::exit(1);
            }
            if upload {
                let feed_url = uploader.url_for_file(&out);
                let mut upload_files = vec![out];
                if let Some(image) = &image {
                    upload_files.push(image.clone());
                }
                upload_files.extend(files);
                match uploader.upload(upload_files) {
                    Ok(_) => {
                        eprintln!("Upload complete");
                        eprintln!("Podcast available at {}", feed_url);
                        process::exit(0);
                    }
                    Err(e) => {
                        eprintln!("Upload error: {}", e.message);
                        process::exit(1);
                    }
                }
            }
        }
        Opt::Upload {
            region,
            bucket,
            files,
        } => {
            let uploader = upload::S3Uploader::new(&region, &bucket).unwrap();
            uploader.upload(files).unwrap();
        }
    };
}
