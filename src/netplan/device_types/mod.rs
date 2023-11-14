mod ethernets;
pub use ethernets::*;

mod modems;
pub use modems::*;

mod wifis;
pub use wifis::*;

mod bridges;
pub use bridges::*;

mod dummy_devices;
pub use dummy_devices::*;

mod bonds;
pub use bonds::*;

mod tunnels;
pub use tunnels::*;

mod vlans;
pub use vlans::*;

mod vrfs;
pub use vrfs::*;

mod nm_devices;
pub use nm_devices::*;

mod physical;
pub use physical::*;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "derive_builder")]
use derive_builder::Builder;

use crate::{
    AddressMapping, DhcpOverrides, Ipv6AddressGeneration, NameserverConfig, Renderer,
    RoutingConfig, RoutingPolicy,
};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct CommonPropertiesAllDevices {
    /// Use the given networking backend for this definition. Currently supported are
    /// networkd and NetworkManager. This property can be specified globally
    /// in network:, for a device type (in e. g. ethernets:) or
    /// for a particular device definition. Default is networkd.
    ///
    /// (Since 0.99) The renderer property has one additional acceptable value for vlan
    /// objects (i. e. defined in vlans:): sriov. If a vlan is defined with the
    /// sriov renderer for an SR-IOV Virtual Function interface, this causes netplan to
    /// set up a hardware VLAN filter for it. There can be only one defined per VF.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub renderer: Option<Renderer>,
    /// Enable DHCP for IPv4. Off by default.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub dhcp4: Option<bool>,
    /// Enable DHCP for IPv6. Off by default. This covers both stateless DHCP -
    /// where the DHCP server supplies information like DNS nameservers but not the
    /// IP address - and stateful DHCP, where the server provides both the address
    /// and the other information.
    ///
    /// If you are in an IPv6-only environment with completely stateless
    /// autoconfiguration (SLAAC with RDNSS), this option can be set to cause the
    /// interface to be brought up. (Setting accept-ra alone is not sufficient.)
    /// Autoconfiguration will still honour the contents of the router advertisement
    /// and only use DHCP if requested in the RA.
    ///
    /// Note that rdnssd(8) is required to use RDNSS with networkd. No extra
    /// software is required for NetworkManager.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub dhcp6: Option<bool>,
    /// Set the IPv6 MTU (only supported with networkd backend). Note
    /// that needing to set this is an unusual requirement.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ipv6_mtu: Option<u16>,
    /// Enable IPv6 Privacy Extensions (RFC 4941) for the specified interface, and
    /// prefer temporary addresses. Defaults to false - no privacy extensions. There
    /// is currently no way to have a private address but prefer the public address.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub ipv6_privacy: Option<bool>,
    /// Configure the link-local addresses to bring up. Valid options are ‘ipv4’
    /// and ‘ipv6’, which respectively allow enabling IPv4 and IPv6 link local
    /// addressing. If this field is not defined, the default is to enable only
    /// IPv6 link-local addresses. If the field is defined but configured as an
    /// empty set, IPv6 link-local addresses are disabled as well as IPv4 link-
    /// local addresses.
    ///
    /// This feature enables or disables link-local addresses for a protocol, but
    /// the actual implementation differs per backend. On networkd, this directly
    /// changes the behavior and may add an extra address on an interface. When
    /// using the NetworkManager backend, enabling link-local has no effect if the
    /// interface also has DHCP enabled.
    ///
    /// Example to enable only IPv4 link-local: `link-local: [ ipv4 ]`
    /// Example to enable all link-local addresses: `link-local: [ ipv4, ipv6 ]`
    /// Example to disable all link-local addresses: `link-local: [ ]`
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub link_local: Option<Vec<String>>,
    /// (networkd backend only) Allow the specified interface to be configured even
    /// if it has no carrier.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub ignore_carrier: Option<bool>,
    /// Designate the connection as “critical to the system”, meaning that special
    /// care will be taken by to not release the assigned IP when the daemon is
    /// restarted. (not recognized by NetworkManager)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub critical: Option<bool>,
    /// (networkd backend only) Sets the source of DHCPv4 client identifier. If mac
    /// is specified, the MAC address of the link is used. If this option is omitted,
    /// or if duid is specified, networkd will generate an RFC4361-compliant client
    /// identifier for the interface by combining the link’s IAID and DUID.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub dhcp_identifier: Option<String>,
    /// (networkd backend only) Overrides default DHCP behavior
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub dhcp4_overrides: Option<DhcpOverrides>,
    /// (networkd backend only) Overrides default DHCP behavior
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub dhcp6_overrides: Option<DhcpOverrides>,
    /// Accept Router Advertisement that would have the kernel configure IPv6 by itself.
    /// When enabled, accept Router Advertisements. When disabled, do not respond to
    /// Router Advertisements. If unset use the host kernel default setting.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub accept_ra: Option<bool>,
    /// Add static addresses to the interface in addition to the ones received
    /// through DHCP or RA. Each sequence entry is in CIDR notation, i. e. of the
    /// form addr/prefixlen. addr is an IPv4 or IPv6 address as recognized
    /// by inet_pton(3) and prefixlen the number of bits of the subnet.
    ///
    /// For virtual devices (bridges, bonds, vlan) if there is no address
    /// configured and DHCP is disabled, the interface may still be brought online,
    /// but will not be addressable from the network.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub addresses: Option<Vec<AddressMapping>>,
    /// Configure method for creating the address for use with RFC4862 IPv6
    /// Stateless Address Autoconfiguration (only supported with NetworkManager
    /// backend). Possible values are eui64 or stable-privacy.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ipv6_address_generation: Option<Ipv6AddressGeneration>,
    /// Define an IPv6 address token for creating a static interface identifier for
    /// IPv6 Stateless Address Autoconfiguration. This is mutually exclusive with
    /// ipv6-address-generation.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ipv6_address_token: Option<String>,
    /// Deprecated, see Default routes.
    /// Set default gateway for IPv4/6, for manual address configuration. This
    /// requires setting addresses too. Gateway IPs must be in a form
    /// recognized by inet_pton(3). There should only be a single gateway
    /// per IP address family set in your global config, to make it unambiguous.
    /// If you need multiple default routes, please define them via
    /// routing-policy.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub gateway4: Option<String>,
    /// Deprecated, see Default routes.
    /// Set default gateway for IPv4/6, for manual address configuration. This
    /// requires setting addresses too. Gateway IPs must be in a form
    /// recognized by inet_pton(3). There should only be a single gateway
    /// per IP address family set in your global config, to make it unambiguous.
    /// If you need multiple default routes, please define them via
    /// routing-policy.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub gateway6: Option<String>,
    /// Set DNS servers and search domains, for manual address configuration.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub nameservers: Option<NameserverConfig>,
    /// Set the device’s MAC address. The MAC address must be in the form
    /// “XX:XX:XX:XX:XX:XX”.
    ///
    /// Note: This will not work reliably for devices matched by name
    /// only and rendered by networkd, due to interactions with device
    /// renaming in udev. Match devices by MAC when setting MAC addresses.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub macaddress: Option<String>,
    /// Set the Maximum Transmission Unit for the interface. The default is 1500.
    /// Valid values depend on your network interface.
    ///
    /// Note: This will not work reliably for devices matched by name
    /// only and rendered by networkd, due to interactions with device
    /// renaming in udev. Match devices by MAC when setting MTU.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub mtu: Option<u16>,
    /// An optional device is not required for booting. Normally, networkd will
    /// wait some time for device to become configured before proceeding with
    /// booting. However, if a device is marked as optional, networkd will not wait
    /// for it. This is only supported by networkd, and the default is false.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub optional: Option<bool>,
    /// Specify types of addresses that are not required for a device to be
    /// considered online. This changes the behavior of backends at boot time to
    /// avoid waiting for addresses that are marked optional, and thus consider
    /// the interface as “usable” sooner. This does not disable these addresses,
    /// which will be brought up anyway.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub optional_addresses: Option<Vec<String>>,
    /// Allows specifying the management policy of the selected interface. By
    /// default, netplan brings up any configured interface if possible. Using the
    /// activation-mode setting users can override that behavior by either
    /// specifying manual, to hand over control over the interface state to the
    /// administrator or (for networkd backend only) off to force the link
    /// in a down state at all times. Any interface with activation-mode
    /// defined is implicitly considered optional.
    /// Supported officially as of networkd v248+.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub activation_mode: Option<ActivationMode>,
    /// Configure static routing for the device
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub routes: Option<Vec<RoutingConfig>>,
    /// Configure policy routing for the device
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub routing_policy: Option<Vec<RoutingPolicy>>,
}

/// Allows specifying the management policy of the selected interface. By
/// default, netplan brings up any configured interface if possible. Using the
/// activation-mode setting users can override that behavior by either
/// specifying manual, to hand over control over the interface state to the
/// administrator or (for networkd backend only) off to force the link
/// in a down state at all times. Any interface with activation-mode
/// defined is implicitly considered optional.
/// Supported officially as of networkd v248+.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename = "lowercase"))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum ActivationMode {
    Manual,
    Off,
}
