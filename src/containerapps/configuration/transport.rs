use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Transport {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "http")]
    Http1,
    #[serde(rename = "http2")]
    Http2,
}
impl Default for Transport {
    fn default() -> Self {
        Transport::Auto
    }
}
