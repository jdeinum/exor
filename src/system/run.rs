use crate::{cli::util::RunArgs, inventory::get_inventory};
use anyhow::{Context, Result};

pub async fn run(args: &RunArgs) -> Result<()> {
    // first we'll want to get all of the machines and their values
    let machines = get_inventory(args.inventory.as_ref()).context("get inventory")?;

    // next, we'll want to parse the playbook

    // finally we'll run the system
    // to start, we'll only use ssh using pki

    Ok(())
}
