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

pub fn write_containerapps_arm_template(file_path: &Path, config: &ContainerAppConfig) -> Result<()> {
    let mut arm_template_outline = ArmWrapper::default();

    let mut container_config = config.clone();
    container_config.kind = None;
    container_config.api_version = Some("2021-03-01".to_string());
    container_config.resource_group = None;
    arm_template_outline.resources.push(container_config);
    arm_template_outline.outputs.containerapp_fqdn = OutputValue::new(&config.name); 

    let output_content = serde_json::to_string(&arm_template_outline)?;
    let mut file = File::create(file_path)
        .unwrap_or_else(|_| panic!("Failed to create the output file - {:?}.", file_path));
    file.write_all(output_content.into_bytes().as_ref())?;
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArmWrapper {
    #[serde(rename="$schema")]
    pub schema: &'static str,
    #[serde(rename="contentVersion")]
    pub content_version: &'static str,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<ContainerAppConfig>,
    pub outputs: OutputWrapper,
}
impl Default for ArmWrapper {
    fn default() -> ArmWrapper {
        ArmWrapper {
            schema: "https://schema.management.azure.com/schemas/2019-08-01/deploymentTemplate.json#",
            content_version: "1.0.0.0",
            resources: Vec::new(),
            outputs: OutputWrapper::default(),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct OutputWrapper {
    #[serde(rename = "containerappFqdn")]
    pub containerapp_fqdn: OutputValue
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OutputValue {
    #[serde(rename = "type")]
    pub output_type: String,
    pub value: String
}
impl OutputValue {
    pub fn new(service_name: &str) -> OutputValue {
        OutputValue {
            output_type: "string".to_string(), 
            value: format!("[reference(resourceId('Microsoft.Web/containerApps', '{}')).configuration.ingress.fqdn]", service_name)
        }
    }
}
impl Default for OutputValue {
    fn default() -> OutputValue {
        OutputValue {
            output_type: "string".to_string(),
            value: "[reference(resourceId('Microsoft.Web/containerApps', 'SERVICENAME')).configuration.ingress.fqdn]".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContainerAppConfig {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "apiVersion", skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    pub location: String,
    pub name: String,
    #[serde(rename = "resourceGroup", skip_serializing_if = "Option::is_none")]
    pub resource_group: Option<String>,
    #[serde(rename = "type")]
    pub resource_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, String>>,
    pub properties: Properties,
}
impl Default for ContainerAppConfig {
    fn default() -> ContainerAppConfig {
        ContainerAppConfig {
            kind: Some("containerapp".to_string()),
            api_version: None,
            location: String::default(),
            name: String::default(),
            resource_group: None,
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
