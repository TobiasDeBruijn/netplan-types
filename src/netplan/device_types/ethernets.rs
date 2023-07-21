#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "derive_builder")]
use derive_builder::Builder;

use crate::{CommonPropertiesAllDevices, CommonPropertiesPhysicalDeviceType};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub struct EthernetConfig {
    /// (SR-IOV devices only) The link property declares the device as a
    /// Virtual Function of the selected Physical Function device, as identified
    /// by the given netplan id.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub link: Option<String>,
    /// (SR-IOV devices only) In certain special cases VFs might need to be
    /// configured outside of netplan. For such configurations virtual-function-count
    /// can be optionally used to set an explicit number of Virtual Functions for
    /// the given Physical Function. If unset, the default is to create only as many
    /// VFs as are defined in the netplan configuration. This should be used for special
    /// cases only.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub virtual_function_count: Option<u16>,
    /// (SR-IOV devices only) Change the operational mode of the embedded switch
    /// of a supported SmartNIC PCI device (e.g. Mellanox ConnectX-5). Possible
    /// values are switchdev or legacy, if unspecified the vendor’s
    /// default configuration is used.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub embedded_switch_mode: Option<EmbeddedSwitchMode>,
    /// (SR-IOV devices only) Delay rebinding of SR-IOV virtual functions to its
    /// driver after changing the embedded-switch-mode setting to a later stage.
    /// Can be enabled when bonding/VF LAG is in use. Defaults to false.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub delay_virtual_functions_rebind: Option<bool>,
    /// Common properties for physical device types
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub common_physical: Option<CommonPropertiesPhysicalDeviceType>,
    /// Common properties for all devices
    #[cfg_attr(feature = "serde", serde(flatten))]
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub common_all: Option<CommonPropertiesAllDevices>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
pub enum EmbeddedSwitchMode {
    Switchdev,
    Legacy,
}
