#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "derive_builder")]
use derive_builder::Builder;

use crate::CommonPropertiesAllDevices;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
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
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
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
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub stp: Option<bool>,
}
