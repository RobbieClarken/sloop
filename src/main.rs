mod upload;

fn main() {
    let uploader = upload::S3Uploader::new("ap-southeast-2", "sloop-rbc-test-1").unwrap();
    uploader.upload("test_fixtures/dir1/").unwrap();
}
