#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "derive_builder")]
use derive_builder::Builder;

use crate::CommonPropertiesAllDevices;

/// Purpose: Use the vrfs key to create Virtual Routing and Forwarding (VRF) interfaces.
///
/// Structure: The key consists of a mapping of VRF interface names.
/// The interface used in the link option (enp5s0 in the example below) must also be
/// defined in the Netplan configuration.
/// The general configuration structure for VRFs is shown below.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub struct VrfsConfig {
    /// The numeric routing table identifier. This setting is compulsory.
    pub table: i32,
    /// All devices matching this ID list will be added to the VRF.
    /// This may be an empty list,
    /// in which case the VRF will be brought online with no member interfaces.
    pub interfaces: Vec<String>,
    /// Common properties for all devices
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub common_all: Option<CommonPropertiesAllDevices>,
}
