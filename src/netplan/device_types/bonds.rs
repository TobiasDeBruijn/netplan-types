#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "derive_builder")]
use derive_builder::Builder;

use crate::CommonPropertiesAllDevices;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
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
    #[cfg_attr(feature = "serde", serde(default))]
    #[cfg_attr(
        feature = "serde",
        serde(deserialize_with = "crate::bool::string_or_bool_option")
    )]
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

/// Set the bonding mode used for the interfaces. The default is
/// balance-rr (round robin). Possible values are balance-rr,
/// active-backup, balance-xor, broadcast, 802.3ad,
/// balance-tlb, and balance-alb.
/// For OpenVSwitch active-backup and the additional modes
/// balance-tcp and balance-slb are supported.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

/// Set the rate at which LACPDUs are transmitted. This is only useful
/// in 802.3ad mode. Possible values are slow (30 seconds, default),
/// and fast (every second).
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LacpRate {
    #[cfg_attr(feature = "serde", serde(rename = "slow"))]
    Slow,
    #[cfg_attr(feature = "serde", serde(rename = "fast"))]
    Fast,
}

/// Specifies the transmit hash policy for the selection of slaves. This
/// is only useful in balance-xor, 802.3ad and balance-tlb modes.
/// Possible values are layer2, layer3+4, layer2+3,
/// encap2+3, and encap3+4.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TransmitHashPolicy {
    #[cfg_attr(feature = "serde", serde(rename = "layer2"))]
    Layer2,
    #[cfg_attr(feature = "serde", serde(rename = "layer3+4"))]
    Layer3Plus4,
    #[cfg_attr(feature = "serde", serde(rename = "encap2+3"))]
    Encap2Plus3,
    #[cfg_attr(feature = "serde", serde(rename = "encap3+4"))]
    Encap3Plus4,
}

/// Set the aggregation selection mode. Possible values are stable,
/// bandwidth, and count. This option is only used in 802.3ad
/// mode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum AdSelect {
    #[cfg_attr(feature = "serde", serde(rename = "stable"))]
    Stable,
    #[cfg_attr(feature = "serde", serde(rename = "bandwidth"))]
    Bandwidth,
    #[cfg_attr(feature = "serde", serde(rename = "count"))]
    Count,
}

/// Configure how ARP replies are to be validated when using ARP link
/// monitoring. Possible values are none, active, backup,
/// and all.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ArpValidate {
    #[cfg_attr(feature = "serde", serde(rename = "none"))]
    None,
    #[cfg_attr(feature = "serde", serde(rename = "active"))]
    Active,
    #[cfg_attr(feature = "serde", serde(rename = "backup"))]
    Backup,
    #[cfg_attr(feature = "serde", serde(rename = "all"))]
    All,
}

/// Specify whether to use any ARP IP target being up as sufficient for
/// a slave to be considered up; or if all the targets must be up. This
/// is only used for active-backup mode when arp-validate is
/// enabled. Possible values are any and all.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ArpAllTargets {
    #[cfg_attr(feature = "serde", serde(rename = "any"))]
    Any,
    #[cfg_attr(feature = "serde", serde(rename = "all"))]
    All,
}

/// Set whether to set all slaves to the same MAC address when adding
/// them to the bond, or how else the system should handle MAC addresses.
/// The possible values are none, active, and follow.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum FailOverMacPolicy {
    #[cfg_attr(feature = "serde", serde(rename = "none"))]
    None,
    #[cfg_attr(feature = "serde", serde(rename = "activv"))]
    Active,
    #[cfg_attr(feature = "serde", serde(rename = "follow"))]
    Follow,
}

/// Set the reselection policy for the primary slave. On failure of the
/// active slave, the system will use this policy to decide how the new
/// active slave will be chosen and how recovery will be handled. The
/// possible values are always, better, and failure.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PrimaryReselectPolicy {
    #[cfg_attr(feature = "serde", serde(rename = "always"))]
    Always,
    #[cfg_attr(feature = "serde", serde(rename = "better"))]
    Better,
    #[cfg_attr(feature = "serde", serde(rename = "failure"))]
    Failure,
}
