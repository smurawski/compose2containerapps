use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod configuration;
mod properties;
mod template;

pub use configuration::*;
pub use properties::Properties;
pub use template::*;

pub fn write_to_containerapps_file(file_path: &Path, config: &ContainerAppConfig) -> Result<()> {
    let output_content = serde_yaml::to_string(config)?;
    let mut file = File::create(file_path)
        .unwrap_or_else(|_| panic!("Failed to create the output file - {:?}.", file_path));
    file.write_all(output_content.into_bytes().as_ref())?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerAppConfig {
    pub kind: String,
    pub location: String,
    pub name: String,
    #[serde(rename = "resourceGroup")]
    pub resource_group: String,
    #[serde(rename = "type")]
    pub resource_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, String>>,
    pub properties: Properties,
}
impl Default for ContainerAppConfig {
    fn default() -> ContainerAppConfig {
        ContainerAppConfig {
            kind: "containerapp".to_string(),
            location: String::default(),
            name: String::default(),
            resource_group: String::default(),
            resource_type: "Microsoft.Web/containerApps".to_string(),
            tags: None,
            properties: Properties::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    #[test]
    fn default_containerapps_config_can_serialize() {
        let config = ContainerAppConfig::default();
        serde_yaml::to_value(config).unwrap();
    }

    #[test]
    fn sample_containerapps_yaml_deserializes_properly() {
        let file = File::open("test/containerapps.yml").unwrap();
        let _config: ContainerAppConfig = serde_yaml::from_reader(file).unwrap();
    }
}
