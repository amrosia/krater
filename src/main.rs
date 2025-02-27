// NOT production-ready, this crate is under development

mod modules;
use krater::parse_args;
use tokio::task::JoinHandle;
use modules::get_modules;
use krater::Query;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let args = parse_args();

    if let Some(url) = args.url {
        let modules = get_modules();
        let url = Arc::new(url);

        let mut handles: Vec<JoinHandle<String>> = Vec::new();

        for (name, module) in modules {
            let url = Arc::clone(&url);
            handles.push(tokio::spawn(async move {
                let result = module.run(url.clone()).await;
                result
            }));
            println!("Running {} module...", name);
        }

        for handle in handles {
            let result = handle.await.unwrap();
            println!("Module finished: {}", result);
        }
    } else {
        println!("No URL provided");
    }
}