use crate::Query;
use std::collections::HashMap;

pub mod whois;
pub mod test;

use whois::Whois;
use test::Test;

pub fn get_modules() -> HashMap<&'static str, Box<dyn Query + Send + Sync>> {
    let mut modules: HashMap<&str, Box<dyn Query + Send + Sync>> = HashMap::new();
    modules.insert("whois", Box::new(Whois {}));
    modules.insert("test", Box::new(Test {}));
    
    modules
}