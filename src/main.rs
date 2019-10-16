use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

mod feed;
mod upload;

#[derive(Debug, StructOpt)]
#[structopt(about = "audiobook to podcast tool")]
enum Opt {
    Feed {
        #[structopt(short, long)]
        title: String,
        #[structopt(short, long)]
        region: String,
        #[structopt(short, long)]
        bucket: String,
        #[structopt(short, long)]
        out: PathBuf,
        #[structopt(parse(from_os_str))]
        files: Vec<PathBuf>,
    },
    Upload {
        #[structopt(short, long)]
        region: String,
        #[structopt(short, long)]
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
            region,
            bucket,
            out,
            files,
        } => {
            let uploader = upload::S3Uploader::new(&region, &bucket).unwrap();
            let feed = feed::FeedGenerator {
                title,
                base_url: uploader.base_url(),
            };
            let out = File::create(out).unwrap();
            let files = files.iter().map(|path| feed::MediaFile { path }).collect();
            feed.generate_for_files(files, out);
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
