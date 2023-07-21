//! netplan-types
//!
//! ## Motivation
//! This crate attempts to map the entire netplan configuration into Rust structs and enums.
//! The 'layout' is as close to what you'd write in your netplan YAML files.
//!
//! The intented use of this crate is to allow for easy editing of the network configuration via the netplan
//! configuration files from a Rust program.
//!
//! Based on the documentation from netplan, which can be found [here](https://netplan.io/reference/)
//! Please note that I do not check the docs often for updates, if anything is missing or incorrect in the future,
//! please open an issue or a pull-request so the issue can be addressed.
//!
//! ## Features
//! - `serde`: \[Default\] Add serde support
//! - `derive_builder` Enable the derive_builder crate for an automatically generated builder pattern API

#[cfg(feature = "serde")]
mod bool;

mod netplan;
pub use netplan::*;

use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "derive_builder")]
use derive_builder::Builder;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub struct NetplanConfig {
    pub network: NetworkConfig,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub struct NetworkConfig {
    pub version: u8,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub renderer: Option<Renderer>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ethernets: Option<HashMap<String, EthernetConfig>>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub wifis: Option<HashMap<String, WifiConfig>>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub bonds: Option<HashMap<String, BondConfig>>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub bridges: Option<HashMap<String, BridgeConfig>>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub vlans: Option<HashMap<String, VlanConfig>>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub tunnels: Option<HashMap<String, TunnelConfig>>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub vrfs: Option<HashMap<String, VrfsConfig>>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub dummy_devices: Option<HashMap<String, DummyDeviceConfig>>,
}

/// Use the given networking backend for this definition. Currently supported are
/// networkd and NetworkManager. This property can be specified globally
/// in network:, for a device type (in e. g. ethernets:) or
/// for a particular device definition. Default is networkd.
///
/// (Since 0.99) The renderer property has one additional acceptable value for vlan
/// objects (i. e. defined in vlans:): sriov. If a vlan is defined with the
/// sriov renderer for an SR-IOV Virtual Function interface, this causes netplan to
/// set up a hardware VLAN filter for it. There can be only one defined per VF.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Renderer {
    #[cfg_attr(feature = "serde", serde(rename = "networkd"))]
    Networkd,
    #[cfg_attr(feature = "serde", serde(rename = "NetworkManager"))]
    NetworkManager,
    #[cfg_attr(feature = "serde", serde(rename = "sriov"))]
    Sriov,
}

/// Takes a boolean, or the special value “route”. When true, the domain
/// name received from the DHCP server will be used as DNS search domain
/// over this link, similar to the effect of the Domains= setting. If set
/// to “route”, the domain name received from the DHCP server will be
/// used for routing DNS queries only, but not for searching, similar to
/// the effect of the Domains= setting when the argument is prefixed with
/// “~”.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[cfg_attr(feature = "serde", serde(rename = "lowercase"))]
pub enum UseDomains {
    Boolean(
        #[cfg_attr(
            feature = "serde",
            serde(deserialize_with = "crate::bool::string_or_bool")
        )]
        bool,
    ),
    Route,
}

#[cfg(test)]
mod test {
    use crate::NetplanConfig;

    #[test]
    fn yaml_booleans() {
        let input = r#"
            network:
              version: 2
              ethernets:
                nics:
                  match:
                    name: ens*
                  dhcp4: on
                  dhcp6: N
            "#;

        let netplan_config: NetplanConfig = serde_yaml::from_str(&input).unwrap();
        let ethernets = netplan_config.network.ethernets.unwrap();
        let ethernet = ethernets.values().next().unwrap();

        assert!(ethernet.common_all.is_some());

        let common = ethernet.common_all.as_ref().unwrap();

        assert_eq!(common.dhcp4, Some(true));
    }
}
