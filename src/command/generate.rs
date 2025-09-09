use std::path::Path;

use crate::{command::traits::Command, error::Result};

pub async fn generate_commands(
    playbook_path: &Path,
) -> Result<impl IntoIterator<Item = Box<dyn Command>>> {
    Ok(Vec::new())
}
