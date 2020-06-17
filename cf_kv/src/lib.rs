use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait(?Send)]
pub trait CloudFlareKV<'de, T>
where
    T: Serialize + Deserialize<'de>,
{
    async fn get(key: &str) -> Result<Option<T>, std::io::Error>;

    async fn put(key: &str, value: T) -> Result<T, std::io::Error>;

    async fn delete(key: &str) -> Result<(), std::io::Error>;
}
