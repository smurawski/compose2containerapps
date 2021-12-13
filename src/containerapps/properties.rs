use serde::{Deserialize, Serialize};

use super::{Configuration, Template};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Properties {
    #[serde(rename = "kubeEnvironmentId")]
    pub kube_environment_id: String,
    pub configuration: Configuration,
    pub template: Template,
}
