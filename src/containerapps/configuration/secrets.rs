use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SecretsConfiguration {
    pub name: String,
    pub value: String,
}
