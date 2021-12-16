use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ResourceConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
}
