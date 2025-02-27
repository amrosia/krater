use async_trait::async_trait;
use tokio::time::sleep;
use std::time::Duration;
use crate::Query;
use std::sync::Arc;
use tokio::time::Instant;

pub struct Whois {}

#[async_trait]
impl Query for Whois {
    async fn run(&self, url: Arc<String>) -> String {
        let time = Instant::now();
        sleep(Duration::from_secs(5)).await;
        format!("Whois for {} in {:?}", *url, time.elapsed())
    }
}
