# Exor

I want to build something from scratch that is reasonably complex and is
also helpful for my job. I landed on Ansible, which is a Python based system
that helps bring devices to a desired state.

I know that things like NixOS solve a similar problem, but I want a solution
that doesn't use a DSL.

# Goals

1. Support for inventory files written in toml
2. Support for installing packages using nix
3. Support for running commands as particular users
4. Support for containers, wasm, and lightweight VMs of somekind (tbd)
5. Support for copying files from a target system to a remote system
6. An identity management system builtin, like SPIRE
7. A nice CLI for managing the tool
8. Prometheus metrics
9. Performance concious, only sending what you need
10. Support for peer to peer communication
11. An algorithm that determines the fastest way to provision new devices
12. Simple configuration using toml
13. Provisoning for a device should happen atomically, if any action fails, it
    needs to be rolled back to its previous working state

# Non Goals

1. Non Linux support
