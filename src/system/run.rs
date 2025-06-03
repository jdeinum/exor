use crate::{cli::util::RunArgs, inventory::get_inventory};
use anyhow::{Context, Result};

pub async fn run(args: &RunArgs) -> Result<()> {
    let machines = get_inventory(args.inventory.as_ref()).context("get inventory")?;
    for entry in machines {
        println!("Host: {}, Vars: {:?}", entry.machine.host, entry.vars);
    }

    Ok(())
}
