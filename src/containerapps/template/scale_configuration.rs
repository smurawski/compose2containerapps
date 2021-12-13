use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleConfiguration {
    #[serde(rename = "minReplicas")]
    pub min_replicas: u32,
    #[serde(rename = "maxReplicas", skip_serializing_if = "Option::is_none")]
    pub max_replicas: Option<u32>,
}

impl Default for ScaleConfiguration {
    fn default() -> Self {
        ScaleConfiguration {
            min_replicas: 1,
            max_replicas: None,
        }
    }
}
