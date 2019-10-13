use rusoto_core::{RusotoError, RusotoFuture};
use rusoto_s3::CreateBucketError::{BucketAlreadyExists, BucketAlreadyOwnedByYou};
use rusoto_s3::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Default)]
pub struct S3Mock {
    pub create_bucket_request: Rc<RefCell<Vec<CreateBucketRequest>>>,
    pub create_bucket_error: Option<CreateBucketError>,
}

impl S3 for S3Mock {
    fn create_bucket(
        &self,
        request: CreateBucketRequest,
    ) -> RusotoFuture<CreateBucketOutput, CreateBucketError> {
        self.create_bucket_request.borrow_mut().push(request);
        match &self.create_bucket_error {
            None => RusotoFuture::from(Ok(CreateBucketOutput { location: None })),
            Some(e) => match e {
                BucketAlreadyOwnedByYou(msg) => Err(RusotoError::Service(BucketAlreadyOwnedByYou(
                    msg.to_string(),
                )))
                .into(),
                BucketAlreadyExists(msg) => Err(RusotoError::Service(BucketAlreadyExists(
                    msg.to_string(),
                )))
                .into(),
            },
        }
    }

    fn abort_multipart_upload(
        &self,
        _input: AbortMultipartUploadRequest,
    ) -> RusotoFuture<AbortMultipartUploadOutput, AbortMultipartUploadError> {
        unimplemented!();
    }

    fn put_object(
        &self,
        _input: PutObjectRequest,
    ) -> RusotoFuture<PutObjectOutput, PutObjectError> {
        unimplemented!();
    }

    fn complete_multipart_upload(
        &self,
        _: CompleteMultipartUploadRequest,
    ) -> RusotoFuture<CompleteMultipartUploadOutput, CompleteMultipartUploadError> {
        unimplemented!()
    }

    fn copy_object(&self, _: CopyObjectRequest) -> RusotoFuture<CopyObjectOutput, CopyObjectError> {
        unimplemented!()
    }

    fn create_multipart_upload(
        &self,
        _: CreateMultipartUploadRequest,
    ) -> RusotoFuture<CreateMultipartUploadOutput, CreateMultipartUploadError> {
        unimplemented!()
    }

    fn delete_bucket(&self, _: DeleteBucketRequest) -> RusotoFuture<(), DeleteBucketError> {
        unimplemented!()
    }

    fn delete_bucket_analytics_configuration(
        &self,
        _: DeleteBucketAnalyticsConfigurationRequest,
    ) -> RusotoFuture<(), DeleteBucketAnalyticsConfigurationError> {
        unimplemented!()
    }

    fn delete_bucket_cors(
        &self,
        _: DeleteBucketCorsRequest,
    ) -> RusotoFuture<(), DeleteBucketCorsError> {
        unimplemented!()
    }

    fn delete_bucket_encryption(
        &self,
        _: DeleteBucketEncryptionRequest,
    ) -> RusotoFuture<(), DeleteBucketEncryptionError> {
        unimplemented!()
    }

    fn delete_bucket_inventory_configuration(
        &self,
        _: DeleteBucketInventoryConfigurationRequest,
    ) -> RusotoFuture<(), DeleteBucketInventoryConfigurationError> {
        unimplemented!()
    }

    fn delete_bucket_lifecycle(
        &self,
        _: DeleteBucketLifecycleRequest,
    ) -> RusotoFuture<(), DeleteBucketLifecycleError> {
        unimplemented!()
    }

    fn delete_bucket_metrics_configuration(
        &self,
        _: DeleteBucketMetricsConfigurationRequest,
    ) -> RusotoFuture<(), DeleteBucketMetricsConfigurationError> {
        unimplemented!()
    }

    fn delete_bucket_policy(
        &self,
        _: DeleteBucketPolicyRequest,
    ) -> RusotoFuture<(), DeleteBucketPolicyError> {
        unimplemented!()
    }

    fn delete_bucket_replication(
        &self,
        _: DeleteBucketReplicationRequest,
    ) -> RusotoFuture<(), DeleteBucketReplicationError> {
        unimplemented!()
    }

    fn delete_bucket_tagging(
        &self,
        _: DeleteBucketTaggingRequest,
    ) -> RusotoFuture<(), DeleteBucketTaggingError> {
        unimplemented!()
    }

    fn delete_bucket_website(
        &self,
        _: DeleteBucketWebsiteRequest,
    ) -> RusotoFuture<(), DeleteBucketWebsiteError> {
        unimplemented!()
    }

    fn delete_object(
        &self,
        _: DeleteObjectRequest,
    ) -> RusotoFuture<DeleteObjectOutput, DeleteObjectError> {
        unimplemented!()
    }

    fn delete_object_tagging(
        &self,
        _: DeleteObjectTaggingRequest,
    ) -> RusotoFuture<DeleteObjectTaggingOutput, DeleteObjectTaggingError> {
        unimplemented!()
    }

    fn delete_objects(
        &self,
        _: DeleteObjectsRequest,
    ) -> RusotoFuture<DeleteObjectsOutput, DeleteObjectsError> {
        unimplemented!()
    }

    fn get_bucket_accelerate_configuration(
        &self,
        _: GetBucketAccelerateConfigurationRequest,
    ) -> RusotoFuture<GetBucketAccelerateConfigurationOutput, GetBucketAccelerateConfigurationError>
    {
        unimplemented!()
    }

    fn get_bucket_acl(
        &self,
        _: GetBucketAclRequest,
    ) -> RusotoFuture<GetBucketAclOutput, GetBucketAclError> {
        unimplemented!()
    }

    fn get_bucket_analytics_configuration(
        &self,
        _: GetBucketAnalyticsConfigurationRequest,
    ) -> RusotoFuture<GetBucketAnalyticsConfigurationOutput, GetBucketAnalyticsConfigurationError>
    {
        unimplemented!()
    }

    fn get_bucket_cors(
        &self,
        _: GetBucketCorsRequest,
    ) -> RusotoFuture<GetBucketCorsOutput, GetBucketCorsError> {
        unimplemented!()
    }

    fn get_bucket_encryption(
        &self,
        _: GetBucketEncryptionRequest,
    ) -> RusotoFuture<GetBucketEncryptionOutput, GetBucketEncryptionError> {
        unimplemented!()
    }

    fn get_bucket_inventory_configuration(
        &self,
        _: GetBucketInventoryConfigurationRequest,
    ) -> RusotoFuture<GetBucketInventoryConfigurationOutput, GetBucketInventoryConfigurationError>
    {
        unimplemented!()
    }

    fn get_bucket_lifecycle(
        &self,
        _: GetBucketLifecycleRequest,
    ) -> RusotoFuture<GetBucketLifecycleOutput, GetBucketLifecycleError> {
        unimplemented!()
    }

    fn get_bucket_lifecycle_configuration(
        &self,
        _: GetBucketLifecycleConfigurationRequest,
    ) -> RusotoFuture<GetBucketLifecycleConfigurationOutput, GetBucketLifecycleConfigurationError>
    {
        unimplemented!()
    }

    fn get_bucket_location(
        &self,
        _: GetBucketLocationRequest,
    ) -> RusotoFuture<GetBucketLocationOutput, GetBucketLocationError> {
        unimplemented!()
    }

    fn get_bucket_logging(
        &self,
        _: GetBucketLoggingRequest,
    ) -> RusotoFuture<GetBucketLoggingOutput, GetBucketLoggingError> {
        unimplemented!()
    }

    fn get_bucket_metrics_configuration(
        &self,
        _: GetBucketMetricsConfigurationRequest,
    ) -> RusotoFuture<GetBucketMetricsConfigurationOutput, GetBucketMetricsConfigurationError> {
        unimplemented!()
    }

    fn get_bucket_notification(
        &self,
        _: GetBucketNotificationConfigurationRequest,
    ) -> RusotoFuture<NotificationConfigurationDeprecated, GetBucketNotificationError> {
        unimplemented!()
    }

    fn get_bucket_notification_configuration(
        &self,
        _: GetBucketNotificationConfigurationRequest,
    ) -> RusotoFuture<NotificationConfiguration, GetBucketNotificationConfigurationError> {
        unimplemented!()
    }

    fn get_bucket_policy(
        &self,
        _: GetBucketPolicyRequest,
    ) -> RusotoFuture<GetBucketPolicyOutput, GetBucketPolicyError> {
        unimplemented!()
    }

    fn get_bucket_replication(
        &self,
        _: GetBucketReplicationRequest,
    ) -> RusotoFuture<GetBucketReplicationOutput, GetBucketReplicationError> {
        unimplemented!()
    }

    fn get_bucket_request_payment(
        &self,
        _: GetBucketRequestPaymentRequest,
    ) -> RusotoFuture<GetBucketRequestPaymentOutput, GetBucketRequestPaymentError> {
        unimplemented!()
    }

    fn get_bucket_tagging(
        &self,
        _: GetBucketTaggingRequest,
    ) -> RusotoFuture<GetBucketTaggingOutput, GetBucketTaggingError> {
        unimplemented!()
    }

    fn get_bucket_versioning(
        &self,
        _: GetBucketVersioningRequest,
    ) -> RusotoFuture<GetBucketVersioningOutput, GetBucketVersioningError> {
        unimplemented!()
    }

    fn get_bucket_website(
        &self,
        _: GetBucketWebsiteRequest,
    ) -> RusotoFuture<GetBucketWebsiteOutput, GetBucketWebsiteError> {
        unimplemented!()
    }

    fn get_object(&self, _: GetObjectRequest) -> RusotoFuture<GetObjectOutput, GetObjectError> {
        unimplemented!()
    }

    fn get_object_acl(
        &self,
        _: GetObjectAclRequest,
    ) -> RusotoFuture<GetObjectAclOutput, GetObjectAclError> {
        unimplemented!()
    }

    fn get_object_tagging(
        &self,
        _: GetObjectTaggingRequest,
    ) -> RusotoFuture<GetObjectTaggingOutput, GetObjectTaggingError> {
        unimplemented!()
    }

    fn get_object_torrent(
        &self,
        _: GetObjectTorrentRequest,
    ) -> RusotoFuture<GetObjectTorrentOutput, GetObjectTorrentError> {
        unimplemented!()
    }

    fn head_bucket(&self, _: HeadBucketRequest) -> RusotoFuture<(), HeadBucketError> {
        unimplemented!()
    }

    fn head_object(&self, _: HeadObjectRequest) -> RusotoFuture<HeadObjectOutput, HeadObjectError> {
        unimplemented!()
    }

    fn list_bucket_analytics_configurations(
        &self,
        _: ListBucketAnalyticsConfigurationsRequest,
    ) -> RusotoFuture<ListBucketAnalyticsConfigurationsOutput, ListBucketAnalyticsConfigurationsError>
    {
        unimplemented!()
    }

    fn list_bucket_inventory_configurations(
        &self,
        _: ListBucketInventoryConfigurationsRequest,
    ) -> RusotoFuture<ListBucketInventoryConfigurationsOutput, ListBucketInventoryConfigurationsError>
    {
        unimplemented!()
    }

    fn list_bucket_metrics_configurations(
        &self,
        _: ListBucketMetricsConfigurationsRequest,
    ) -> RusotoFuture<ListBucketMetricsConfigurationsOutput, ListBucketMetricsConfigurationsError>
    {
        unimplemented!()
    }

    fn list_buckets(&self) -> RusotoFuture<ListBucketsOutput, ListBucketsError> {
        unimplemented!()
    }

    fn list_multipart_uploads(
        &self,
        _: ListMultipartUploadsRequest,
    ) -> RusotoFuture<ListMultipartUploadsOutput, ListMultipartUploadsError> {
        unimplemented!()
    }

    fn list_object_versions(
        &self,
        _: ListObjectVersionsRequest,
    ) -> RusotoFuture<ListObjectVersionsOutput, ListObjectVersionsError> {
        unimplemented!()
    }

    fn list_objects(
        &self,
        _: ListObjectsRequest,
    ) -> RusotoFuture<ListObjectsOutput, ListObjectsError> {
        unimplemented!()
    }

    fn list_objects_v2(
        &self,
        _: ListObjectsV2Request,
    ) -> RusotoFuture<ListObjectsV2Output, ListObjectsV2Error> {
        unimplemented!()
    }

    fn list_parts(&self, _: ListPartsRequest) -> RusotoFuture<ListPartsOutput, ListPartsError> {
        unimplemented!()
    }

    fn put_bucket_accelerate_configuration(
        &self,
        _: PutBucketAccelerateConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketAccelerateConfigurationError> {
        unimplemented!()
    }

    fn put_bucket_acl(&self, _: PutBucketAclRequest) -> RusotoFuture<(), PutBucketAclError> {
        unimplemented!()
    }

    fn put_bucket_analytics_configuration(
        &self,
        _: PutBucketAnalyticsConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketAnalyticsConfigurationError> {
        unimplemented!()
    }

    fn put_bucket_cors(&self, _: PutBucketCorsRequest) -> RusotoFuture<(), PutBucketCorsError> {
        unimplemented!()
    }

    fn put_bucket_encryption(
        &self,
        _: PutBucketEncryptionRequest,
    ) -> RusotoFuture<(), PutBucketEncryptionError> {
        unimplemented!()
    }

    fn put_bucket_inventory_configuration(
        &self,
        _: PutBucketInventoryConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketInventoryConfigurationError> {
        unimplemented!()
    }

    fn put_bucket_lifecycle(
        &self,
        _: PutBucketLifecycleRequest,
    ) -> RusotoFuture<(), PutBucketLifecycleError> {
        unimplemented!()
    }

    fn put_bucket_lifecycle_configuration(
        &self,
        _: PutBucketLifecycleConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketLifecycleConfigurationError> {
        unimplemented!()
    }

    fn put_bucket_logging(
        &self,
        _: PutBucketLoggingRequest,
    ) -> RusotoFuture<(), PutBucketLoggingError> {
        unimplemented!()
    }

    fn put_bucket_metrics_configuration(
        &self,
        _: PutBucketMetricsConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketMetricsConfigurationError> {
        unimplemented!()
    }

    fn put_bucket_notification(
        &self,
        _: PutBucketNotificationRequest,
    ) -> RusotoFuture<(), PutBucketNotificationError> {
        unimplemented!()
    }

    fn put_bucket_notification_configuration(
        &self,
        _: PutBucketNotificationConfigurationRequest,
    ) -> RusotoFuture<(), PutBucketNotificationConfigurationError> {
        unimplemented!()
    }

    fn put_bucket_policy(
        &self,
        _: PutBucketPolicyRequest,
    ) -> RusotoFuture<(), PutBucketPolicyError> {
        unimplemented!()
    }

    fn put_bucket_replication(
        &self,
        _: PutBucketReplicationRequest,
    ) -> RusotoFuture<(), PutBucketReplicationError> {
        unimplemented!()
    }

    fn put_bucket_request_payment(
        &self,
        _: PutBucketRequestPaymentRequest,
    ) -> RusotoFuture<(), PutBucketRequestPaymentError> {
        unimplemented!()
    }

    fn put_bucket_tagging(
        &self,
        _: PutBucketTaggingRequest,
    ) -> RusotoFuture<(), PutBucketTaggingError> {
        unimplemented!()
    }

    fn put_bucket_versioning(
        &self,
        _: PutBucketVersioningRequest,
    ) -> RusotoFuture<(), PutBucketVersioningError> {
        unimplemented!()
    }

    fn put_bucket_website(
        &self,
        _: PutBucketWebsiteRequest,
    ) -> RusotoFuture<(), PutBucketWebsiteError> {
        unimplemented!()
    }

    fn put_object_acl(
        &self,
        _: PutObjectAclRequest,
    ) -> RusotoFuture<PutObjectAclOutput, PutObjectAclError> {
        unimplemented!()
    }

    fn put_object_tagging(
        &self,
        _: PutObjectTaggingRequest,
    ) -> RusotoFuture<PutObjectTaggingOutput, PutObjectTaggingError> {
        unimplemented!()
    }

    fn restore_object(
        &self,
        _: RestoreObjectRequest,
    ) -> RusotoFuture<RestoreObjectOutput, RestoreObjectError> {
        unimplemented!()
    }

    fn select_object_content(
        &self,
        _: SelectObjectContentRequest,
    ) -> RusotoFuture<SelectObjectContentOutput, SelectObjectContentError> {
        unimplemented!()
    }

    fn upload_part(&self, _: UploadPartRequest) -> RusotoFuture<UploadPartOutput, UploadPartError> {
        unimplemented!()
    }

    fn upload_part_copy(
        &self,
        _: UploadPartCopyRequest,
    ) -> RusotoFuture<UploadPartCopyOutput, UploadPartCopyError> {
        unimplemented!()
    }

    fn delete_public_access_block(
        &self,
        _input: DeletePublicAccessBlockRequest,
    ) -> RusotoFuture<(), DeletePublicAccessBlockError> {
        unimplemented!();
    }

    fn get_bucket_policy_status(
        &self,
        _input: GetBucketPolicyStatusRequest,
    ) -> RusotoFuture<GetBucketPolicyStatusOutput, GetBucketPolicyStatusError> {
        unimplemented!();
    }

    fn get_object_legal_hold(
        &self,
        _input: GetObjectLegalHoldRequest,
    ) -> RusotoFuture<GetObjectLegalHoldOutput, GetObjectLegalHoldError> {
        unimplemented!();
    }

    fn get_object_lock_configuration(
        &self,
        _input: GetObjectLockConfigurationRequest,
    ) -> RusotoFuture<GetObjectLockConfigurationOutput, GetObjectLockConfigurationError> {
        unimplemented!();
    }

    fn get_object_retention(
        &self,
        _input: GetObjectRetentionRequest,
    ) -> RusotoFuture<GetObjectRetentionOutput, GetObjectRetentionError> {
        unimplemented!();
    }

    fn get_public_access_block(
        &self,
        _input: GetPublicAccessBlockRequest,
    ) -> RusotoFuture<GetPublicAccessBlockOutput, GetPublicAccessBlockError> {
        unimplemented!();
    }

    fn put_object_legal_hold(
        &self,
        _input: PutObjectLegalHoldRequest,
    ) -> RusotoFuture<PutObjectLegalHoldOutput, PutObjectLegalHoldError> {
        unimplemented!();
    }

    fn put_object_lock_configuration(
        &self,
        _input: PutObjectLockConfigurationRequest,
    ) -> RusotoFuture<PutObjectLockConfigurationOutput, PutObjectLockConfigurationError> {
        unimplemented!();
    }

    fn put_object_retention(
        &self,
        _input: PutObjectRetentionRequest,
    ) -> RusotoFuture<PutObjectRetentionOutput, PutObjectRetentionError> {
        unimplemented!();
    }

    fn put_public_access_block(
        &self,
        _input: PutPublicAccessBlockRequest,
    ) -> RusotoFuture<(), PutPublicAccessBlockError> {
        unimplemented!();
    }
}
