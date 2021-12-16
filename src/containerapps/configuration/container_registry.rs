use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct ContainerRegistry {
    pub server: String,
    pub username: String,
    pub password_secret_ref: String,
}
