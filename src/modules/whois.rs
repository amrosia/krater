use async_trait::async_trait;
use crate::Query;
use tokio::time::Instant;
use whois_rust::{WhoIs, WhoIsLookupOptions};

pub struct Whois {
    url: String,
}

impl Whois {
    pub fn new(url: String) -> Self {
        Whois { url }
    }
}

#[async_trait]
impl Query for Whois {
    async fn run(&self) -> Result<String, Box<dyn std::error::Error>> {
        let time = Instant::now();

        let results = format!("WHOIS RESULTS PLACEHOLDER done in {:?}", time.elapsed());
        Ok(results)
    }
}
