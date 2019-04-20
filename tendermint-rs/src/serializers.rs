//! Serde serializers

use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "rpc")]
use std::time::Duration;

/// Parse `u64` from a JSON string
pub(crate) fn parse_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .parse::<u64>()
        .map_err(|e| D::Error::custom(format!("{}", e)))
}

/// Serialize `u64` as a JSON string
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn serialize_u64<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    format!("{}", value).serialize(serializer)
}

/// Parse `Duration` from a JSON string containing a nanosecond count
#[cfg(feature = "rpc")]
pub(crate) fn parse_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    // TODO(tarcieri): handle 64-bit overflow?
    let nanos = String::deserialize(deserializer)?
        .parse::<u64>()
        .map_err(|e| D::Error::custom(format!("{}", e)))?;

    Ok(Duration::from_nanos(nanos))
}

/// Serialize `Duration` as a JSON string containing a nanosecond count
#[cfg(feature = "rpc")]
pub(crate) fn serialize_duration<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // TODO(tarcieri): use `as_nanos` when we're Rust 1.33+
    format!(
        "{}",
        (duration.as_secs() * 1_000_000_000) + u64::from(duration.subsec_nanos())
    )
    .serialize(serializer)
}
