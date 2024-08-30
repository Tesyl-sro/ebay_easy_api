use ebay_easy_api::{models::Marketplace, Client};
use std::env;

fn main() {
    let token = env::var("EBAY_TOKEN").expect("Missing ebay API token");
    let marketplace = Marketplace::UnitedStates;
    let client = Client::new(token, marketplace).unwrap();

    let browser = client.search();
    let results = browser.search("omnron plc", 3).unwrap();

    println!("{results:#?}");
}
