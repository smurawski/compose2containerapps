use serde::{Deserialize, Serialize};

use super::{EnvironmentConfiguration, ResourceConfiguration};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct Container {
    pub image: String,
    pub name: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<EnvironmentConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceConfiguration>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub command: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub args: Vec<String>,
}
