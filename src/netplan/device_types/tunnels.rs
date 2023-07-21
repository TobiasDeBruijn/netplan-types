#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "derive_builder")]
use derive_builder::Builder;

use crate::CommonPropertiesAllDevices;

/// Tunnels allow traffic to pass as if it was between systems on the same local
/// network, although systems may be far from each other but reachable via the
/// Internet. They may be used to support IPv6 traffic on a network where the ISP
/// does not provide the service, or to extend and “connect” separate local
/// networks. Please see <https://en.wikipedia.org/wiki/Tunneling_protocol> for
/// more general information about tunnels.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
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
pub enum TunnelKey {
    Simple(String),
    Complex {
        /// The input key for the tunnel
        input: Option<String>,
        /// The output key for the tunnel
        output: Option<String>,
        /// A base64-encoded private key required for WireGuard tunnels. When the
        /// systemd-networkd backend (v242+) is used, this can also be an
        /// absolute path to a file containing the private key.
        private: Option<String>,
    },
}

/// Defines the tunnel mode. Valid options are sit, gre, ip6gre,
/// ipip, ipip6, ip6ip6, vti, vti6 and wireguard.
/// Additionally, the networkd backend also supports gretap and
/// ip6gretap modes.
/// In addition, the NetworkManager backend supports isatap tunnels.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
