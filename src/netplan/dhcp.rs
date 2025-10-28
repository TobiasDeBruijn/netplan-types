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
    Complex(std::collections::HashMap<String, AddressProperties>),
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct AddressProperties {
    /// An IP address label, equivalent to the ip address label
    /// command. Currently supported on the networkd backend only.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub label: Option<String>,

    /// Default: forever. This can be forever or 0 and corresponds
    /// to the PreferredLifetime option in systemd-networkd's Address
    /// section. Currently supported on the networkd backend only.
    /// Since 0.100.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub lifetime: Option<PreferredLifetime>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dhcp_overrides_defaults() {
        let overrides = DhcpOverrides::default();
        assert_eq!(overrides.use_dns, None);
        assert_eq!(overrides.use_ntp, None);
        assert_eq!(overrides.send_hostname, None);
        assert_eq!(overrides.use_hostname, None);
        assert_eq!(overrides.use_mtu, None);
        assert_eq!(overrides.hostname, None);
        assert_eq!(overrides.use_routes, None);
        assert_eq!(overrides.route_metric, None);
        assert_eq!(overrides.use_domains, None);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_dhcp_overrides_serialize() {
        let overrides = DhcpOverrides {
            use_dns: Some(false),
            use_ntp: Some(true),
            send_hostname: Some(true),
            use_hostname: Some(false),
            use_mtu: Some(true),
            hostname: Some("test-host".to_string()),
            use_routes: Some(false),
            route_metric: Some(100),
            use_domains: Some("route".to_string()),
        };

        let yaml = serde_yaml::to_string(&overrides).unwrap();
        assert!(yaml.contains("use-dns: false"));
        assert!(yaml.contains("use-ntp: true"));
        assert!(yaml.contains("send-hostname: true"));
        assert!(yaml.contains("use-hostname: false"));
        assert!(yaml.contains("use-mtu: true"));
        assert!(yaml.contains("hostname: test-host"));
        assert!(yaml.contains("use-routes: false"));
        assert!(yaml.contains("route-metric: 100"));
        assert!(yaml.contains("use-domains: route"));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_dhcp_overrides_deserialize() {
        let yaml = r#"
use-dns: false
use-ntp: true
send-hostname: true
use-hostname: false
use-mtu: true
hostname: test-host
use-routes: false
route-metric: 100
use-domains: route
"#;

        let overrides: DhcpOverrides = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(overrides.use_dns, Some(false));
        assert_eq!(overrides.use_ntp, Some(true));
        assert_eq!(overrides.send_hostname, Some(true));
        assert_eq!(overrides.use_hostname, Some(false));
        assert_eq!(overrides.use_mtu, Some(true));
        assert_eq!(overrides.hostname, Some("test-host".to_string()));
        assert_eq!(overrides.use_routes, Some(false));
        assert_eq!(overrides.route_metric, Some(100));
        assert_eq!(overrides.use_domains, Some("route".to_string()));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_dhcp_overrides_skip_none_serialization() {
        let overrides = DhcpOverrides {
            use_dns: Some(false),
            use_ntp: None,
            ..Default::default()
        };

        let yaml = serde_yaml::to_string(&overrides).unwrap();
        assert!(yaml.contains("use-dns: false"));
        assert!(!yaml.contains("use-ntp"));
        assert!(!yaml.contains("send-hostname"));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_dhcp_overrides_bool_as_string() {
        let yaml = r#"
use-dns: "false"
use-ntp: "true"
"#;

        let overrides: DhcpOverrides = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(overrides.use_dns, Some(false));
        assert_eq!(overrides.use_ntp, Some(true));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_ipv6_address_generation_serialize() {
        let eui64 = Ipv6AddressGeneration::Eui64;
        let stable = Ipv6AddressGeneration::StablePrivacy;

        let eui64_yaml = serde_yaml::to_string(&eui64).unwrap();
        let stable_yaml = serde_yaml::to_string(&stable).unwrap();

        assert!(eui64_yaml.contains("eui64"));
        assert!(stable_yaml.contains("stable-privacy"));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_ipv6_address_generation_deserialize() {
        let eui64: Ipv6AddressGeneration = serde_yaml::from_str("eui64").unwrap();
        let stable: Ipv6AddressGeneration = serde_yaml::from_str("stable-privacy").unwrap();

        assert_eq!(eui64, Ipv6AddressGeneration::Eui64);
        assert_eq!(stable, Ipv6AddressGeneration::StablePrivacy);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_address_mapping_simple() {
        let simple = AddressMapping::Simple("192.168.1.10/24".to_string());

        let yaml = serde_yaml::to_string(&simple).unwrap();
        assert_eq!(yaml.trim(), "192.168.1.10/24");

        let deserialized: AddressMapping = serde_yaml::from_str("192.168.1.10/24").unwrap();
        assert_eq!(deserialized, simple);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_address_mapping_complex() {
        let mut map = std::collections::HashMap::new();
        map.insert(
            "192.168.1.10/24".to_string(),
            AddressProperties {
                label: Some("my-label".to_string()),
                lifetime: Some(PreferredLifetime::Forever),
            },
        );
        let complex = AddressMapping::Complex(map);

        let yaml = serde_yaml::to_string(&complex).unwrap();
        assert!(yaml.contains("192.168.1.10/24"));
        assert!(yaml.contains("label: my-label"));
        assert!(yaml.contains("lifetime: forever"));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_address_mapping_complex_deserialize() {
        let yaml = r#"
192.168.1.10/24:
  label: my-label
  lifetime: forever
"#;

        let mapping: AddressMapping = serde_yaml::from_str(yaml).unwrap();

        if let AddressMapping::Complex(map) = mapping {
            let props = map.get("192.168.1.10/24").unwrap();
            assert_eq!(props.label, Some("my-label".to_string()));
            assert_eq!(props.lifetime, Some(PreferredLifetime::Forever));
        } else {
            panic!("Expected Complex variant");
        }
    }

    #[test]
    fn test_address_properties_defaults() {
        let props = AddressProperties::default();
        assert_eq!(props.label, None);
        assert_eq!(props.lifetime, None);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_address_properties_lifetime_forever() {
        let yaml = r#"
label: test
lifetime: forever
"#;

        let props: AddressProperties = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(props.label, Some("test".to_string()));
        assert_eq!(props.lifetime, Some(PreferredLifetime::Forever));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_address_properties_lifetime_zero() {
        let yaml = r#"
label: test
lifetime: "0"
"#;

        let props: AddressProperties = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(props.label, Some("test".to_string()));
        assert_eq!(props.lifetime, Some(PreferredLifetime::Zero));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_address_properties_skip_none() {
        let props = AddressProperties {
            label: Some("test".to_string()),
            lifetime: None,
        };

        let yaml = serde_yaml::to_string(&props).unwrap();
        assert!(yaml.contains("label: test"));
        assert!(!yaml.contains("lifetime"));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_address_properties_label_only() {
        let yaml = r#"
label: management
"#;

        let props: AddressProperties = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(props.label, Some("management".to_string()));
        assert_eq!(props.lifetime, None);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_use_domains_boolean() {
        let yaml = r#"
use-domains: true
"#;

        let overrides: DhcpOverrides = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(overrides.use_domains, Some("true".to_string()));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_use_domains_route() {
        let yaml = r#"
use-domains: route
"#;

        let overrides: DhcpOverrides = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(overrides.use_domains, Some("route".to_string()));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_address_mapping_list() {
        let yaml = r#"
- 192.168.1.10/24
- 192.168.1.11/24:
    label: backup
    lifetime: "0"
- 10.0.0.1/8
"#;

        let mappings: Vec<AddressMapping> = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(mappings.len(), 3);

        // First should be simple
        assert!(matches!(mappings[0], AddressMapping::Simple(_)));

        // Second should be complex
        if let AddressMapping::Complex(map) = &mappings[1] {
            let props = map.get("192.168.1.11/24").unwrap();
            assert_eq!(props.label, Some("backup".to_string()));
            assert_eq!(props.lifetime, Some(PreferredLifetime::Zero));
        } else {
            panic!("Expected Complex variant");
        }

        // Third should be simple
        assert!(matches!(mappings[2], AddressMapping::Simple(_)));
    }

    #[test]
    fn test_ipv6_address_generation_equality() {
        assert_eq!(Ipv6AddressGeneration::Eui64, Ipv6AddressGeneration::Eui64);
        assert_eq!(
            Ipv6AddressGeneration::StablePrivacy,
            Ipv6AddressGeneration::StablePrivacy
        );
        assert_ne!(
            Ipv6AddressGeneration::Eui64,
            Ipv6AddressGeneration::StablePrivacy
        );
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_route_metric_range() {
        let yaml = r#"
route-metric: 65535
"#;

        let overrides: DhcpOverrides = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(overrides.route_metric, Some(65535));
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_empty_dhcp_overrides() {
        let yaml = "{}";
        let overrides: DhcpOverrides = serde_yaml::from_str(yaml).unwrap();

        assert_eq!(overrides.use_dns, None);
        assert_eq!(overrides.use_ntp, None);
        assert_eq!(overrides.route_metric, None);
    }
}
