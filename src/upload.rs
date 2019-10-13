use rusoto_core::Region;
use rusoto_s3::{CreateBucketConfiguration, CreateBucketRequest, S3Client, S3};
use std::str::FromStr;

pub struct S3Uploader {
    client: Box<dyn S3>,
    region: String,
    bucket_name: String,
}

#[derive(Debug)]
pub struct UploadError {
    message: String,
}

impl S3Uploader {
    pub fn new(region: &str, bucket_name: &str) -> Result<Self, UploadError> {
        let rusoto_region = Region::from_str(region).or(Err(UploadError {
            message: format!("invalid region: {}", region),
        }))?;
        let client = S3Client::new(rusoto_region);
        Ok(Self {
            client: Box::new(client),
            region: region.to_owned(),
            bucket_name: bucket_name.to_owned(),
        })
    }

    pub fn upload(&self, _target_dir: &str) -> Result<(), UploadError> {
        self.create_bucket();
        Ok(())
    }

    fn create_bucket(&self) {
        let request = CreateBucketRequest {
            bucket: self.bucket_name.clone(),
            create_bucket_configuration: Some(CreateBucketConfiguration {
                location_constraint: Some(self.region.clone()),
            }),
            ..Default::default()
        };
        self.client.create_bucket(request).sync().unwrap();
    }
}

#[cfg(test)]
mod tests {
    mod s3_mock;

    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn creates_an_s3_bucket() {
        let requests = Rc::new(RefCell::new(Vec::new()));
        let s3 = s3_mock::S3Mock {
            create_bucket_request: Rc::clone(&requests),
        };
        let uploader = S3Uploader {
            client: Box::new(s3),
            region: String::from("region1"),
            bucket_name: String::from("bucket1"),
        };
        uploader.upload("test_fixtures/dir1/file1.txt").unwrap();
        let request = requests.borrow().get(0).unwrap().clone();
        assert_eq!(request.bucket, "bucket1");
        assert_eq!(
            request
                .create_bucket_configuration
                .expect("bucket configuration not set")
                .location_constraint,
            Some(String::from("region1")),
        );
    }
}
