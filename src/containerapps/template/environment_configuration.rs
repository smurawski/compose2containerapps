use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct EnvironmentConfiguration {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "secretRef", skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<String>,
}
