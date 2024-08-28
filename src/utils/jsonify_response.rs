use crate::error::Result;
use reqwest::blocking::Response;
use serde::de::DeserializeOwned;
use std::io::Read;

pub trait Jsonify {
    fn jsonify<T: DeserializeOwned>(self) -> Result<T>;
}

impl Jsonify for Response {
    fn jsonify<T: DeserializeOwned>(mut self) -> Result<T> {
        // A search result with 200 items is ~310kB.
        // This should prevent some allocations.
        let mut buf = Vec::with_capacity(320 * 1024);
        self.read_to_end(&mut buf).unwrap();

        Ok(serde_json::from_slice(&buf)?)
    }
}
