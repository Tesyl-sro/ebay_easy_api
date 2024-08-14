use ebay_easy_api::{models::Marketplace, EbayApiClient};
use std::env::var;

#[test]
fn pagination() {
    let client =
        EbayApiClient::new_unchecked(var("EBAY_TOKEN").unwrap(), Marketplace::UnitedStates);

    let browser = client.search();
    let results = browser.search("omron plc", 10).unwrap();

    assert!(!results.items.is_empty());
    assert_ne!(results.total, 0);
    assert_ne!(results.len(), 0);

    let results = browser.paginate(results).unwrap().unwrap();

    assert!(!results.items.is_empty());
    assert_ne!(results.total, 0);
    assert_ne!(results.len(), 0);
}
