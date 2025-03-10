use async_trait::async_trait;
use tokio::time::sleep;
use std::time::Duration;
use crate::Query;
use tokio::time::Instant;

pub struct Test {
    url: String,
}

impl Test {
    pub fn new(url: String) -> Self {
        Test { url }
    }
}

#[async_trait]
impl Query for Test {
    async fn run(&self) -> Result<String, Box<dyn std::error::Error>> {
        let time = Instant::now();

        sleep(Duration::from_secs(9)).await;

        let results = format!("Test for {} done in {:?}", self.url, time.elapsed());
        Ok(results)
    }
}
