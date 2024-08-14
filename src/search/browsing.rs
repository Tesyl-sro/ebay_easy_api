use crate::{
    error::{handle_response_error, Error, Result},
    models::{Item, SearchResults},
    utils::Jsonify,
    EbayApiClient,
};
use reqwest::{Method, StatusCode};

const SEARCH_ENDPOINT: &str = "buy/browse/v1/item_summary/search";
const ITEM_SEARCH_ENDPOINT: &str = "buy/browse/v1/item/";

const SEARCH_API_LIMIT: usize = 200;

/// A Browser API client.
///
/// This structure references an instance of [`EbayApiClient`](EbayApiClient), so
/// it can only be used as long as the referenced [`EbayApiClient`](EbayApiClient) lives.
pub struct Browser<'c>(pub(crate) &'c EbayApiClient);

impl<'c> Browser<'c> {
    /// Perform a search using the given query string and return up to `limit` results.
    ///
    /// # Errors
    /// This can return an error if the request fails, or the response body could not be deserialized.
    ///
    /// # Example
    /// ```rust
    /// # use ebay_easy_api::{models::Marketplace, EbayApiClient};
    /// # use std::env::var;
    /// # let token = var("EBAY_TOKEN").unwrap();
    /// # let marketplace = Marketplace::UnitedStates;
    /// let client = EbayApiClient::new_unchecked(token, marketplace);
    ///
    /// // Obtain a search API client
    /// let browser = client.search();
    ///
    /// // Define a limit
    /// let limit: usize = 3;
    ///
    /// // Perform the search query
    /// let results = browser.search("gaming pc", limit);
    ///
    /// // If the search was successfull, we should get the results.
    /// assert!(results.is_ok());
    ///
    /// // We should not get more than `limit` results.
    /// assert!(results.unwrap().len() <= limit);
    /// ```
    pub fn search<S: AsRef<str>>(&self, query: S, limit: usize) -> Result<SearchResults> {
        if limit > SEARCH_API_LIMIT {
            return Err(Error::Limit(SEARCH_API_LIMIT));
        }

        let limit_as_str = limit.to_string();
        let query = [("q", query.as_ref()), ("limit", limit_as_str.as_str())];

        let builder = self
            .0
            .request_builder(Method::GET, SEARCH_ENDPOINT)
            .query(&query);
        let response = builder.send()?;

        let response = handle_response_error(response)?;
        let response = response.jsonify::<SearchResults>()?;

        Ok(response)
    }

    /// Finds an item by it's ID.
    ///
    /// # Errors
    /// This can return an error if the request fails or the response body could not be deserialized.
    /// Furthermore, if the item was not found [`None`](None) is returned.
    ///
    /// # Example
    /// ```rust
    /// # use ebay_easy_api::{models::Marketplace, EbayApiClient};
    /// # use std::env::var;
    /// # let token = var("EBAY_TOKEN").unwrap();
    /// # let marketplace = Marketplace::UnitedStates;
    /// let client = EbayApiClient::new_unchecked(token, marketplace);
    ///
    /// // Obtain a search API client
    /// let browser = client.search();
    ///
    /// // Perform the search query
    /// let result = browser.find_item("v1|202975928242|0");
    ///
    /// // If the search was successfull, we should get the results.
    /// assert!(result.is_ok());
    ///
    /// match result.unwrap() {
    ///     Some(item) => println!("Found {}, with price {}", item.title, item.price),
    ///     None => println!("Item was not found"),
    /// }
    /// ```
    pub fn find_item<S: AsRef<str>>(&self, id: S) -> Result<Option<Item>> {
        let endpoint = format!("{ITEM_SEARCH_ENDPOINT}{}", id.as_ref());
        let builder = self.0.request_builder(Method::GET, endpoint);

        let response = builder.send()?;

        match handle_response_error(response) {
            Ok(response) => Ok(Some(response.jsonify()?)),
            Err(why) => {
                if why.status_code() == Some(StatusCode::NOT_FOUND) {
                    return Ok(None);
                }

                Err(why)
            }
        }
    }

    pub fn paginate(&self, results: SearchResults) -> Result<Option<SearchResults>> {
        let Some(next_url) = results.next else {
            return Ok(None);
        };

        let builder = self.0.request_builder_with_url(Method::GET, next_url);
        let response = builder.send()?;

        let response = handle_response_error(response)?;
        let response = response.jsonify::<SearchResults>()?;

        Ok(Some(response))
    }
}
