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
//! - `serde`: [Default] Add serde support
//! - `derive_builder` Enable the derive_builder crate for an automatically generated builder pattern API
//! - `repr-c` Add the `repr(C)` attribute to every type

use std::collections::HashMap;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[cfg(feature = "derive_builder")]
use derive_builder::Builder;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct NetplanConfig {
    pub network: NetworkConfig
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct NetworkConfig {
    pub version: u8,
    pub renderer: Option<Renderer>,
    pub ethernets: Option<HashMap<String, EthernetConfig>>,
    pub wifis: Option<HashMap<String, WifiConfig>>,
    pub bonds: Option<HashMap<String, BondConfig>>,
    pub bridges: Option<HashMap<String, BridgeConfig>>,
    pub vlans: Option<HashMap<String, VlanConfig>>,
    pub tunnels: Option<HashMap<String, TunnelConfig>>,
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
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum Renderer {
    #[cfg_attr(feature = "serde", serde(rename = "networkd"))]
    Networkd,
    #[cfg_attr(feature = "serde", serde(rename = "NetworkManager"))]
    NetworkManager,
    #[cfg_attr(feature = "serde", serde(rename = "sriov"))]
    Sriov,
}

/// Common properties for physical device types
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct CommonPropertiesPhysicalDeviceType {
    /// This selects a subset of available physical devices by various hardware
    /// properties. The following configuration will then apply to all matching
    /// devices, as soon as they appear. All specified properties must match.
    pub r#match: Option<MatchConfig>,
    /// When matching on unique properties such as path or MAC, or with additional
    /// assumptions such as “there will only ever be one wifi device”,
    /// match rules can be written so that they only match one device. Then this
    /// property can be used to give that device a more specific/desirable/nicer
    /// name than the default from udev’s ifnames. Any additional device that
    /// satisfies the match rules will then fail to get renamed and keep the
    /// original kernel name (and dmesg will show an error).
    pub set_name: Option<String>,
    /// Enable wake on LAN. Off by default.
    ///
    /// Note: This will not work reliably for devices matched by name
    /// only and rendered by networkd, due to interactions with device
    /// renaming in udev. Match devices by MAC when setting wake on LAN.
    pub wakeonlan: Option<bool>,
    /// (networkd backend only) Whether to emit LLDP packets. Off by default.
    pub emit_lldp: Option<bool>,
    /// (networkd backend only) If set to true, the hardware offload for
    /// checksumming of ingress network packets is enabled. When unset,
    /// the kernel’s default will be used.
    pub receive_checksum_offload: Option<bool>,
    /// (networkd backend only) If set to true, the hardware offload for
    /// checksumming of egress network packets is enabled. When unset,
    /// the kernel’s default will be used.
    pub transmit_checksum_offload: Option<bool>,
    /// (networkd backend only) If set to true, the TCP Segmentation
    /// Offload (TSO) is enabled. When unset, the kernel’s default will
    /// be used.
    pub tcp_segmentation_offload: Option<bool>,
    /// (networkd backend only) If set to true, the TCP6 Segmentation
    /// Offload (tx-tcp6-segmentation) is enabled. When unset, the
    /// kernel’s default will be used.
    pub tcp6_segmentation_offload: Option<bool>,
    /// (networkd backend only) If set to true, the Generic Segmentation
    /// Offload (GSO) is enabled. When unset, the kernel’s default will
    /// be used.
    pub generic_segmentation_offload: Option<bool>,
    /// (networkd backend only) If set to true, the Generic Receive
    /// Offload (GRO) is enabled. When unset, the kernel’s default will
    /// be used.
    pub generic_receive_offload: Option<bool>,
    /// (networkd backend only) If set to true, the Generic Receive
    /// Offload (GRO) is enabled. When unset, the kernel’s default will
    /// be used.
    pub large_receive_offload: Option<bool>,
    /// This provides additional configuration for the network device for openvswitch.
    /// If openvswitch is not available on the system, netplan treats the presence of
    /// openvswitch configuration as an error.
    ///
    /// Any supported network device that is declared with the openvswitch mapping
    /// (or any bond/bridge that includes an interface with an openvswitch configuration)
    /// will be created in openvswitch instead of the defined renderer.
    /// In the case of a vlan definition declared the same way, netplan will create
    /// a fake VLAN bridge in openvswitch with the requested vlan properties.
    pub openvswitch: Option<OpenVSwitchConfig>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
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
    pub renderer: Option<Renderer>,
    /// Enable DHCP for IPv4. Off by default.
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
    pub dhcp6: Option<bool>,
    /// Set the IPv6 MTU (only supported with networkd backend). Note
    /// that needing to set this is an unusual requirement.
    pub ipv6_mtu: Option<u16>,
    /// Enable IPv6 Privacy Extensions (RFC 4941) for the specified interface, and
    /// prefer temporary addresses. Defaults to false - no privacy extensions. There
    /// is currently no way to have a private address but prefer the public address.
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
    /// Example to enable only IPv4 link-local: link-local: [ ipv4 ]
    /// Example to enable all link-local addresses: link-local: [ ipv4, ipv6 ]
    /// Example to disable all link-local addresses: link-local: [ ]
    pub link_local: Option<Vec<String>>,
    /// (networkd backend only) Allow the specified interface to be configured even
    /// if it has no carrier.
    pub ignore_carrier: Option<bool>,
    /// Designate the connection as “critical to the system”, meaning that special
    /// care will be taken by to not release the assigned IP when the daemon is
    /// restarted. (not recognized by NetworkManager)
    pub critical: Option<bool>,
    /// (networkd backend only) Sets the source of DHCPv4 client identifier. If mac
    /// is specified, the MAC address of the link is used. If this option is omitted,
    /// or if duid is specified, networkd will generate an RFC4361-compliant client
    /// identifier for the interface by combining the link’s IAID and DUID.
    pub dhcp_identifier: Option<String>,
    /// (networkd backend only) Overrides default DHCP behavior
    pub dhcp4_overrides: Option<DhcpOverrides>,
    /// (networkd backend only) Overrides default DHCP behavior
    pub dhcp6_overrides: Option<DhcpOverrides>,
    /// Accept Router Advertisement that would have the kernel configure IPv6 by itself.
    /// When enabled, accept Router Advertisements. When disabled, do not respond to
    /// Router Advertisements. If unset use the host kernel default setting.
    pub accept_ra: Option<bool>,
    /// Add static addresses to the interface in addition to the ones received
    /// through DHCP or RA. Each sequence entry is in CIDR notation, i. e. of the
    /// form addr/prefixlen. addr is an IPv4 or IPv6 address as recognized
    /// by inet_pton(3) and prefixlen the number of bits of the subnet.
    ///
    /// For virtual devices (bridges, bonds, vlan) if there is no address
    /// configured and DHCP is disabled, the interface may still be brought online,
    /// but will not be addressable from the network.
    pub addresses: Option<Vec<AddressMapping>>,
    /// Configure method for creating the address for use with RFC4862 IPv6
    /// Stateless Address Autoconfiguration (only supported with NetworkManager
    /// backend). Possible values are eui64 or stable-privacy.
    pub ipv6_address_generation: Option<Ipv6AddressGeneration>,
    /// Define an IPv6 address token for creating a static interface identifier for
    /// IPv6 Stateless Address Autoconfiguration. This is mutually exclusive with
    /// ipv6-address-generation.
    pub ipv6_address_token: Option<String>,
    /// Deprecated, see Default routes.
    /// Set default gateway for IPv4/6, for manual address configuration. This
    /// requires setting addresses too. Gateway IPs must be in a form
    /// recognized by inet_pton(3). There should only be a single gateway
    /// per IP address family set in your global config, to make it unambiguous.
    /// If you need multiple default routes, please define them via
    /// routing-policy.
    pub gateway4: Option<String>,
    /// Deprecated, see Default routes.
    /// Set default gateway for IPv4/6, for manual address configuration. This
    /// requires setting addresses too. Gateway IPs must be in a form
    /// recognized by inet_pton(3). There should only be a single gateway
    /// per IP address family set in your global config, to make it unambiguous.
    /// If you need multiple default routes, please define them via
    /// routing-policy.
    pub gateway6: Option<String>,
    /// Set DNS servers and search domains, for manual address configuration.
    pub nameservers: Option<NameserverConfig>,
    /// Set the device’s MAC address. The MAC address must be in the form
    /// “XX:XX:XX:XX:XX:XX”.
    ///
    /// Note: This will not work reliably for devices matched by name
    /// only and rendered by networkd, due to interactions with device
    /// renaming in udev. Match devices by MAC when setting MAC addresses.
    pub macaddress: Option<String>,
    /// Set the Maximum Transmission Unit for the interface. The default is 1500.
    /// Valid values depend on your network interface.
    ///
    /// Note: This will not work reliably for devices matched by name
    /// only and rendered by networkd, due to interactions with device
    /// renaming in udev. Match devices by MAC when setting MTU.
    pub mtu: u16,
    /// An optional device is not required for booting. Normally, networkd will
    /// wait some time for device to become configured before proceeding with
    /// booting. However, if a device is marked as optional, networkd will not wait
    /// for it. This is only supported by networkd, and the default is false.
    pub optional: Option<bool>,
    /// Specify types of addresses that are not required for a device to be
    /// considered online. This changes the behavior of backends at boot time to
    /// avoid waiting for addresses that are marked optional, and thus consider
    /// the interface as “usable” sooner. This does not disable these addresses,
    /// which will be brought up anyway.
    pub optional_addresses: Option<Vec<String>>,
    /// Allows specifying the management policy of the selected interface. By
    /// default, netplan brings up any configured interface if possible. Using the
    /// activation-mode setting users can override that behavior by either
    /// specifying manual, to hand over control over the interface state to the
    /// administrator or (for networkd backend only) off to force the link
    /// in a down state at all times. Any interface with activation-mode
    /// defined is implicitly considered optional.
    /// Supported officially as of networkd v248+.
    pub activation_mode: Option<ActivationMode>,
    /// Configure static routing for the device
    pub routes: Option<Vec<RoutingConfig>>,
    /// Configure policy routing for the device
    pub routing_policy: Option<Vec<RoutingPolicy>>,
}

/// Several DHCP behavior overrides are available. Most currently only have any
/// effect when using the networkd backend, with the exception of use-routes
/// and route-metric.
///
/// Overrides only have an effect if the corresponding dhcp4 or dhcp6 is
/// set to true.
///
/// If both dhcp4 and dhcp6 are true, the networkd backend requires
/// that dhcp4-overrides and dhcp6-overrides contain the same keys and
/// values. If the values do not match, an error will be shown and the network
/// configuration will not be applied.
///
/// When using the NetworkManager backend, different values may be specified for
/// dhcp4-overrides and dhcp6-overrides, and will be applied to the DHCP
/// client processes as specified in the netplan YAML.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct DhcpOverrides {
    /// Default: true. When true, the DNS servers received from the
    /// DHCP server will be used and take precedence over any statically
    /// configured ones. Currently only has an effect on the networkd
    /// backend.
    pub use_dns: Option<bool>,
    /// Default: true. When true, the NTP servers received from the
    /// DHCP server will be used by systemd-timesyncd and take precedence
    /// over any statically configured ones. Currently only has an effect on
    /// the networkd backend.
    pub use_ntp: Option<bool>,
    /// Default: true. When true, the machine’s hostname will be sent
    /// to the DHCP server. Currently only has an effect on the networkd
    /// backend.
    pub send_hostname: Option<bool>,
    /// Default: true. When true, the hostname received from the DHCP
    /// server will be set as the transient hostname of the system. Currently
    /// only has an effect on the networkd backend.
    pub use_hostname: Option<bool>,
    /// Default: true. When true, the MTU received from the DHCP
    /// server will be set as the MTU of the network interface. When false,
    /// the MTU advertised by the DHCP server will be ignored. Currently only
    /// has an effect on the networkd backend.
    pub use_mtu: Option<bool>,
    /// Use this value for the hostname which is sent to the DHCP server,
    /// instead of machine’s hostname. Currently only has an effect on the
    /// networkd backend.
    pub hostname: Option<String>,
    /// Default: true. When true, the routes received from the DHCP
    /// server will be installed in the routing table normally. When set to
    /// false, routes from the DHCP server will be ignored: in this case,
    /// the user is responsible for adding static routes if necessary for
    /// correct network operation. This allows users to avoid installing a
    /// default gateway for interfaces configured via DHCP. Available for
    /// both the networkd and NetworkManager backends.
    pub use_routes: Option<bool>,
    /// Use this value for default metric for automatically-added routes.
    /// Use this to prioritize routes for devices by setting a lower metric
    /// on a preferred interface. Available for both the networkd and
    /// NetworkManager backends.
    pub route_metric: Option<u16>,
    /// Takes a boolean, or the special value “route”. When true, the domain
    /// name received from the DHCP server will be used as DNS search domain
    /// over this link, similar to the effect of the Domains= setting. If set
    /// to “route”, the domain name received from the DHCP server will be
    /// used for routing DNS queries only, but not for searching, similar to
    /// the effect of the Domains= setting when the argument is prefixed with
    /// “~”.
    pub use_domains: Option<String>
}

/// The routes block defines standard static routes for an interface.
/// At least to must be specified. If type is local or nat a
/// default scope of host is assumed.
/// If type is unicast and no gateway (via) is given or type is
/// broadcast, multicast or anycast a default scope of link
/// is assumend. Otherwise, a global scope is the default setting.
///
/// For from, to, and via, both IPv4 and IPv6 addresses are
/// recognized, and must be in the form addr/prefixlen or addr.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct RoutingConfig {
    /// Set a source IP address for traffic going through the route.
    /// (NetworkManager: as of v1.8.0)
    pub from: Option<String>,
    /// Destination address for the route.
    pub to: Option<String>,
    /// Address to the gateway to use for this route.
    pub via: Option<String>,
    /// When set to “true”, specifies that the route is directly connected
    /// to the interface.
    /// (NetworkManager: as of v1.12.0 for IPv4 and v1.18.0 for IPv6)
    pub on_link: Option<bool>,
    /// The relative priority of the route. Must be a positive integer value.
    pub metric: Option<u16>,
    /// The type of route. Valid options are “unicast” (default), “anycast”,
    /// “blackhole”, “broadcast”, “local”, “multicast”, “nat”, “prohibit”,
    /// “throw”, “unreachable” or “xresolve”.
    pub r#type: Option<RouteType>,
    /// The route scope, how wide-ranging it is to the network. Possible
    /// values are “global”, “link”, or “host”.
    pub scope: Option<RouteScope>,
    /// The table number to use for the route. In some scenarios, it may be
    /// useful to set routes in a separate routing table. It may also be used
    /// to refer to routing policy rules which also accept a table
    /// parameter. Allowed values are positive integers starting from 1.
    /// Some values are already in use to refer to specific routing tables:
    /// see /etc/iproute2/rt_tables.
    /// (NetworkManager: as of v1.10.0)
    pub table: Option<u16>,
    /// The MTU to be used for the route, in bytes. Must be a positive integer
    /// value.
    pub mtu: Option<u16>,
    /// The congestion window to be used for the route, represented by number
    /// of segments. Must be a positive integer value.
    pub congestion_window: Option<u16>,
    /// The receive window to be advertised for the route, represented by
    /// number of segments. Must be a positive integer value.
    pub advertised_receive_window: Option<u16>
}

/// The type of route. Valid options are “unicast” (default), “anycast”,
/// “blackhole”, “broadcast”, “local”, “multicast”, “nat”, “prohibit”,
/// “throw”, “unreachable” or “xresolve”.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum RouteType {
    Unicast,
    Anycast,
    Blackhole,
    Broadcast,
    Local,
    Multicast,
    Nat,
    Prohibit,
    Throw,
    Unreachable,
    Xresolve,
}

/// The route scope, how wide-ranging it is to the network. Possible
/// values are “global”, “link”, or “host”.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum RouteScope {
    Global,
    Link,
    Host,
}

/// The routing-policy block defines extra routing policy for a network,
/// where traffic may be handled specially based on the source IP, firewall
/// marking, etc.
///
/// For from, to, both IPv4 and IPv6 addresses are recognized, and
/// must be in the form addr/prefixlen or addr.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct RoutingPolicy {
    /// Set a source IP address to match traffic for this policy rule.
    pub from: Option<String>,
    /// Match on traffic going to the specified destination.
    pub to: Option<String>,
    /// The table number to match for the route. In some scenarios, it may be
    /// useful to set routes in a separate routing table. It may also be used
    /// to refer to routes which also accept a table parameter.
    /// Allowed values are positive integers starting from 1.
    /// Some values are already in use to refer to specific routing tables:
    /// see /etc/iproute2/rt_tables.
    pub table: u16,
    /// Specify a priority for the routing policy rule, to influence the order
    /// in which routing rules are processed. A higher number means lower
    /// priority: rules are processed in order by increasing priority number.
    pub priority: Option<i32>,
    /// Have this routing policy rule match on traffic that has been marked
    /// by the iptables firewall with this value. Allowed values are positive
    /// integers starting from 1.
    pub mark: Option<u16>,
    /// Match this policy rule based on the type of service number applied to
    /// the traffic.
    pub type_of_service: Option<String>
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
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum UseDomains {
    Boolean(bool),
    Route,
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
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum ActivationMode {
    Manual,
    Off,
}

/// Set DNS servers and search domains, for manual address configuration.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct NameserverConfig {
    /// A list of IPv4 or IPv6 addresses
    pub addresses: Option<Vec<String>>,
    /// A list of search domains.
    pub search: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum Ipv6AddressGeneration {
    #[cfg_attr(feature = "serde", serde(rename = "eui64"))]
    Eui64,
    #[cfg_attr(feature = "serde", serde(rename = "stable-privacy"))]
    StablePrivacy
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum AddressMapping {
    Simple(String),
    Complex {
        /// Default: forever. This can be forever or 0 and corresponds
        /// to the PreferredLifetime option in systemd-networkd’s Address
        /// section. Currently supported on the networkd backend only.
        lifetime: PreferredLifetime,
        /// An IP address label, equivalent to the ip address label
        /// command. Currently supported on the networkd backend only.
        label: String,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum PreferredLifetime {
    #[cfg_attr(feature = "serde", serde(rename = "forever"))]
    Forever,
    #[cfg_attr(feature = "serde", serde(rename = "0"))]
    Zero,
}

/// Netplan supports advanced authentication settings for ethernet and wifi
/// interfaces, as well as individual wifi networks, by means of the auth block.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct AuthConfig {
    /// The supported key management modes are none (no key management);
    /// psk (WPA with pre-shared key, common for home wifi); eap (WPA
    /// with EAP, common for enterprise wifi); and 802.1x (used primarily
    /// for wired Ethernet connections).
    pub key_managment: Option<KeyManagmentMode>,
    /// The password string for EAP, or the pre-shared key for WPA-PSK.
    pub password: Option<String>,
    /// The EAP method to use. The supported EAP methods are tls (TLS),
    /// peap (Protected EAP), and ttls (Tunneled TLS).
    pub method: Option<AuthMethod>,
    /// The identity to use for EAP.
    pub identity: Option<String>,
    /// The identity to pass over the unencrypted channel if the chosen EAP
    /// method supports passing a different tunnelled identity.
    pub anonymous_identity: Option<String>,
    /// Path to a file with one or more trusted certificate authority (CA)
    /// certificates.
    pub ca_certificate: Option<String>,
    /// Path to a file containing the certificate to be used by the client
    /// during authentication.
    pub client_certificate: Option<String>,
    /// Path to a file containing the private key corresponding to
    /// client-certificate.
    pub client_key: Option<String>,
    /// Password to use to decrypt the private key specified in
    /// client-key if it is encrypted.
    pub client_key_password: Option<String>,
    /// Phase 2 authentication mechanism.
    pub phase2_auth: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum AuthMethod {
    #[cfg_attr(feature = "serde", serde(rename = "tls"))]
    Tls,
    #[cfg_attr(feature = "serde", serde(rename = "peap"))]
    Peap,
    #[cfg_attr(feature = "serde", serde(rename = "ttls"))]
    Ttls
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum KeyManagmentMode {
    #[cfg_attr(feature = "serde", serde(rename = "none"))]
    None,
    #[cfg_attr(feature = "serde", serde(rename = "psk"))]
    Psk,
    #[cfg_attr(feature = "serde", serde(rename = "eap"))]
    Eap,
    /// 802.1x
    #[cfg_attr(feature = "serde", serde(rename = "802.1x"))]
    EightZeroTwoDotOneX
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct EthernetConfig {
    /// (SR-IOV devices only) The link property declares the device as a
    /// Virtual Function of the selected Physical Function device, as identified
    /// by the given netplan id.
    pub link: Option<String>,
    /// (SR-IOV devices only) In certain special cases VFs might need to be
    /// configured outside of netplan. For such configurations virtual-function-count
    /// can be optionally used to set an explicit number of Virtual Functions for
    /// the given Physical Function. If unset, the default is to create only as many
    /// VFs as are defined in the netplan configuration. This should be used for special
    /// cases only.
    pub virtual_function_count: Option<u16>,
    /// (SR-IOV devices only) Change the operational mode of the embedded switch
    /// of a supported SmartNIC PCI device (e.g. Mellanox ConnectX-5). Possible
    /// values are switchdev or legacy, if unspecified the vendor’s
    /// default configuration is used.
    pub embedded_switch_mode: Option<EmbeddedSwitchMode>,
    /// (SR-IOV devices only) Delay rebinding of SR-IOV virtual functions to its
    /// driver after changing the embedded-switch-mode setting to a later stage.
    /// Can be enabled when bonding/VF LAG is in use. Defaults to false.
    pub delay_virtual_functions_rebind: Option<bool>,
    /// Common properties for physical device types
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub common_physical: Option<CommonPropertiesPhysicalDeviceType>,
    /// Common properties for all devices
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub common_all: Option<CommonPropertiesAllDevices>,
}

/// GSM/CDMA modem configuration is only supported for the NetworkManager
/// backend. systemd-networkd does not support modems.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct ModemConfig {
    /// Set the carrier APN (Access Point Name). This can be omitted if
    /// auto-config is enabled.
    pub apn: Option<String>,
    /// Specify whether to try and autoconfigure the modem by doing a lookup of
    /// the carrier against the Mobile Broadband Provider database. This may not
    /// work for all carriers.
    pub auto_config: Option<bool>,
    /// Specify the device ID (as given by the WWAN management service) of the
    /// modem to match. This can be found using mmcli.
    pub device_id: Option<String>,
    /// Specify the Network ID (GSM LAI format). If this is specified, the device
    /// will not roam networks.
    pub network_id: Option<String>,
    /// The number to dial to establish the connection to the mobile broadband
    /// network. (Deprecated for GSM)
    pub number: Option<String>,
    /// Specify the password used to authenticate with the carrier network. This
    /// can be omitted if auto-config is enabled.
    pub password: Option<String>,
    /// Specify the SIM PIN to allow it to operate if a PIN is set.
    pub pin: Option<String>,
    /// Specify the SIM unique identifier (as given by the WWAN management service)
    /// which this connection applies to. If given, the connection will apply to
    /// any device also allowed by device-id which contains a SIM card matching
    /// the given identifier.
    pub sim_id: Option<String>,
    /// Specify the MCC/MNC string (such as “310260” or “21601”) which identifies
    /// the carrier that this connection should apply to. If given, the connection
    /// will apply to any device also allowed by device-id and sim-id
    /// which contains a SIM card provisioned by the given operator.
    pub sim_operator_id: Option<String>,
    /// Specify the username used to authentiate with the carrier network. This
    /// can be omitted if auto-config is enabled.
    pub username: Option<String>,
    /// Common properties for physical device types
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub common_physical: Option<CommonPropertiesPhysicalDeviceType>,
    /// Common properties for all devices
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub common_all: Option<CommonPropertiesAllDevices>,
}

/// This provides additional configuration for the network device for openvswitch.
/// If openvswitch is not available on the system, netplan treats the presence of
/// openvswitch configuration as an error.
///
/// Any supported network device that is declared with the openvswitch mapping
/// (or any bond/bridge that includes an interface with an openvswitch configuration)
/// will be created in openvswitch instead of the defined renderer.
/// In the case of a vlan definition declared the same way, netplan will create
/// a fake VLAN bridge in openvswitch with the requested vlan properties.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct OpenVSwitchConfig {
    /// Passed-through directly to OpenVSwitch
    pub external_ids: Option<String>,
    /// Passed-through directly to OpenVSwitch
    pub other_config: Option<String>,
    /// Valid for bond interfaces. Accepts active, passive or off (the default).
    pub lacp: Option<Lacp>,
    /// Valid for bridge interfaces. Accepts secure or standalone (the default).
    pub fail_mode: Option<FailMode>,
    /// Valid for bridge interfaces. False by default.
    pub mcast_snooping: Option<bool>,
    /// Valid for bridge interfaces or the network section. List of protocols to be used when
    /// negotiating a connection with the controller. Accepts OpenFlow10, OpenFlow11,
    /// OpenFlow12, OpenFlow13, OpenFlow14, OpenFlow15 and OpenFlow16.
    pub protocols: Option<Vec<OpenFlowProtocol>>,
    /// Valid for bridge interfaces. False by default.
    pub rtsp: Option<bool>,
    /// Valid for bridge interfaces. Specify an external OpenFlow controller.
    pub controller: Option<ControllerConfig>,
    /// OpenvSwitch patch ports. Each port is declared as a pair of names
    /// which can be referenced as interfaces in dependent virtual devices
    /// (bonds, bridges).
    pub ports: Option<Vec<String>>,
    /// Valid for global openvswitch settings. Options for configuring SSL
    /// server endpoint for the switch.
    pub ssl: Option<SslConfig>,
}

/// Valid for global openvswitch settings. Options for configuring SSL
/// server endpoint for the switch.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct SslConfig {
    /// Path to a file containing the CA certificate to be used.
    pub ca_cert: Option<String>,
    /// Path to a file containing the server certificate.
    pub certificate: Option<String>,
    /// Path to a file containing the private key for the server.
    pub private_key: Option<String>,
}

/// Valid for bridge interfaces. Specify an external OpenFlow controller.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct ControllerConfig {
    /// Set the list of addresses to use for the controller targets. The
    /// syntax of these addresses is as defined in ovs-vsctl(8). Example:
    /// addresses: [tcp:127.0.0.1:6653, "ssl:[fe80::1234%eth0]:6653"]
    pub addresses: Option<Vec<String>>,
    /// Set the connection mode for the controller. Supported options are
    /// in-band and out-of-band. The default is in-band.
    pub connection_mode: Option<ConnectionMode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum ConnectionMode {
    InBand,
    OutOfBand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum OpenFlowProtocol {
    OpenFlow10,
    OpenFlow11,
    OpenFlow12,
    OpenFlow13,
    OpenFlow14,
    OpenFlow15,
    OpenFlow16,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum Lacp {
    Active,
    Passive,
    Off,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum FailMode {
    Secure,
    Standalone,
}

/// This selects a subset of available physical devices by various hardware
/// properties. The following configuration will then apply to all matching
/// devices, as soon as they appear. All specified properties must match.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct MatchConfig {
    /// Current interface name. Globs are supported, and the primary use case
    /// for matching on names, as selecting one fixed name can be more easily
    /// achieved with having no match: at all and just using the ID (see
    /// above).
    /// (NetworkManager: as of v1.14.0)
    pub name: Option<String>,
    /// Device’s MAC address in the form “XX:XX:XX:XX:XX:XX”. Globs are not
    /// allowed.
    pub macaddress: Option<String>,
    /// Kernel driver name, corresponding to the DRIVER udev property.
    /// A sequence of globs is supported, any of which must match.
    /// Matching on driver is only supported with networkd.
    pub driver: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum EmbeddedSwitchMode {
    Switchdev,
    Legacy,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct WifiConfig {
    /// This provides pre-configured connections to NetworkManager. Note that
    /// users can of course select other access points/SSIDs. The keys of the
    /// mapping are the SSIDs, and the values are mappings with the following
    /// supported properties:
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub access_points: Option<HashMap<String, AccessPointConfig>>,
    /// This enables WakeOnWLan on supported devices. Not all drivers support all
    /// options. May be any combination of any, disconnect, magic_pkt,
    /// gtk_rekey_failure, eap_identity_req, four_way_handshake,
    /// rfkill_release or tcp (NetworkManager only). Or the exclusive
    /// default flag (the default).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub wakeonwlan: Option<Vec<WakeOnWLan>>,
    /// Common properties for physical device types
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub common_physical: Option<CommonPropertiesPhysicalDeviceType>,
    /// Common properties for all devices
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub common_all: Option<CommonPropertiesAllDevices>,
}

/// This enables WakeOnWLan on supported devices. Not all drivers support all
/// options. May be any combination of any, disconnect, magic_pkt,
/// gtk_rekey_failure, eap_identity_req, four_way_handshake,
/// rfkill_release or tcp (NetworkManager only). Or the exclusive
/// default flag (the default).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum WakeOnWLan {
    #[cfg_attr(feature = "serde", serde(rename = "any"))]
    Any,
    #[cfg_attr(feature = "serde", serde(rename = "disconnect"))]
    Disconnect,
    #[cfg_attr(feature = "serde", serde(rename = "magic_pkt"))]
    MagicPkt,
    #[cfg_attr(feature = "serde", serde(rename = "gtk_rekey_failure"))]
    GtkRekeyFailure,
    #[cfg_attr(feature = "serde", serde(rename = "eap_identity_req"))]
    EapIdentityReq,
    #[cfg_attr(feature = "serde", serde(rename = "four_way_handshake"))]
    FourWayHandshake,
    #[cfg_attr(feature = "serde", serde(rename = "rfkill_release"))]
    RfkillRelease,
    #[cfg_attr(feature = "serde", serde(rename = "tcp"))]
    Tcp,
    #[cfg_attr(feature = "serde", serde(rename = "default"))]
    Default,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct AccessPointConfig {
    /// Enable WPA2 authentication and set the passphrase for it. If neither
    /// this nor an auth block are given, the network is assumed to be
    /// open. The setting
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub password: Option<String>,
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub auth: Option<AuthConfig>,
    /// Possible access point modes are infrastructure (the default),
    /// ap (create an access point to which other devices can connect),
    /// and adhoc (peer to peer networks without a central access point).
    /// ap is only supported with NetworkManager.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub mode: Option<AccessPointMode>,
    /// If specified, directs the device to only associate with the given
    /// access point.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub bssid: Option<String>,
    /// Possible bands are 5GHz (for 5GHz 802.11a) and 2.4GHz
    /// (for 2.4GHz 802.11), do not restrict the 802.11 frequency band of the
    /// network if unset (the default).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub band: Option<WirelessBand>,
    /// Wireless channel to use for the Wi-Fi connection. Because channel
    /// numbers overlap between bands, this property takes effect only if
    /// the band property is also set.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub channel: Option<u32>,
    /// Set to true to change the SSID scan technique for connecting to
    /// hidden WiFi networks. Note this may have slower performance compared
    /// to false (the default) when connecting to publicly broadcast
    /// SSIDs.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub hidden: Option<bool>,
}

/// Possible bands are 5GHz (for 5GHz 802.11a) and 2.4GHz
/// (for 2.4GHz 802.11), do not restrict the 802.11 frequency band of the
/// network if unset (the default).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum WirelessBand {
    /// 2.4Ghz
    #[cfg_attr(feature = "serde", serde(rename = "2.4GHz"))]
    Ghz2,
    /// 5Ghz
    #[cfg_attr(feature = "serde", serde(rename = "5GHz"))]
    Ghz5,
}

/// Possible access point modes are infrastructure (the default),
/// ap (create an access point to which other devices can connect),
/// and adhoc (peer to peer networks without a central access point).
/// ap is only supported with NetworkManager.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum AccessPointMode {
    #[cfg_attr(feature = "serde", serde(rename = "infrastructure"))]
    Infrastructure,
    #[cfg_attr(feature = "serde", serde(rename = "ap"))]
    Ap,
    #[cfg_attr(feature = "serde", serde(rename = "adhoc"))]
    Adhoc,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct BondConfig {
    /// All devices matching this ID list will be added to the bond.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub interfaces: Option<Vec<String>>,
    /// Customization parameters for special bonding options. Time intervals
    /// may need to be expressed as a number of seconds or milliseconds: the
    /// default value type is specified below. If necessary, time intervals can
    /// be qualified using a time suffix (such as “s” for seconds, “ms” for
    /// milliseconds) to allow for more control over its behavior.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub parameters: Option<BondParameters>,
    /// Common properties for all devices
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub common_all: Option<CommonPropertiesAllDevices>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct BondParameters {
    /// Set the bonding mode used for the interfaces. The default is
    /// balance-rr (round robin). Possible values are balance-rr,
    /// active-backup, balance-xor, broadcast, 802.3ad,
    /// balance-tlb, and balance-alb.
    /// For OpenVSwitch active-backup and the additional modes
    /// balance-tcp and balance-slb are supported.
    /// #[serde(skip_serializing_if = "Option
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub mode: Option<BondMode>,
    /// Set the rate at which LACPDUs are transmitted. This is only useful
    /// in 802.3ad mode. Possible values are slow (30 seconds, default),
    /// and fast (every second).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub lacp_rate: Option<LacpRate>,
    /// Specifies the interval for MII monitoring (verifying if an interface
    /// of the bond has carrier). The default is 0; which disables MII
    /// monitoring. This is equivalent to the MIIMonitorSec= field for the
    /// networkd backend. If no time suffix is specified, the value will be
    /// interpreted as milliseconds.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub mii_monitor_interval: Option<String>,
    /// The minimum number of links up in a bond to consider the bond
    /// interface to be up.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub min_links: Option<u16>,
    /// Specifies the transmit hash policy for the selection of slaves. This
    /// is only useful in balance-xor, 802.3ad and balance-tlb modes.
    /// Possible values are layer2, layer3+4, layer2+3,
    /// encap2+3, and encap3+4.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub transmit_hash_policy: Option<TransmitHashPolicy>,
    /// Set the aggregation selection mode. Possible values are stable,
    /// bandwidth, and count. This option is only used in 802.3ad
    /// mode.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ad_select: Option<AdSelect>,
    /// If the bond should drop duplicate frames received on inactive ports,
    /// set this option to false. If they should be delivered, set this
    /// option to true. The default value is false, and is the desirable
    /// behavior in most situations.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub all_slaves_active: Option<bool>,
    /// Set the interval value for how frequently ARP link monitoring should
    /// happen. The default value is 0, which disables ARP monitoring.
    /// For the networkd backend, this maps to the ARPIntervalSec= property.
    /// If no time suffix is specified, the value will be interpreted as
    /// milliseconds.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub arp_interval: Option<String>,
    /// IPs of other hosts on the link which should be sent ARP requests in
    /// order to validate that a slave is up. This option is only used when
    /// arp-interval is set to a value other than 0. At least one IP
    /// address must be given for ARP link monitoring to function. Only IPv4
    /// addresses are supported. You can specify up to 16 IP addresses. The
    /// default value is an empty list.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub arp_ip_targets: Option<Vec<String>>,
    /// Configure how ARP replies are to be validated when using ARP link
    /// monitoring. Possible values are none, active, backup,
    /// and all.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub arp_validate: Option<ArpValidate>,
    /// Specify whether to use any ARP IP target being up as sufficient for
    /// a slave to be considered up; or if all the targets must be up. This
    /// is only used for active-backup mode when arp-validate is
    /// enabled. Possible values are any and all.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub arp_all_targets: Option<ArpAllTargets>,
    /// Specify the delay before enabling a link once the link is physically
    /// up. The default value is 0. This maps to the UpDelaySec= property
    /// for the networkd renderer. This option is only valid for the miimon
    /// link monitor. If no time suffix is specified, the value will be
    /// interpreted as milliseconds.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub up_delay: Option<String>,
    /// Specify the delay before disabling a link once the link has been
    /// lost. The default value is 0. This maps to the DownDelaySec=
    /// property for the networkd renderer. This option is only valid for the
    /// miimon link monitor. If no time suffix is specified, the value will
    /// be interpreted as milliseconds.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub down_delay: Option<String>,
    /// Set whether to set all slaves to the same MAC address when adding
    /// them to the bond, or how else the system should handle MAC addresses.
    /// The possible values are none, active, and follow.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub fail_over_mac_policy: Option<FailOverMacPolicy>,
    /// Specify how many ARP packets to send after failover. Once a link is
    /// up on a new slave, a notification is sent and possibly repeated if
    /// this value is set to a number greater than 1. The default value
    /// is 1 and valid values are between 1 and 255. This only
    /// affects active-backup mode.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub gratuitous_arp: Option<u8>,
    /// In balance-rr mode, specifies the number of packets to transmit
    /// on a slave before switching to the next. When this value is set to
    /// 0, slaves are chosen at random. Allowable values are between
    /// 0 and 65535. The default value is 1. This setting is
    /// only used in balance-rr mode.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub packets_per_slave: Option<u32>,
    /// Set the reselection policy for the primary slave. On failure of the
    /// active slave, the system will use this policy to decide how the new
    /// active slave will be chosen and how recovery will be handled. The
    /// possible values are always, better, and failure.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub primary_reselect_policy: Option<PrimaryReselectPolicy>,
    /// In modes balance-rr, active-backup, balance-tlb and
    /// balance-alb, a failover can switch IGMP traffic from one
    /// slave to another.
    ///
    /// This parameter specifies how many IGMP membership reports
    /// are issued on a failover event. Values range from 0 to 255. 0
    /// disables sending membership reports. Otherwise, the first
    /// membership report is sent on failover and subsequent reports
    /// are sent at 200ms intervals.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub resend_igmp: Option<u8>,
    /// Specify the interval between sending learning packets to
    /// each slave. The value range is between 1 and 0x7fffffff.
    /// The default value is 1. This option only affects balance-tlb
    /// and balance-alb modes. Using the networkd renderer, this field
    /// maps to the LearnPacketIntervalSec= property. If no time suffix is
    /// specified, the value will be interpreted as seconds.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub learn_packet_interval: Option<String>,
    /// Specify a device to be used as a primary slave, or preferred device
    /// to use as a slave for the bond (ie. the preferred device to send
    /// data through), whenever it is available. This only affects
    /// active-backup, balance-alb, and balance-tlb modes.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub primary: Option<String>,
}

/// Set the reselection policy for the primary slave. On failure of the
/// active slave, the system will use this policy to decide how the new
/// active slave will be chosen and how recovery will be handled. The
/// possible values are always, better, and failure.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum PrimaryReselectPolicy {
    #[cfg_attr(feature = "serde", serde(rename = "always"))]
    Always,
    #[cfg_attr(feature = "serde", serde(rename = "better"))]
    Better,
    #[cfg_attr(feature = "serde", serde(rename = "failure"))]
    Failure
}

/// Set whether to set all slaves to the same MAC address when adding
/// them to the bond, or how else the system should handle MAC addresses.
/// The possible values are none, active, and follow.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum FailOverMacPolicy {
    #[cfg_attr(feature = "serde", serde(rename = "none"))]
    None,
    #[cfg_attr(feature = "serde", serde(rename = "activv"))]
    Active,
    #[cfg_attr(feature = "serde", serde(rename = "follow"))]
    Follow
}

/// Specify whether to use any ARP IP target being up as sufficient for
/// a slave to be considered up; or if all the targets must be up. This
/// is only used for active-backup mode when arp-validate is
/// enabled. Possible values are any and all.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum ArpAllTargets {
    #[cfg_attr(feature = "serde", serde(rename = "any"))]
    Any,
    #[cfg_attr(feature = "serde", serde(rename = "all"))]
    All
}

/// Configure how ARP replies are to be validated when using ARP link
/// monitoring. Possible values are none, active, backup,
/// and all.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum ArpValidate {
    #[cfg_attr(feature = "serde", serde(rename = "none"))]
    None,
    #[cfg_attr(feature = "serde", serde(rename = "active"))]
    Active,
    #[cfg_attr(feature = "serde", serde(rename = "backup"))]
    Backup,
    #[cfg_attr(feature = "serde", serde(rename = "all"))]
    All
}

/// Set the aggregation selection mode. Possible values are stable,
/// bandwidth, and count. This option is only used in 802.3ad
/// mode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum AdSelect {
    #[cfg_attr(feature = "serde", serde(rename = "stable"))]
    Stable,
    #[cfg_attr(feature = "serde", serde(rename = "bandwidth"))]
    Bandwidth,
    #[cfg_attr(feature = "serde", serde(rename = "count"))]
    Count
}

/// Specifies the transmit hash policy for the selection of slaves. This
/// is only useful in balance-xor, 802.3ad and balance-tlb modes.
/// Possible values are layer2, layer3+4, layer2+3,
/// encap2+3, and encap3+4.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum TransmitHashPolicy {
    #[cfg_attr(feature = "serde", serde(rename = "layer2"))]
    Layer2,
    #[cfg_attr(feature = "serde", serde(rename = "layer3+4"))]
    Layer3Plus4,
    #[cfg_attr(feature = "serde", serde(rename = "encap2+3"))]
    Encap2Plus3,
    #[cfg_attr(feature = "serde", serde(rename = "encap3+4"))]
    Encap3Plus4
}

/// Set the rate at which LACPDUs are transmitted. This is only useful
/// in 802.3ad mode. Possible values are slow (30 seconds, default),
/// and fast (every second).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum LacpRate {
    #[cfg_attr(feature = "serde", serde(rename = "slow"))]
    Slow,
    #[cfg_attr(feature = "serde", serde(rename = "fast"))]
    Fast,
}

/// Set the bonding mode used for the interfaces. The default is
/// balance-rr (round robin). Possible values are balance-rr,
/// active-backup, balance-xor, broadcast, 802.3ad,
/// balance-tlb, and balance-alb.
/// For OpenVSwitch active-backup and the additional modes
/// balance-tcp and balance-slb are supported.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum BondMode {
    #[cfg_attr(feature = "serde", serde(rename = "balance-rr"))]
    BalanceRr,
    #[cfg_attr(feature = "serde", serde(rename = "active-backup"))]
    ActiveBackup,
    #[cfg_attr(feature = "serde", serde(rename = "balance-xor"))]
    BalanceXor,
    #[cfg_attr(feature = "serde", serde(rename = "broadcast"))]
    Broadcast,
    /// 802.3ad
    #[cfg_attr(feature = "serde", serde(rename = "802.3ad"))]
    EightZeroTwoDotThreeAD,
    #[cfg_attr(feature = "serde", serde(rename = "balance-tlb"))]
    BalanceTlb,
    #[cfg_attr(feature = "serde", serde(rename = "balance-alb"))]
    BalanceAlb,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct BridgeConfig {
    /// All devices matching this ID list will be added to the bridge. This may
    /// be an empty list, in which case the bridge will be brought online with
    /// no member interfaces.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub interfaces: Option<Vec<String>>,
    /// Customization parameters for special bridging options. Time intervals
    /// may need to be expressed as a number of seconds or milliseconds: the
    /// default value type is specified below. If necessary, time intervals can
    /// be qualified using a time suffix (such as “s” for seconds, “ms” for
    /// milliseconds) to allow for more control over its behavior.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub parameters: Option<BridgeParameters>,
    /// Common properties for all devices
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub common_all: Option<CommonPropertiesAllDevices>,
}

/// Customization parameters for special bridging options. Time intervals
/// may need to be expressed as a number of seconds or milliseconds: the
/// default value type is specified below. If necessary, time intervals can
/// be qualified using a time suffix (such as “s” for seconds, “ms” for
/// milliseconds) to allow for more control over its behavior.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct BridgeParameters {
    /// Set the period of time to keep a MAC address in the forwarding
    /// database after a packet is received. This maps to the AgeingTimeSec=
    /// property when the networkd renderer is used. If no time suffix is
    /// specified, the value will be interpreted as seconds.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ageing_time: Option<String>,
    /// Set the priority value for the bridge. This value should be a
    /// number between 0 and 65535. Lower values mean higher
    /// priority. The bridge with the higher priority will be elected as
    /// the root bridge.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub priority: Option<u32>,
    /// Set the port priority to . The priority value is
    /// a number between 0 and 63. This metric is used in the
    /// designated port and root port selection algorithms.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub port_priority: Option<u8>,
    /// Specify the period of time the bridge will remain in Listening and
    /// Learning states before getting to the Forwarding state. This field
    /// maps to the ForwardDelaySec= property for the networkd renderer.
    /// If no time suffix is specified, the value will be interpreted as
    /// seconds.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub forward_delay: Option<String>,
    /// Specify the interval between two hello packets being sent out from
    /// the root and designated bridges. Hello packets communicate
    /// information about the network topology. When the networkd renderer
    /// is used, this maps to the HelloTimeSec= property. If no time suffix
    /// is specified, the value will be interpreted as seconds.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub hello_time: Option<String>,
    /// Set the maximum age of a hello packet. If the last hello packet is
    /// older than that value, the bridge will attempt to become the root
    /// bridge. This maps to the MaxAgeSec= property when the networkd
    /// renderer is used. If no time suffix is specified, the value will be
    /// interpreted as seconds.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub max_age: Option<String>,
    /// Set the cost of a path on the bridge. Faster interfaces should have
    /// a lower cost. This allows a finer control on the network topology
    /// so that the fastest paths are available whenever possible.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub path_cost: Option<i32>,
    /// Define whether the bridge should use Spanning Tree Protocol. The
    /// default value is “true”, which means that Spanning Tree should be
    /// used.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub stp: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct VlanConfig {
    /// VLAN ID, a number between 0 and 4094.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub id: Option<u16>,
    /// netplan ID of the underlying device definition on which this VLAN gets
    /// created.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub link: Option<String>,
    /// Common properties for all devices
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub common_all: Option<CommonPropertiesAllDevices>,
}

/// Tunnels allow traffic to pass as if it was between systems on the same local
/// network, although systems may be far from each other but reachable via the
/// Internet. They may be used to support IPv6 traffic on a network where the ISP
/// does not provide the service, or to extend and “connect” separate local
/// networks. Please see https://en.wikipedia.org/wiki/Tunneling_protocol for
/// more general information about tunnels.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct TunnelConfig {
    /// Defines the tunnel mode. Valid options are sit, gre, ip6gre,
    /// ipip, ipip6, ip6ip6, vti, vti6 and wireguard.
    /// Additionally, the networkd backend also supports gretap and
    /// ip6gretap modes.
    /// In addition, the NetworkManager backend supports isatap tunnels.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub mode: Option<TunnelMode>,
    /// Defines the address of the local endpoint of the tunnel.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub local: Option<String>,
    /// Defines the address of the remote endpoint of the tunnel.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub remote: Option<String>,
    /// Defines the TTL of the tunnel.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ttl: Option<u64>,
    /// Define keys to use for the tunnel. The key can be a number or a dotted
    /// quad (an IPv4 address). For wireguard it can be a base64-encoded
    /// private key or (as of networkd v242+) an absolute path to a file,
    /// containing the private key (since 0.100).
    /// It is used for identification of IP transforms. This is only required
    /// for vti and vti6 when using the networkd backend, and for
    /// gre or ip6gre tunnels when using the NetworkManager backend.
    ///
    /// This field may be used as a scalar (meaning that a single key is
    /// specified and to be used for input, output and private key), or as a
    /// mapping, where you can further specify input/output/private.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub key: Option<TunnelKey>,
    /// Firewall mark for outgoing WireGuard packets from this interface,
    /// optional.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub mark: Option<String>,
    /// UDP port to listen at or auto. Optional, defaults to auto.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub port: Option<String>,
    /// A list of peers
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
    pub peers: Vec<WireGuardPeer>,
    /// Common properties for all devices
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub common_all: Option<CommonPropertiesAllDevices>,
}

/// A list of peers
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct WireGuardPeer {
    /// Remote endpoint IPv4/IPv6 address or a hostname, followed by a colon
    /// and a port number.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub endpoint: Option<String>,
    /// A list of IP (v4 or v6) addresses with CIDR masks from which this peer
    /// is allowed to send incoming traffic and to which outgoing traffic for
    /// this peer is directed. The catch-all 0.0.0.0/0 may be specified for
    /// matching all IPv4 addresses, and ::/0 may be specified for matching
    /// all IPv6 addresses.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub allowed_ips: Option<Vec<String>>,
    /// An interval in seconds, between 1 and 65535 inclusive, of how often to
    /// send an authenticated empty packet to the peer for the purpose of
    /// keeping a stateful firewall or NAT mapping valid persistently. Optional.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub keepalive: Option<u32>,
    /// Define keys to use for the WireGuard peers.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub keys: Option<WireGuardPeerKey>,
}

/// Define keys to use for the WireGuard peers.
///
/// This field can be used as a mapping, where you can further specify the
/// public and shared keys.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct WireGuardPeerKey {
    /// A base64-encoded public key, required for WireGuard peers.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub public: Option<String>,
    /// A base64-encoded preshared key. Optional for WireGuard peers.
    /// When the systemd-networkd backend (v242+) is used, this can
    /// also be an absolute path to a file containing the preshared key.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub shared: Option<String>,
}

/// Define keys to use for the tunnel. The key can be a number or a dotted
/// quad (an IPv4 address). For wireguard it can be a base64-encoded
/// private key or (as of networkd v242+) an absolute path to a file,
/// containing the private key (since 0.100).
/// It is used for identification of IP transforms. This is only required
/// for vti and vti6 when using the networkd backend, and for
/// gre or ip6gre tunnels when using the NetworkManager backend.
///
/// This field may be used as a scalar (meaning that a single key is
/// specified and to be used for input, output and private key), or as a
/// mapping, where you can further specify input/output/private.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum TunnelKey {
    Simple(String),
    Complex {
        /// The input key for the tunnel
        input: Option<String>,
        /// The output key for the tunnel
        output: Option<String>,
        /// A base64-encoded private key required for WireGuard tunnels. When the
        // systemd-networkd backend (v242+) is used, this can also be an
        // absolute path to a file containing the private key.
        private: Option<String>,
    }
}

/// Defines the tunnel mode. Valid options are sit, gre, ip6gre,
/// ipip, ipip6, ip6ip6, vti, vti6 and wireguard.
/// Additionally, the networkd backend also supports gretap and
/// ip6gretap modes.
/// In addition, the NetworkManager backend supports isatap tunnels.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "repr-c", repr(C))]
pub enum TunnelMode {
    #[cfg_attr(feature = "serde", serde(rename = "sit"))]
    Sit,
    #[cfg_attr(feature = "serde", serde(rename = "gre"))]
    Gre,
    #[cfg_attr(feature = "serde", serde(rename = "ip6gre"))]
    Ip6gre,
    #[cfg_attr(feature = "serde", serde(rename = "ipip"))]
    Ipip,
    #[cfg_attr(feature = "serde", serde(rename = "ipip6"))]
    Ipip6,
    #[cfg_attr(feature = "serde", serde(rename = "ip6ip6"))]
    Ip6ip6,
    #[cfg_attr(feature = "serde", serde(rename = "vti"))]
    Vti,
    #[cfg_attr(feature = "serde", serde(rename = "vti6"))]
    Vti6,
    #[cfg_attr(feature = "serde", serde(rename = "wireguard"))]
    Wireguard,
    #[cfg_attr(feature = "serde", serde(rename = "gretap"))]
    Gretap,
    #[cfg_attr(feature = "serde", serde(rename = "ip6gretap"))]
    Ip6gretap,
    #[cfg_attr(feature = "serde", serde(rename = "isatap"))]
    Isatap,
}