use crate::error::Result;
use crate::utils::cache::cache_mem_service::MemCacheService;
use crate::utils::cache::cache_redis_service::RedisCacheService;
use crate::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use std::ops::Deref;

#[async_trait]
pub trait ICacheService: Sync + Send + Debug {
    /// set key-value, ex seconds expire, 0 = no expire
    async fn set_string(&self, k: &str, v: &str, ex: u64) -> Result<String>;

    /// get value from key
    async fn get_string(&self, k: &str) -> Result<String>;
    
    /// get key  Time To Live(secs), -2 = key does not exist, -1 = expire, 0 = no expire, >0 = seconds until expire
    async fn ttl(&self, k: &str) -> Result<i64>;

    async fn remove(&self, k: &str) -> Result<bool>;

}


pub struct CacheService {
    pub inner: Box<dyn ICacheService>,
}

pub enum CacheType {
    Mem,
    Redis,
}


impl  CacheService{

    pub fn new(cache_type: &str) -> Result<Self> {
        match cache_type {
            "mem" => {
                println!("[cache] cache_type: mem");
                Ok(Self {
                    inner: Box::new(MemCacheService::new()),
                })
            }
            "redis" => {
                println!("[cache] cache_type: redis");
                Ok(Self {
                    inner: Box::new(RedisCacheService::new()),
                })
            }
            _ => {
                Err(Error::Internal(format!("cache_type:{} not support", cache_type)))
            }
           
        }
    }


    // pub async fn set_string(&self, k: &str, v: &str) -> Result<String> {
    //     self.inner.set_string(k, v).await
    // }

    // pub async fn get_string(&self, k: &str) -> Result<String> {
    //     self.inner.get_string(k).await
    // }

    pub async fn set_json<T>(&self, k: &str, v: &T, ex: u64) -> Result<String>
    where
        T: Serialize + Sync,
    {
        let data = serde_json::to_string(v)
           .map_err(|e| Error::Internal(format!("MemCacheService set_json fail:{}", e)))?;
       
        let data = self.set_string(k, data.as_str(), ex).await?;
        Ok(data)
    }

    pub async fn get_json<T>(&self, k: &str) -> Result<T>
    where
        T: DeserializeOwned + Sync,
    {
        let mut r = self.get_string(k).await?;
        if r.is_empty() {
            r = "null".to_string();
        }
        let data: T = serde_json::from_str(r.as_str())
           .map_err(|e| Error::Internal(format!("MemCacheService get_json fail:{}", e)))?;
        
        Ok(data)
    }

    

    // pub async fn ttl(&self, k: &str) -> Result<i64> {
    //     self.inner.ttl(k).await
    // }
}

impl Deref for CacheService {
    type Target = Box<dyn ICacheService>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Default for CacheService{
    fn default() -> Self {
        Self::new("mem").unwrap()
    }
}



#[tokio::test]
async fn test_cache_service() {
    let cache = CacheService::new("mem").unwrap();
    let _ = cache.set_string("test", "123", 0).await;
    let v = cache.get_string("test").await.unwrap();
    assert_eq!(v, "123");
    let _ = cache.set_json("test_json", &123, 0).await;
    let v = cache.get_json::<i32>("test_json").await.unwrap();
    assert_eq!(v, 123);
    
}