use serde::{Deserialize, Serialize};

mod container_registry;
mod ingress;
mod revision_mode;
mod secrets;
mod traffic_configuration;
mod transport;

pub use self::container_registry::ContainerRegistry;
pub use self::ingress::IngressConfiguration;
pub use self::revision_mode::RevisionMode;
pub use self::secrets::SecretsConfiguration;
pub use self::traffic_configuration::TrafficConfiguration;
pub use self::transport::Transport;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Configuration {
    #[serde(rename = "activeRevisionsMode", default)]
    pub active_revisions_mode: RevisionMode,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<SecretsConfiguration>,
    pub ingress: IngressConfiguration,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub registries: Vec<ContainerRegistry>,
}
