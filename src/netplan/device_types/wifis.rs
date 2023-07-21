#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "derive_builder")]
use derive_builder::Builder;

use crate::{AuthConfig, CommonPropertiesAllDevices, CommonPropertiesPhysicalDeviceType};
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
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

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
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
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub hidden: Option<bool>,
}

/// Possible bands are 5GHz (for 5GHz 802.11a) and 2.4GHz
/// (for 2.4GHz 802.11), do not restrict the 802.11 frequency band of the
/// network if unset (the default).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
pub enum AccessPointMode {
    #[cfg_attr(feature = "serde", serde(rename = "infrastructure"))]
    Infrastructure,
    #[cfg_attr(feature = "serde", serde(rename = "ap"))]
    Ap,
    #[cfg_attr(feature = "serde", serde(rename = "adhoc"))]
    Adhoc,
}

/// This enables WakeOnWLan on supported devices. Not all drivers support all
/// options. May be any combination of any, disconnect, magic_pkt,
/// gtk_rekey_failure, eap_identity_req, four_way_handshake,
/// rfkill_release or tcp (NetworkManager only). Or the exclusive
/// default flag (the default).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
