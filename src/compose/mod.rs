use anyhow::Result;
use std::path::Path;

pub use compose_yml::v3::File as Compose;
pub use compose_yml::v3::{PortMapping, Service, Ports};

pub fn read_compose_file(path: &Path) -> Result<Compose> {
    let config: Compose = Compose::read_from_path(path)?;
    Ok(config)
}

// #[derive(Default, Debug, Serialize, Deserialize)]
// pub struct Compose {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub version: Option<String>,
//     pub services: HashMap<String, ComposeService>,
//     // #[serde(skip_serializing_if = "Vec::is_empty")]
//     // pub volumes: Vec<ComposeVolume>,
//     // #[serde(skip_serializing_if = "Vec::is_empty")]
//     // pub configs: Vec<ComposeConfig>,
//     // #[serde(skip_serializing_if = "Vec::is_empty")]
//     // pub secrets: Vec<ComposeSecret>,
//     // #[serde(skip_serializing_if = "Vec::is_empty")]
//     // pub networks: Vec<ComposeNetwork>,
// }

// #[derive(Default, Debug, Serialize, Deserialize)]
// pub struct ComposeService {
//     pub image: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub container_name: Option<String>,
//     // Expose is internal ports
//     #[serde(default = "Vec::new", skip_serializing_if = "Vec::is_empty")]
//     pub expose: Vec<String>,
//     // ports are external
//     #[serde(skip_serializing_if = "Vec::is_empty")]
//     pub ports: Vec<String>,
//     #[serde(skip_serializing_if = "Vec::is_empty")]
//     pub environment: Vec<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub deploy: Option<ComposeServiceDeploy>,
// }

// #[derive(Default, Debug, Serialize, Deserialize)]
// pub struct ComposeServiceDeploy {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub endpoint_mode: Option<EndpointMode>,
//     // TODO: map to container app tags
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub labels: Option<HashMap<String, String>>,
//     pub mode: DeployMode,
//     // TODO: map to scale configuration
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub replicas: Option<u32>,
//     //TODO: map to resource configuration
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub resources: Option<ComposeServiceDeployResources>,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub enum EndpointMode {
//     #[serde(rename = "vip")]
//     VIP,
//     #[serde(rename = "dnsrr")]
//     DNSRR,
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub enum DeployMode {
//     #[serde(rename = "global")]
//     GLOBAL,
//     #[serde(rename = "replicated")]
//     REPLICATED,
// }
// impl Default for DeployMode {
//     fn default() -> Self {
//         DeployMode::REPLICATED
//     }
// }

// #[derive(Default, Debug, Serialize, Deserialize)]
// pub struct ComposeServiceDeployResources {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub limits: Option<ResourceConfiguration>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub reservations: Option<ResourceConfiguration>,
// }

// #[derive(Default, Debug, Serialize, Deserialize)]
// pub struct ResourceConfiguration {
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub cpus: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub memory: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub pids: Option<String>,
// }

// // #[derive(Default, Debug, Serialize, Deserialize)]
// // pub struct ComposeVolume {}

// // #[derive(Default, Debug, Serialize, Deserialize)]
// // pub struct ComposeConfig {}

// // #[derive(Default, Debug, Serialize, Deserialize)]
// // pub struct ComposeSecret {}

// // #[derive(Default, Debug, Serialize, Deserialize)]
// // pub struct ComposeNetwork {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn default_compose_can_serialize() {
        let config = Compose::default();
        serde_yaml::to_value(config).unwrap();
    }

    #[test]
    fn sample_compose_yaml_deserializes_properly() {
        let file = File::open("test/docker-compose.yml").unwrap();
        let _config: Compose = serde_yaml::from_reader(file).unwrap();
    }
}
