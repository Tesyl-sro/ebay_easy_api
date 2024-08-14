use crate::ReadOnlyString;
use rust_decimal::Decimal;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};
use std::fmt::Display;

/// An Ebay seller.
#[serde_as]
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Seller {
    /// Username of the seller account.
    pub username: ReadOnlyString,

    /// Feedback percentage as a fixed-precision decimal number.
    #[serde_as(as = "DisplayFromStr")]
    #[serde(rename = "feedbackPercentage")]
    pub feedback_percentage: Decimal,

    /// Feedback score. This can sometimes be `-1`.
    #[serde(rename = "feedbackScore")]
    pub feedback_score: i32,
}

impl Display for Seller {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.username)
    }
}
