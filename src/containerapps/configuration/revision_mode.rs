use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum RevisionMode {
    #[serde(rename = "multiple")]
    Multiple,
    #[serde(rename = "single")]
    Single,
}
impl Default for RevisionMode {
    fn default() -> Self {
        RevisionMode::Single
    }
}
