# netplan-types

## Motivation
This crate attempts to map the entire netplan configuration into Rust structs and enums.
The 'layout' is as close to what you'd write in your netplan YAML files.

The intented use of this crate is to allow for easy editing of the network configuration via the netplan
configuration files from a Rust program.

Based on the documentation from netplan, which can be found [here](https://netplan.io/reference/)
Please note that I do not check the docs often for updates, if anything is missing or incorrect in the future,
please open an issue or a pull-request so the issue can be addressed.

## Features
- `serde`: [Default] Add serde support
- `derive_builder` Enable the derive_builder crate for an automatically generated builder pattern API
- `repr-c` Add the `repr(C)` attribute to every type

## License
This crate is licensed under the MIT license, or the Apache 2.0 license, at your discretion.