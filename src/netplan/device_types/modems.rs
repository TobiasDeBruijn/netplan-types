#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "derive_builder")]
use derive_builder::Builder;

use crate::{CommonPropertiesAllDevices, CommonPropertiesPhysicalDeviceType};

/// GSM/CDMA modem configuration is only supported for the NetworkManager
/// backend. systemd-networkd does not support modems.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub struct ModemConfig {
    /// Set the carrier APN (Access Point Name). This can be omitted if
    /// auto-config is enabled.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub apn: Option<String>,
    /// Specify whether to try and autoconfigure the modem by doing a lookup of
    /// the carrier against the Mobile Broadband Provider database. This may not
    /// work for all carriers.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
    pub auto_config: Option<bool>,
    /// Specify the device ID (as given by the WWAN management service) of the
    /// modem to match. This can be found using mmcli.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub device_id: Option<String>,
    /// Specify the Network ID (GSM LAI format). If this is specified, the device
    /// will not roam networks.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub network_id: Option<String>,
    /// The number to dial to establish the connection to the mobile broadband
    /// network. (Deprecated for GSM)
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub number: Option<String>,
    /// Specify the password used to authenticate with the carrier network. This
    /// can be omitted if auto-config is enabled.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub password: Option<String>,
    /// Specify the SIM PIN to allow it to operate if a PIN is set.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub pin: Option<String>,
    /// Specify the SIM unique identifier (as given by the WWAN management service)
    /// which this connection applies to. If given, the connection will apply to
    /// any device also allowed by device-id which contains a SIM card matching
    /// the given identifier.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub sim_id: Option<String>,
    /// Specify the MCC/MNC string (such as “310260” or “21601”) which identifies
    /// the carrier that this connection should apply to. If given, the connection
    /// will apply to any device also allowed by device-id and sim-id
    /// which contains a SIM card provisioned by the given operator.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub sim_operator_id: Option<String>,
    /// Specify the username used to authentiate with the carrier network. This
    /// can be omitted if auto-config is enabled.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub username: Option<String>,
    /// Common properties for physical device types
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub common_physical: Option<CommonPropertiesPhysicalDeviceType>,
    /// Common properties for all devices
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub common_all: Option<CommonPropertiesAllDevices>,
}
