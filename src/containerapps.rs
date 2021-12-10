use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn write_to_containerapps_file(file_path: &Path, config: &ContainerAppConfig) -> Result<()> {
    let output_content = serde_yaml::to_string(config)?;
    let mut file = File::create(file_path).expect("Failed to create the output file.");
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

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Properties {
    #[serde(rename = "kubeEnvironmentId")]
    pub kube_environment_id: String,
    pub configuration: Configuration,
    pub template: Template,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Configuration {
    #[serde(rename = "activeRevisionsMode", default)]
    pub active_revisions_mode: RevisionMode,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub secrets: Vec<SecretsConfiguration>,
    pub ingress: IngressConfiguration,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct SecretsConfiguration {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IngressConfiguration {
    pub external: bool,
    #[serde(rename = "allowInsecure", skip_serializing_if = "Option::is_none")]
    pub allow_insecure: Option<bool>,
    #[serde(rename = "targetPort", skip_serializing_if = "Option::is_none")]
    pub target_port: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub traffic: Option<Vec<TrafficConfiguration>>,
    #[serde(default)]
    pub transport: Transport,
}
impl Default for IngressConfiguration {
    fn default() -> IngressConfiguration {
        IngressConfiguration {
            external: true,
            allow_insecure: Some(false),
            target_port: Some(80),
            traffic: None,
            transport: Transport::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Transport {
    #[serde(rename = "auto")]
    Auto,
    #[serde(rename = "http")]
    Http1,
    #[serde(rename = "http2")]
    Http2,
}
impl Default for Transport {
    fn default() -> Self {
        Transport::Auto
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct TrafficConfiguration {
    #[serde(rename = "latestRevision", skip_serializing_if = "Option::is_none")]
    pub latest_revision: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Template {
    #[serde(rename = "revisionSuffix", skip_serializing_if = "Option::is_none")]
    pub revision_suffix: Option<String>,
    pub containers: Vec<Container>,
    pub scale: ScaleConfiguration,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Container {
    pub image: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<EnvironmentConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourceConfiguration>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct EnvironmentConfiguration {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "secretRef", skip_serializing_if = "Option::is_none")]
    pub secret_ref: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct ResourceConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScaleConfiguration {
    #[serde(rename = "minReplicas")]
    pub min_replicas: u32,
    #[serde(rename = "maxReplicas", skip_serializing_if = "Option::is_none")]
    pub max_replicas: Option<u32>,
}

impl Default for ScaleConfiguration {
    fn default() -> Self {
        ScaleConfiguration {
            min_replicas: 1,
            max_replicas: None,
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
