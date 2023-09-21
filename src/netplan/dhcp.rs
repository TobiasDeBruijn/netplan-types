#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "derive_builder")]
use derive_builder::Builder;

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
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct DhcpOverrides {
    /// Default: true. When true, the DNS servers received from the
    /// DHCP server will be used and take precedence over any statically
    /// configured ones. Currently only has an effect on the networkd
    /// backend.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub use_dns: Option<bool>,
    /// Default: true. When true, the NTP servers received from the
    /// DHCP server will be used by systemd-timesyncd and take precedence
    /// over any statically configured ones. Currently only has an effect on
    /// the networkd backend.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub use_ntp: Option<bool>,
    /// Default: true. When true, the machine’s hostname will be sent
    /// to the DHCP server. Currently only has an effect on the networkd
    /// backend.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub send_hostname: Option<bool>,
    /// Default: true. When true, the hostname received from the DHCP
    /// server will be set as the transient hostname of the system. Currently
    /// only has an effect on the networkd backend.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub use_hostname: Option<bool>,
    /// Default: true. When true, the MTU received from the DHCP
    /// server will be set as the MTU of the network interface. When false,
    /// the MTU advertised by the DHCP server will be ignored. Currently only
    /// has an effect on the networkd backend.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub use_mtu: Option<bool>,
    /// Use this value for the hostname which is sent to the DHCP server,
    /// instead of machine’s hostname. Currently only has an effect on the
    /// networkd backend.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub hostname: Option<String>,
    /// Default: true. When true, the routes received from the DHCP
    /// server will be installed in the routing table normally. When set to
    /// false, routes from the DHCP server will be ignored: in this case,
    /// the user is responsible for adding static routes if necessary for
    /// correct network operation. This allows users to avoid installing a
    /// default gateway for interfaces configured via DHCP. Available for
    /// both the networkd and NetworkManager backends.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub use_routes: Option<bool>,
    /// Use this value for default metric for automatically-added routes.
    /// Use this to prioritize routes for devices by setting a lower metric
    /// on a preferred interface. Available for both the networkd and
    /// NetworkManager backends.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub route_metric: Option<u16>,
    /// Takes a boolean, or the special value “route”. When true, the domain
    /// name received from the DHCP server will be used as DNS search domain
    /// over this link, similar to the effect of the Domains= setting. If set
    /// to “route”, the domain name received from the DHCP server will be
    /// used for routing DNS queries only, but not for searching, similar to
    /// the effect of the Domains= setting when the argument is prefixed with
    /// “~”.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub use_domains: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum Ipv6AddressGeneration {
    #[cfg_attr(feature = "serde", serde(rename = "eui64"))]
    Eui64,
    #[cfg_attr(feature = "serde", serde(rename = "stable-privacy"))]
    StablePrivacy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(untagged))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
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
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum PreferredLifetime {
    #[cfg_attr(feature = "serde", serde(rename = "forever"))]
    Forever,
    #[cfg_attr(feature = "serde", serde(rename = "0"))]
    Zero,
}
