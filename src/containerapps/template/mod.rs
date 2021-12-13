use serde::{Deserialize, Serialize};

mod container;
mod environment_configuration;
mod resource_configuration;
mod scale_configuration;

pub use self::container::Container;
pub use self::environment_configuration::EnvironmentConfiguration;
pub use self::resource_configuration::ResourceConfiguration;
pub use self::scale_configuration::ScaleConfiguration;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Template {
    #[serde(rename = "revisionSuffix", skip_serializing_if = "Option::is_none")]
    pub revision_suffix: Option<String>,
    pub containers: Vec<Container>,
    pub scale: ScaleConfiguration,
}
