#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "derive_builder")]
use derive_builder::Builder;

/// Common properties for physical device types
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub struct CommonPropertiesPhysicalDeviceType {
    /// This selects a subset of available physical devices by various hardware
    /// properties. The following configuration will then apply to all matching
    /// devices, as soon as they appear. All specified properties must match.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub r#match: Option<MatchConfig>,
    /// When matching on unique properties such as path or MAC, or with additional
    /// assumptions such as “there will only ever be one wifi device”,
    /// match rules can be written so that they only match one device. Then this
    /// property can be used to give that device a more specific/desirable/nicer
    /// name than the default from udev’s ifnames. Any additional device that
    /// satisfies the match rules will then fail to get renamed and keep the
    /// original kernel name (and dmesg will show an error).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub set_name: Option<String>,
    /// Enable wake on LAN. Off by default.
    ///
    /// Note: This will not work reliably for devices matched by name
    /// only and rendered by networkd, due to interactions with device
    /// renaming in udev. Match devices by MAC when setting wake on LAN.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub wakeonlan: Option<bool>,
    /// (networkd backend only) Whether to emit LLDP packets. Off by default.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub emit_lldp: Option<bool>,
    /// (networkd backend only) If set to true, the hardware offload for
    /// checksumming of ingress network packets is enabled. When unset,
    /// the kernel’s default will be used.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub receive_checksum_offload: Option<bool>,
    /// (networkd backend only) If set to true, the hardware offload for
    /// checksumming of egress network packets is enabled. When unset,
    /// the kernel’s default will be used.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub transmit_checksum_offload: Option<bool>,
    /// (networkd backend only) If set to true, the TCP Segmentation
    /// Offload (TSO) is enabled. When unset, the kernel’s default will
    /// be used.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub tcp_segmentation_offload: Option<bool>,
    /// (networkd backend only) If set to true, the TCP6 Segmentation
    /// Offload (tx-tcp6-segmentation) is enabled. When unset, the
    /// kernel’s default will be used.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub tcp6_segmentation_offload: Option<bool>,
    /// (networkd backend only) If set to true, the Generic Segmentation
    /// Offload (GSO) is enabled. When unset, the kernel’s default will
    /// be used.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub generic_segmentation_offload: Option<bool>,
    /// (networkd backend only) If set to true, the Generic Receive
    /// Offload (GRO) is enabled. When unset, the kernel’s default will
    /// be used.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub generic_receive_offload: Option<bool>,
    /// (networkd backend only) If set to true, the Generic Receive
    /// Offload (GRO) is enabled. When unset, the kernel’s default will
    /// be used.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
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
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub openvswitch: Option<OpenVSwitchConfig>,
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
pub struct OpenVSwitchConfig {
    /// Passed-through directly to OpenVSwitch
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub external_ids: Option<String>,
    /// Passed-through directly to OpenVSwitch
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub other_config: Option<String>,
    /// Valid for bond interfaces. Accepts active, passive or off (the default).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub lacp: Option<Lacp>,
    /// Valid for bridge interfaces. Accepts secure or standalone (the default).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub fail_mode: Option<FailMode>,
    /// Valid for bridge interfaces. False by default.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub mcast_snooping: Option<bool>,
    /// Valid for bridge interfaces or the network section. List of protocols to be used when
    /// negotiating a connection with the controller. Accepts OpenFlow10, OpenFlow11,
    /// OpenFlow12, OpenFlow13, OpenFlow14, OpenFlow15 and OpenFlow16.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub protocols: Option<Vec<OpenFlowProtocol>>,
    /// Valid for bridge interfaces. False by default.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub rtsp: Option<bool>,
    /// Valid for bridge interfaces. Specify an external OpenFlow controller.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub controller: Option<ControllerConfig>,
    /// OpenvSwitch patch ports. Each port is declared as a pair of names
    /// which can be referenced as interfaces in dependent virtual devices
    /// (bonds, bridges).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ports: Option<Vec<String>>,
    /// Valid for global openvswitch settings. Options for configuring SSL
    /// server endpoint for the switch.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ssl: Option<SslConfig>,
}

/// Valid for global openvswitch settings. Options for configuring SSL
/// server endpoint for the switch.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub struct SslConfig {
    /// Path to a file containing the CA certificate to be used.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ca_cert: Option<String>,
    /// Path to a file containing the server certificate.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub certificate: Option<String>,
    /// Path to a file containing the private key for the server.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub private_key: Option<String>,
}

/// Valid for bridge interfaces. Specify an external OpenFlow controller.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub struct ControllerConfig {
    /// Set the list of addresses to use for the controller targets. The
    /// syntax of these addresses is as defined in ovs-vsctl(8). Example:
    /// addresses: [tcp:127.0.0.1:6653, "ssl:[fe80::1234%eth0]:6653"]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub addresses: Option<Vec<String>>,
    /// Set the connection mode for the controller. Supported options are
    /// in-band and out-of-band. The default is in-band.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub connection_mode: Option<ConnectionMode>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum ConnectionMode {
    InBand,
    OutOfBand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
pub enum Lacp {
    Active,
    Passive,
    Off,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
pub struct MatchConfig {
    /// Current interface name. Globs are supported, and the primary use case
    /// for matching on names, as selecting one fixed name can be more easily
    /// achieved with having no match: at all and just using the ID (see
    /// above).
    /// (NetworkManager: as of v1.14.0)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub name: Option<String>,
    /// Device’s MAC address in the form “XX:XX:XX:XX:XX:XX”. Globs are not
    /// allowed.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub macaddress: Option<String>,
    /// Kernel driver name, corresponding to the DRIVER udev property.
    /// A sequence of globs is supported, any of which must match.
    /// Matching on driver is only supported with networkd.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub driver: Option<Vec<String>>,
}
