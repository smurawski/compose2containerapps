use crate::compose::Service;
use crate::containerapps::SecretsConfiguration;
use anyhow::Result;

pub fn get_secrets_from_service(_compose_file: &Service) -> Result<Vec<SecretsConfiguration>> {
    let secrets = Vec::new();
    Ok(secrets)
}
