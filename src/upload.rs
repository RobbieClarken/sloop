use rusoto_core::{Region, RusotoError};
use rusoto_s3::CreateBucketError::BucketAlreadyOwnedByYou;
use rusoto_s3::{
    CreateBucketConfiguration, CreateBucketRequest, PutBucketPolicyRequest, PutObjectRequest,
    S3Client, S3,
};
use serde_json::json;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;

pub struct S3Uploader {
    client: Box<dyn S3>,
    region: String,
    bucket_name: String,
}

#[derive(Debug)]
pub struct UploadError {
    pub message: String,
}

impl S3Uploader {
    pub fn new(region: &str, bucket_name: &str) -> Result<Self, UploadError> {
        let rusoto_region = Region::from_str(region).or(Err(UploadError {
            message: format!("Invalid region: {}", region),
        }))?;
        let client = S3Client::new(rusoto_region);
        Ok(Self {
            client: Box::new(client),
            region: region.to_owned(),
            bucket_name: bucket_name.to_owned(),
        })
    }

    pub fn base_url(&self) -> String {
        format!(
            "https://{}.s3-{}.amazonaws.com",
            self.bucket_name, self.region
        )
    }

    pub fn url_for_file(&self, file: &PathBuf) -> String {
        format!(
            "{}/{}",
            self.base_url(), file.file_name().unwrap().to_str().unwrap()
        )
    }

    pub fn upload(&self, files: Vec<PathBuf>) -> Result<(), UploadError> {
        self.create_bucket()?;
        self.make_bucket_public()?;
        self.upload_files(files)?;
        Ok(())
    }

    fn create_bucket(&self) -> Result<(), UploadError> {
        let request = CreateBucketRequest {
            bucket: self.bucket_name.clone(),
            create_bucket_configuration: Some(CreateBucketConfiguration {
                location_constraint: Some(self.region.clone()),
            }),
            ..Default::default()
        };
        if let Some(err) = self.client.create_bucket(request).sync().err() {
            match err {
                RusotoError::Service(BucketAlreadyOwnedByYou(_)) => {}
                _ => {
                    return Err(UploadError {
                        message: format!("Failed to create bucket: {}", err),
                    });
                }
            }
        }
        Ok(())
    }

    fn make_bucket_public(&self) -> Result<(), UploadError> {
        let policy = json!({
            "Version": "2012-10-17",
            "Statement": [{
                "Sid": "AddPerm",
                "Effect": "Allow",
                "Principal": "*",
                "Action": ["s3:GetObject"],
                "Resource": [format!("arn:aws:s3:::{}/*", &self.bucket_name)],
            }]
        })
        .to_string();
        let policy_request = PutBucketPolicyRequest {
            bucket: self.bucket_name.to_owned(),
            policy,
            ..Default::default()
        };
        self.client
            .put_bucket_policy(policy_request)
            .sync()
            .or(Err(UploadError {
                message: "Failed to set bucket policy".to_owned(),
            }))
    }

    fn upload_files(&self, files: Vec<PathBuf>) -> Result<(), UploadError> {
        for p in files {
            let file_name = p.file_name().unwrap().to_str().unwrap();
            let mut file = fs::File::open(&p).unwrap();
            let mut body = vec![];
            file.read_to_end(&mut body).unwrap();
            let request = PutObjectRequest {
                body: Some(body.into()),
                bucket: self.bucket_name.clone(),
                key: file_name.to_owned(),
                ..Default::default()
            };
            self.client.put_object(request).sync().unwrap();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    mod s3_mock;

    use super::*;
    use rusoto_s3::CreateBucketError::BucketAlreadyExists;
    use serde::Deserialize;
    use serde_json;
    use std::cell::RefCell;
    use std::path::Path;
    use std::rc::Rc;

    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    struct BucketPolicy {
        Version: String,
        Statement: Vec<BucketPolicyStatement>,
    }

    #[derive(Deserialize)]
    #[allow(non_snake_case)]
    struct BucketPolicyStatement {
        Sid: String,
        Effect: String,
        Principal: String,
        Action: Vec<String>,
        Resource: Vec<String>,
    }

    #[test]
    fn creates_an_s3_bucket() {
        let requests = Rc::new(RefCell::new(Vec::new()));
        let s3 = s3_mock::S3Mock {
            create_bucket_requests: Rc::clone(&requests),
            ..Default::default()
        };
        let uploader = S3Uploader {
            client: Box::new(s3),
            region: String::from("region1"),
            bucket_name: String::from("bucket1"),
        };
        uploader.upload(vec![]).unwrap();
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

    #[test]
    fn handles_bucket_already_owned_by_user() {
        let s3 = s3_mock::S3Mock {
            create_bucket_error: Some(BucketAlreadyOwnedByYou(String::new())),
            ..Default::default()
        };
        let uploader = S3Uploader {
            client: Box::new(s3),
            region: String::from("region1"),
            bucket_name: String::from("bucket1"),
        };
        uploader.upload(vec![]).unwrap();
    }

    #[test]
    fn throws_for_bucket_already_exists() {
        let s3 = s3_mock::S3Mock {
            create_bucket_error: Some(BucketAlreadyExists(String::new())),
            ..Default::default()
        };
        let uploader = S3Uploader {
            client: Box::new(s3),
            region: String::from("region1"),
            bucket_name: String::from("bucket1"),
        };
        assert_eq!(uploader.upload(vec![]).is_err(), true);
    }

    #[test]
    fn sets_bucket_policy_to_public() {
        let requests = Rc::new(RefCell::new(Vec::new()));
        let s3 = s3_mock::S3Mock {
            put_bucket_policy_requests: Rc::clone(&requests),
            ..Default::default()
        };
        let uploader = S3Uploader {
            client: Box::new(s3),
            region: String::from("region1"),
            bucket_name: String::from("bucket1"),
        };
        uploader.upload(vec![]).unwrap();
        let request = requests.borrow().get(0).unwrap().clone();
        assert_eq!(request.bucket, "bucket1");
        let policy: BucketPolicy = serde_json::from_str(&request.policy).unwrap();
        assert_eq!(policy.Version, "2012-10-17");
        let statement = &policy.Statement[0];
        assert_eq!(statement.Sid, "AddPerm");
        assert_eq!(statement.Effect, "Allow");
        assert_eq!(statement.Principal, "*");
        assert_eq!(statement.Action[0], "s3:GetObject");
        assert_eq!(statement.Resource[0], "arn:aws:s3:::bucket1/*");
    }

    #[test]
    fn returns_error_if_setting_bucket_policy_fails() {
        let s3 = s3_mock::S3Mock {
            put_bucket_policy_error: true,
            ..Default::default()
        };
        let uploader = S3Uploader {
            client: Box::new(s3),
            region: String::from("region1"),
            bucket_name: String::from("bucket1"),
        };
        assert!(uploader.upload(vec![]).is_err(), "expected error");
    }

    #[test]
    fn uploads_files_in_directory() {
        let requests = Rc::new(RefCell::new(Vec::new()));
        {
            let s3 = s3_mock::S3Mock {
                put_object_requests: Rc::clone(&requests),
                ..Default::default()
            };
            let uploader = S3Uploader {
                client: Box::new(s3),
                region: String::from("region1"),
                bucket_name: String::from("bucket1"),
            };
            let files = vec![Path::new("test_fixtures/dir1/file1.mp3").to_path_buf()];
            uploader.upload(files).unwrap();
        }
        let requests = Rc::try_unwrap(requests).unwrap().into_inner();
        let request = requests.get(0).unwrap().clone();
        assert_eq!(request.bucket, String::from("bucket1"));
        assert_eq!(request.key, String::from("file1.mp3"));
        assert_eq!(request.body, b"data1\n");
    }

    #[test]
    fn base_url_returns_url_for_bucket() {
        let s3: s3_mock::S3Mock = Default::default();
        let uploader = S3Uploader {
            client: Box::new(s3),
            region: String::from("region1"),
            bucket_name: String::from("bucket1"),
        };
        assert_eq!(
            uploader.base_url(),
            "https://bucket1.s3-region1.amazonaws.com"
        );
    }

    #[test]
    fn constructs_url_for_file() {
        let s3: s3_mock::S3Mock = Default::default();
        let uploader = S3Uploader {
            client: Box::new(s3),
            region: String::from("region1"),
            bucket_name: String::from("bucket1"),
        };
        assert_eq!(
            uploader.url_for_file(&PathBuf::from("/tmp/file1.txt")),
            "https://bucket1.s3-region1.amazonaws.com/file1.txt"
        );
    }
}
