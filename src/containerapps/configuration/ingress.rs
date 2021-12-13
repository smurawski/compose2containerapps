use serde::{Deserialize, Serialize};

use super::{TrafficConfiguration, Transport};

#[derive(Debug, Serialize, Deserialize)]
pub struct IngressConfiguration {
    #[serde(default)]
    pub external: bool,
    #[serde(default, rename = "allowInsecure")]
    pub allow_insecure: bool,
    #[serde(rename = "targetPort", skip_serializing_if = "Option::is_none")]
    pub target_port: Option<u16>,
    #[serde(default = "Vec::new", skip_serializing_if = "Vec::is_empty")]
    pub traffic: Vec<TrafficConfiguration>,
    #[serde(default)]
    pub transport: Transport,
}
impl Default for IngressConfiguration {
    fn default() -> IngressConfiguration {
        IngressConfiguration {
            external: true,
            allow_insecure: false,
            target_port: Some(80),
            traffic: Vec::new(),
            transport: Transport::default(),
        }
    }
}
