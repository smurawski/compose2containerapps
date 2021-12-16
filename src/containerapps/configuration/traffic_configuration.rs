use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct TrafficConfiguration {
    #[serde(rename = "latestRevision", skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,
}
