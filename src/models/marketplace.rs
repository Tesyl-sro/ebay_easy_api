use crate::ReadOnlyString;
use enum_iterator::{all, Sequence};
use serde::Deserialize;
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

/// An Ebay marketplace ID.
#[derive(
    Debug, Clone, Deserialize, Sequence, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub enum Marketplace {
    /// ID code: *EBAY_US*
    #[default]
    #[serde(rename = "EBAY_US")]
    UnitedStates,

    /// ID code: *EBAY_DE*
    #[serde(rename = "EBAY_DE")]
    Germany,

    /// ID code: *EBAY_IT*
    #[serde(rename = "EBAY_IT")]
    Italy,

    /// ID code: *EBAY_IE*
    #[serde(rename = "EBAY_IE")]
    Ireland,

    /// ID code: *EBAY_SG*
    #[serde(rename = "EBAY_SG")]
    Singapore,

    /// ID code: *EBAY_GB*
    #[serde(rename = "EBAY_GB")]
    UnitedKingdom,

    /// ID code: *EBAY_FR*
    #[serde(rename = "EBAY_FR")]
    France,

    /// ID code: *EBAY_CA*
    // Could be 210 (French) or 2 (English)
    #[serde(rename = "EBAY_CA")]
    Canada,

    /// ID code: *EBAY_AU*
    #[serde(rename = "EBAY_AU")]
    Australia,

    /// ID code: *EBAY_AT*
    #[serde(rename = "EBAY_AT")]
    Austria,

    /// ID code: *EBAY_BE*
    // Could be 123 (Dutch) or 23 (French)
    #[serde(rename = "EBAY_BE")]
    Belgium,

    /// ID code: *EBAY_ES*
    #[serde(rename = "EBAY_ES")]
    Spain,

    /// ID code: *EBAY_CH*
    #[serde(rename = "EBAY_CH")]
    Switzerland,

    /// ID code: *EBAY_NL*
    #[serde(rename = "EBAY_NL")]
    Netherlands,

    /// ID code: *EBAY_HK*
    #[serde(rename = "EBAY_HK")]
    HongKong,

    /// ID code: *EBAY_IN*
    #[serde(rename = "EBAY_IN")]
    India,

    /// ID code: *EBAY_MY*
    #[serde(rename = "EBAY_MY")]
    Malaysia,

    /// ID code: *EBAY_PH*
    #[serde(rename = "EBAY_PH")]
    Philippines,

    /// ID code: *EBAY_PL*
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
            Self::Ireland => "EBAY_IE",
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

    /// Returns the Marketplace country name.
    ///
    /// # Example
    /// ```rust
    /// use ebay_easy_api::models::Marketplace;
    ///
    /// assert_eq!(Marketplace::UnitedStates.as_str(), "United States");
    /// assert_eq!(Marketplace::Germany.as_str(), "Germany");
    /// // ...
    /// ```
    #[must_use]
    pub const fn country(self) -> &'static str {
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
            Self::HongKong => "Hong Kong",
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
        let variants = all::<Self>();

        for variant in variants {
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
