[![Check](https://github.com/jdeinum/exor/workflows/check/badge.svg)](https://github.com/jdeinum/exor/actions)
[![Audit](https://github.com/jdeinum/exor/workflows/audit/badge.svg)](https://github.com/jdeinum/exor/actions)
[![Tests](https://github.com/jdeinum/exor/workflows/test/badge.svg)](https://github.com/jdeinum/exor/actions)

# Exor

A toy ansible-like project that focuses on provisoning speed and consistency.

# Goals

1. ~~Support for inventory files written in toml~~
2. Support for playbook files written in toml
3. Support for configuration files written in toml
4. Atomic playbooks, either everything works or we roll back to a previous state
   (conditional on whether commands with side effects are used)
5. A client / server style approach similar to SaltStack and Puppet
6. Quic as the transport mechanism
7. Support for installing packages using nix
8. Support for running commands
9. Support for starting processes using systemd and docker
10. Support for copying files from a target system to a remote system
11. A nice CLI for managing the tool
12. Prometheus metrics

# Things that would be cool but unlikely to happen (for now)

1. Commands / platbooks are WASM binaries
2. Support for registering devices with an identity management system like
   SPIRE
3. Decentralized provisioning approach (i.e when a device is added to a group,
   it can be provisioned from any device in its group)

# Inventory

I decided that for this project, I'd like inventories to be represented
similarily to how svelte handles its routes. Groups are defined as directories,
which contain a `machines.toml` and a `values.toml` which apply to all of
machines under it. More specific key-value pairs overwrite more general ones.
