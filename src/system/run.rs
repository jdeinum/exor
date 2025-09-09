use std::path::Path;

use crate::{cli::util::RunArgs, command::generate::generate_commands, inventory::get_inventory};
use anyhow::{Context, Result};

pub async fn run(args: &RunArgs) -> Result<()> {
    // first we'll want to get all of the machines and their values
    let machines = get_inventory(args.inventory.as_ref()).context("get inventory")?;

    // next, we'll want to parse the playbook into a series of commands
    let commands = generate_commands(Path::new(&args.playbook))
        .await
        .context("get playbook")?;

    // create the connection to the remote server
    // this happens over quic, the exact method as to how we do that securely is yet to be
    // determined

    // finally we'll run the system

    Ok(())
}
