//! Handling of YAML booleans.
//! The YAML spec allows more values than just `true` and `false:
//! - `true`, `yes`, `on`, `y` or `Y` for truthy
//! - `fals`, `no`, `off`, `n` or `N` for falsy
//!
//! This module handles these variants, as well as Optional values.

use serde::de::{Error, Visitor};
use serde::Deserializer;
use std::fmt::Formatter;

/// Deserialize a YAML boolean to a bool
pub fn string_or_bool<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    deserializer.deserialize_any(StringOrBool)
}

/// Deserialize an optional YAML boolean to a bool
/// Note that when applying this to an `Option<bool>` with `#[serde(deserialize_with = "string_or_bool_option")]`,
/// you should also apply the `#[serde(default)]` attribute.
///
/// <https://stackoverflow.com/questions/68080171/deserializer-never-called-when-field-is-not-present>
pub fn string_or_bool_option<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<bool>, D::Error> {
    deserializer.deserialize_option(StringOrBoolOption)
}

struct StringOrBool;

impl<'de> Visitor<'de> for StringOrBool {
    type Value = bool;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("YAML boolean")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match v.to_lowercase().as_str() {
            "true" | "yes" | "on" | "y" => Ok(true),
            "false" | "no" | "off" | "n" => Ok(false),
            _ => Err(Error::unknown_variant(
                v,
                &["true", "false", "yes", "no", "on", "off", "y", "n"],
            )),
        }
    }
}

struct StringOrBoolOption;

impl<'de> Visitor<'de> for StringOrBoolOption {
    type Value = Option<bool>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("YAML boolean or null")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        string_or_bool(deserializer).map(Some)
    }
}
