use crate::compose::Compose;
use crate::containerapps::{Configuration, RevisionMode};
use anyhow::Result;

use super::get_ingress_from_compose;
use super::get_secrets_from_compose;

pub fn get_configuration_from_compose(compose_file: &Compose) -> Result<Configuration> {
    let config = Configuration {
        secrets: get_secrets_from_compose(compose_file)?,
        ingress: get_ingress_from_compose(compose_file)?,
        active_revisions_mode: RevisionMode::default(),
    };
    Ok(config)
}
