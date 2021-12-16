use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct SecretsConfiguration {
    pub name: String,
    pub value: String,
}
