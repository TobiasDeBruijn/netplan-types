#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "derive_builder")]
use derive_builder::Builder;

/// Netplan supports advanced authentication settings for ethernet and wifi
/// interfaces, as well as individual wifi networks, by means of the auth block.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "derive_builder", derive(Builder))]
#[cfg_attr(feature = "serde", serde(rename_all = "kebab-case"))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct AuthConfig {
    /// The supported key management modes are none (no key management);
    /// psk (WPA with pre-shared key, common for home wifi); eap (WPA
    /// with EAP, common for enterprise wifi); and 802.1x (used primarily
    /// for wired Ethernet connections).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub key_managment: Option<KeyManagmentMode>,
    /// The password string for EAP, or the pre-shared key for WPA-PSK.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub password: Option<String>,
    /// The EAP method to use. The supported EAP methods are tls (TLS),
    /// peap (Protected EAP), and ttls (Tunneled TLS).
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub method: Option<AuthMethod>,
    /// The identity to use for EAP.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub identity: Option<String>,
    /// The identity to pass over the unencrypted channel if the chosen EAP
    /// method supports passing a different tunnelled identity.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub anonymous_identity: Option<String>,
    /// Path to a file with one or more trusted certificate authority (CA)
    /// certificates.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub ca_certificate: Option<String>,
    /// Path to a file containing the certificate to be used by the client
    /// during authentication.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub client_certificate: Option<String>,
    /// Path to a file containing the private key corresponding to
    /// client-certificate.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub client_key: Option<String>,
    /// Password to use to decrypt the private key specified in
    /// client-key if it is encrypted.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub client_key_password: Option<String>,
    /// Phase 2 authentication mechanism.
    #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Option::is_none"))]
    pub phase2_auth: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum AuthMethod {
    #[cfg_attr(feature = "serde", serde(rename = "tls"))]
    Tls,
    #[cfg_attr(feature = "serde", serde(rename = "peap"))]
    Peap,
    #[cfg_attr(feature = "serde", serde(rename = "ttls"))]
    Ttls,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum KeyManagmentMode {
    #[cfg_attr(feature = "serde", serde(rename = "none"))]
    None,
    #[cfg_attr(feature = "serde", serde(rename = "psk"))]
    Psk,
    #[cfg_attr(feature = "serde", serde(rename = "eap"))]
    Eap,
    /// 802.1x
    #[cfg_attr(feature = "serde", serde(rename = "802.1x"))]
    EightZeroTwoDotOneX,
}
