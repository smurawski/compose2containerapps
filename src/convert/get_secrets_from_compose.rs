use crate::compose::Compose;
use crate::containerapps::SecretsConfiguration;
use anyhow::Result;

pub fn get_secrets_from_compose(_compose_file: &Compose) -> Result<Vec<SecretsConfiguration>> {
    let secrets = Vec::new();
    Ok(secrets)
}
