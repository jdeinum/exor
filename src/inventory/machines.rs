use crate::error::Result;
use anyhow::Context;
/// I want the inventory file to have the same feel as sveltekits routing
///
/// The inventory will be specified using a hierarchical structure of directories and files within
/// them. Namely:
/// 1. Directories represent groups, any device in the current level and further down are part of
///    the group.
/// 2. Variables defined at level X are applied to all levels below it unless there is a specific
///    key for overriding that value
///
/// Example:
/// .
/// - inventory/
///     - values.toml           (A)
///     - machines.toml         (B)
///     - wired/
///         - machines.toml     (C)
///         - values.toml       (D)
///
/// Notes:
/// All machines are part of the root group (by default), while the devices specified
/// in C are also part of the wired group.
///
/// The machines specified in A have access to the values in B, while the machines specified in C
/// have access to values in both B and D, with values in D taking precedence.
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Our machine specification
/// Currently I'll pretend that the only piece of metadata we care about is the hostname
#[derive(Debug, Clone, Deserialize)]
pub struct Machine {
    pub host: String,
}

// required to deserialize into
#[derive(Debug, Deserialize)]
struct MachinesWrapper {
    machines: Vec<Machine>,
}

/// Our variables specification
/// Currently we'll assume everything is a string
#[derive(Debug, Clone, Deserialize)]
pub struct Variables {
    #[serde(flatten)]
    pub vars: HashMap<String, String>,
}

/// Our receusive node structure
#[derive(Debug)]
pub struct InventoryNode {
    pub _path: PathBuf,
    pub machines: Vec<Machine>,
    pub vars: Variables,
    pub children: Vec<InventoryNode>,
}

/// The output we want from loading an inventory is a vector of hosts with their defined variables
pub struct MachineWithVars {
    pub machine: Machine,
    pub vars: HashMap<String, String>,
}

/// helper function to load our inventory
fn load_inventory(path: &Path) -> Result<InventoryNode> {
    let machines_path = path.join("machines.toml");
    let values_path = path.join("values.toml");

    let machines: Vec<Machine> = if machines_path.exists() {
        let w: MachinesWrapper =
            toml::from_str(&fs::read_to_string(&machines_path).context("read path to string")?)
                .context("read machines")?;
        w.machines
    } else {
        vec![]
    };

    let vars: Variables = if values_path.exists() {
        toml::from_str(&fs::read_to_string(&values_path).context("read path to string")?)
            .context("read variables")?
    } else {
        Variables {
            vars: HashMap::new(),
        }
    };

    let mut children = Vec::new();
    for entry in fs::read_dir(path).context("read directory to get children")? {
        let entry = entry.context("unwrap entry")?;
        if entry.file_type().context("get file type")?.is_dir() {
            children.push(load_inventory(&entry.path())?);
        }
    }

    Ok(InventoryNode {
        _path: path.to_path_buf(),
        machines,
        vars,
        children,
    })
}

/// helper function to get the variables associated with a machine
pub fn flatten_inventory(
    node: &InventoryNode,
    inherited_vars: &HashMap<String, String>,
) -> Vec<MachineWithVars> {
    // merge inherited vars with current, favoring current
    let mut merged_vars = inherited_vars.clone();
    merged_vars.extend(node.vars.vars.clone());

    let mut results = vec![];

    for machine in &node.machines {
        results.push(MachineWithVars {
            machine: machine.clone(),
            vars: merged_vars.clone(),
        });
    }

    for child in &node.children {
        results.extend(flatten_inventory(child, &merged_vars));
    }

    results
}

/// Exported function that is used by the rest of the system to get the inventory
pub fn get_inventory(root: &Path) -> Result<Vec<MachineWithVars>> {
    let i = load_inventory(root).context("load inventory")?;
    Ok(flatten_inventory(&i, &HashMap::new()))
}
