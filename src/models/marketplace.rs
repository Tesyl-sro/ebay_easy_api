use crate::ReadOnlyString;
use enum_iterator::Sequence;
use serde::Deserialize;
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

/// An Ebay marketplace ID.
#[derive(
    Debug, Clone, Deserialize, Sequence, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum Marketplace {
    #[default]
    #[serde(rename = "EBAY_US")]
    UnitedStates,
    #[serde(rename = "EBAY_DE")]
    Germany,
    #[serde(rename = "EBAY_IT")]
    Italy,
    #[serde(rename = "EBAY_IR")]
    Ireland,
    #[serde(rename = "EBAY_SG")]
    Singapore,
    #[serde(rename = "EBAY_UK")]
    UnitedKingdom,
    #[serde(rename = "EBAY_FR")]
    France,
    // Could be 210 (French) or 2 (English)
    #[serde(rename = "EBAY_CA")]
    Canada,
    #[serde(rename = "EBAY_AU")]
    Australia,
    #[serde(rename = "EBAY_AT")]
    Austria,
    // Could be 123 (Dutch) or 23 (French)
    #[serde(rename = "EBAY_BE")]
    Belgium,
    #[serde(rename = "EBAY_ES")]
    Spain,
    #[serde(rename = "EBAY_CH")]
    Switzerland,
    #[serde(rename = "EBAY_NL")]
    Netherlands,
    #[serde(rename = "EBAY_HK")]
    HongKong,
    #[serde(rename = "EBAY_IN")]
    India,
    #[serde(rename = "EBAY_MY")]
    Malaysia,
    #[serde(rename = "EBAY_PH")]
    Philippines,
    #[serde(rename = "EBAY_PL")]
    Poland,
}

/// An error representing a failed conversion from a string to a marketplace ID.
#[derive(Debug, Error)]
#[error("Invalid or unknown marketplace: '{0}'")]
pub struct InvalidMarketplace(ReadOnlyString);

impl Marketplace {
    /// Returns the marketplace ID as a static string.
    ///
    /// # Example
    /// ```rust
    /// use ebay_easy_api::models::Marketplace;
    ///
    /// assert_eq!(Marketplace::UnitedStates.as_str(), "EBAY_US");
    /// assert_eq!(Marketplace::Germany.as_str(), "EBAY_DE");
    /// // ...
    /// ```
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnitedStates => "EBAY_US",
            Self::Germany => "EBAY_DE",
            Self::Italy => "EBAY_IT",
            Self::Ireland => "EBAY_IR",
            Self::Singapore => "EBAY_SG",
            Self::UnitedKingdom => "EBAY_UK",
            Self::France => "EBAY_FR",
            Self::Canada => "EBAY_CA",
            Self::Australia => "EBAY_AU",
            Self::Austria => "EBAY_AT",
            Self::Belgium => "EBAY_BE",
            Self::Spain => "EBAY_ES",
            Self::Switzerland => "EBAY_CH",
            Self::Netherlands => "EBAY_NL",
            Self::HongKong => "EBAY_HK",
            Self::India => "EBAY_IN",
            Self::Malaysia => "EBAY_MY",
            Self::Philippines => "EBAY_PH",
            Self::Poland => "EBAY_PL",
        }
    }

    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::UnitedStates => "United States",
            Self::Germany => "Germany",
            Self::Italy => "Italy",
            Self::Ireland => "Ireland",
            Self::Singapore => "Singapore",
            Self::UnitedKingdom => "United Kingdom",
            Self::France => "France",
            Self::Canada => "Canada",
            Self::Australia => "Australia",
            Self::Austria => "Austria",
            Self::Belgium => "Belgium",
            Self::Spain => "Spain",
            Self::Switzerland => "Switzerland",
            Self::Netherlands => "Netherlands",
            Self::HongKong => "Hong",
            Self::India => "India",
            Self::Malaysia => "Malaysia",
            Self::Philippines => "Philippines",
            Self::Poland => "Poland",
        }
    }
}

impl FromStr for Marketplace {
    type Err = InvalidMarketplace;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const VARIANTS: [Marketplace; 7] = [
            Marketplace::UnitedStates,
            Marketplace::Germany,
            Marketplace::Italy,
            Marketplace::Ireland,
            Marketplace::Singapore,
            Marketplace::UnitedKingdom,
            Marketplace::France,
        ];

        for variant in VARIANTS {
            if s == variant.as_str() {
                return Ok(variant);
            }
        }

        Err(InvalidMarketplace(s.into()))
    }
}

impl Display for Marketplace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
