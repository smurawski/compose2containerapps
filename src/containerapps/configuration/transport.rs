use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Transport {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "http2")]
    Http2,
}
impl Default for Transport {
    fn default() -> Self {
        Transport::Auto
    }
}
impl fmt::Display for Transport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl FromStr for Transport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let result = match s {
            "Auto" => Transport::Auto,
            "Http" => Transport::Http,
            "Http2" => Transport::Http2,
            _ => Transport::default(),
        };
        Ok(result)
    }
}
