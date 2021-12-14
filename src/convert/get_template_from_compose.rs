use crate::compose::Compose;
use crate::containerapps::{ScaleConfiguration, Template};
use anyhow::Result;

use super::get_containers_from_compose;

pub fn get_template_from_compose(compose_file: &Compose) -> Result<Template> {
    let template = Template {
        containers: get_containers_from_compose(compose_file),
        revision_suffix: None,
        scale: ScaleConfiguration::default(),
    };
    Ok(template)
}
