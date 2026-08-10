//! Raw Stonks response types.
//!
//! Names and fields intentionally mirror the upstream response schema.

use derive_setters::Setters;
use serde::de::Error;
use serde::{Deserialize, Deserializer, Serialize};

/// Public-token metadata returned by Stonks.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Setters)]
#[setters(prefix = "with_")]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct PublicToken {
    /// Token symbol supplied by Stonks.
    pub symbol: String,
    /// Raw TON token address supplied by Stonks.
    pub address: String,
    /// Raw buy-tax percentage; this value is not converted to basis points.
    pub buy_tax: u16,
    /// Raw sell-tax percentage; this value is not converted to basis points.
    pub sell_tax: u16,
}

/// Jetton metadata returned by the Stonks batch metadata endpoint.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize, Setters)]
#[setters(prefix = "with_")]
#[non_exhaustive]
pub struct JettonMetadata {
    /// Numeric Stonks deployment identifier.
    pub id: u64,
    /// Token display name.
    pub name: Option<String>,
    /// Optional canonical metadata URI.
    pub uri: Option<String>,
    /// Token display symbol.
    pub symbol: Option<String>,
    /// Token description.
    pub description: Option<String>,
    /// Token image URL.
    pub image: Option<String>,
    /// Inline token image bytes when returned by the service.
    pub image_data: Option<Vec<u8>>,
    /// Token decimal precision. Stonks may encode this as a number or string.
    #[serde(default, deserialize_with = "deserialize_optional_u8")]
    pub decimals: Option<u8>,
}

fn deserialize_optional_u8<'de, D>(deserializer: D) -> Result<Option<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum NumberOrString {
        Number(u8),
        String(String),
    }

    Option::<NumberOrString>::deserialize(deserializer)?
        .map(|value| match value {
            NumberOrString::Number(value) => Ok(value),
            NumberOrString::String(value) => value.parse().map_err(D::Error::custom),
        })
        .transpose()
}

#[cfg(test)]
mod tests {
    use super::JettonMetadata;

    #[test]
    fn test_jetton_metadata_accepts_string_and_numeric_decimals() -> anyhow::Result<()> {
        let string: JettonMetadata = serde_json::from_str(r#"{"id":1,"decimals":"9"}"#)?;
        let number: JettonMetadata = serde_json::from_str(r#"{"id":2,"decimals":6}"#)?;

        assert_eq!(string.decimals, Some(9));
        assert_eq!(number.decimals, Some(6));
        Ok(())
    }
}
