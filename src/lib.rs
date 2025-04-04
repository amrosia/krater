use async_trait::async_trait;
use std::error::Error;

#[async_trait]
pub trait Query {
    async fn run(&self) -> Result<String, Box<dyn Error>>;
}

// module runner, error handler
#[allow(dead_code)]
async fn run_module<T: Query>(module: T, module_name: &str) {
    match module.run().await {
        Ok(results) => {
            println!("| {}", module_name);
            let lines: Vec<&str> = results.lines().collect();
            for (i, line) in lines.iter().enumerate() {
            if i == lines.len() - 1 {
                println!("|_ {}", line);
            } else {
                println!("| {}", line);
            }
            }
        }
        Err(e) => eprintln!("{} module error: {}", module_name, e),
    }
}