#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "derive_builder")]
use derive_builder::Builder;

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
pub struct RoutingConfig {
    /// Set a source IP address for traffic going through the route.
    /// (NetworkManager: as of v1.8.0)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub from: Option<String>,
    /// Destination address for the route.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub to: Option<String>,
    /// Address to the gateway to use for this route.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub via: Option<String>,
    /// When set to “true”, specifies that the route is directly connected
    /// to the interface.
    /// (NetworkManager: as of v1.12.0 for IPv4 and v1.18.0 for IPv6)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub on_link: Option<bool>,
    /// The relative priority of the route. Must be a positive integer value.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub metric: Option<u16>,
    /// The type of route. Valid options are “unicast” (default), “anycast”,
    /// “blackhole”, “broadcast”, “local”, “multicast”, “nat”, “prohibit”,
    /// “throw”, “unreachable” or “xresolve”.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub r#type: Option<RouteType>,
    /// The route scope, how wide-ranging it is to the network. Possible
    /// values are “global”, “link”, or “host”.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub scope: Option<RouteScope>,
    /// The table number to use for the route. In some scenarios, it may be
    /// useful to set routes in a separate routing table. It may also be used
    /// to refer to routing policy rules which also accept a table
    /// parameter. Allowed values are positive integers starting from 1.
    /// Some values are already in use to refer to specific routing tables:
    /// see /etc/iproute2/rt_tables.
    /// (NetworkManager: as of v1.10.0)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub table: Option<u16>,
    /// The MTU to be used for the route, in bytes. Must be a positive integer
    /// value.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub mtu: Option<u16>,
    /// The congestion window to be used for the route, represented by number
    /// of segments. Must be a positive integer value.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub congestion_window: Option<u16>,
    /// The receive window to be advertised for the route, represented by
    /// number of segments. Must be a positive integer value.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub advertised_receive_window: Option<u16>,
}

/// The type of route. Valid options are “unicast” (default), “anycast”,
/// “blackhole”, “broadcast”, “local”, “multicast”, “nat”, “prohibit”,
/// “throw”, “unreachable” or “xresolve”.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
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
pub struct RoutingPolicy {
    /// Set a source IP address to match traffic for this policy rule.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub from: Option<String>,
    /// Match on traffic going to the specified destination.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
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
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub priority: Option<i32>,
    /// Have this routing policy rule match on traffic that has been marked
    /// by the iptables firewall with this value. Allowed values are positive
    /// integers starting from 1.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub mark: Option<u16>,
    /// Match this policy rule based on the type of service number applied to
    /// the traffic.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub type_of_service: Option<String>,
}

/// Set DNS servers and search domains, for manual address configuration.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
pub struct NameserverConfig {
    /// A list of IPv4 or IPv6 addresses
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub addresses: Option<Vec<String>>,
    /// A list of search domains.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub search: Option<Vec<String>>,
}
