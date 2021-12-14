use anyhow::Result;
use std::path::Path;

pub use compose_yml::v3::File as Compose;
pub use compose_yml::v3::{PortMapping, Ports, Protocol, Service};

pub fn read_compose_file(path: &Path) -> Result<Compose> {
    let config: Compose = Compose::read_from_path(path)?;
    Ok(config)
}

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
