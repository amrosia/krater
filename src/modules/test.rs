use async_trait::async_trait;
use tokio::time::sleep;
use std::time::Duration;
use crate::Query;
use std::sync::Arc;
use tokio::time::Instant;

pub struct Test {}

#[async_trait]
impl Query for Test {
    async fn run(&self, url: Arc<String>) -> String {
        let time = Instant::now();
        sleep(Duration::from_secs(9)).await;
        format!("Test for {} in {:?}", *url, time.elapsed())
    }
}
