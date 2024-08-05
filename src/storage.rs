use aws_config::Region;
use aws_sdk_s3::{
    config::{Credentials, SharedCredentialsProvider},
    Client,
};
use axum::body::Bytes;
use eyre::Result;

#[derive(Clone)]
pub(crate) struct Storage {
    client: Client,
    bucket_name: String,
}

impl Storage {
    pub(crate) fn builder() -> StorageBuilder {
        StorageBuilder::new()
    }

    pub(crate) async fn put_object(&self, object: StorageObject) -> Result<()> {
        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(object.key)
            .body(object.data.into())
            .send()
            .await?;
        Ok(())
    }
}

pub(crate) struct StorageBuilder;

impl StorageBuilder {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) async fn initialize(&self, config: StorageConfig) -> Result<Storage> {
        let endpoint_url = format!("https://{}.r2.cloudflarestorage.com", config.account_id);
        let credentials =
            Credentials::new(config.access_id, config.access_secret, None, None, "custom");
        let r2_config = aws_config::from_env()
            .region(Region::new(config.region))
            .credentials_provider(SharedCredentialsProvider::new(credentials))
            .endpoint_url(&endpoint_url)
            .load()
            .await;
        let client = aws_sdk_s3::Client::new(&r2_config);
        Ok(Storage {
            client,
            bucket_name: config.bucket_name,
        })
    }
}

#[derive(Clone)]
pub(crate) struct StorageConfig {
    access_id: String,
    access_secret: String,
    account_id: String,
    bucket_name: String,
    region: String,
}

impl StorageConfig {
    pub(crate) fn load() -> Result<Self> {
        Ok(Self {
            access_id: std::env::var("R2_ACCESS_ID")?,
            access_secret: std::env::var("R2_ACCESS_SECRET")?,
            account_id: std::env::var("R2_ACCOUNT_ID")?,
            bucket_name: std::env::var("R2_BUCKET_NAME")?,
            region: std::env::var("R2_REGION")?,
        })
    }
}

#[derive(Clone)]
pub(crate) struct StorageObject {
    pub(crate) key: String,
    pub(crate) data: Bytes,
}
