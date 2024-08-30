use ebay_easy_api::{models::Marketplace, Client};
use std::env::var;

#[test]
fn item_details() {
    let client =
        Client::new_unchecked(var("EBAY_TOKEN").unwrap(), Marketplace::UnitedStates);

    let browser = client.search();
    let results = browser.search("gaming pc", 2).unwrap();

    let first = &results[0];
    let details = browser.find_item(&first.id).unwrap();

    assert!(details.is_some());
}
