// NOT production-ready, this crate is under development

mod modules;
use krater::parse_args;
use krater::Query;

// url modules
use modules::whois::Whois;
use modules::test::Test;

// module runner, error handler
async fn run_module<T: Query>(module: T, module_name: &str) {
    match module.run().await {
        Ok(results) => println!("{} module results:\n{}", module_name, results),
        Err(e) => eprintln!("{} module error: {}", module_name, e),
    }
}

#[tokio::main]
async fn main() {
    let args = parse_args();

    // --url modules logic
    if let Some(url) = args.url {
        let whois = Whois::new(url.clone());
        run_module(whois, "Whois").await;

        let test = Test::new(url.clone());
        run_module(test, "Test").await;
    } 
    // todo: remove this when another type of data except url added to code
    else {
        println!("No URL provided");
    }



}