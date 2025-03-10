use clap::Parser;
use async_trait::async_trait;
use std::error::{self, Error};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub url: Option<String>,
}

pub fn parse_args() -> Args {
    Args::parse()
}

#[async_trait]
pub trait Query {
    async fn run(&self) -> Result<String, Box<dyn Error>>;
}