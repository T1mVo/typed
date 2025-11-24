use serde::{Deserialize, Serialize};

use crate::FromBytes;

const TYPE_NAME: &str = "version";

/// A structure representing version with 5 components.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(try_from = "VersionCbor", into = "VersionCbor")]
pub struct Version {
    pub major: i64,
    pub minor: i64,
    pub patch: i64,
    pub revision: i64,
    pub build: i64,
}

impl Version {
    pub const fn new(major: i64, minor: i64, patch: i64, revision: i64, build: i64) -> Self {
        Self {
            major,
            minor,
            patch,
            revision,
            build,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct VersionCbor {
    typed_type: String,
    major: i64,
    minor: i64,
    patch: i64,
    revision: i64,
    build: i64,
}

// Serilaize into VersionCbor
impl From<Version> for VersionCbor {
    fn from(value: Version) -> Self {
        Self {
            typed_type: TYPE_NAME.to_string(),
            major: value.major,
            minor: value.minor,
            patch: value.patch,
            revision: value.revision,
            build: value.build,
        }
    }
}

// Deserialize from VersionCbor
impl TryFrom<VersionCbor> for Version {
    type Error = String;

    fn try_from(value: VersionCbor) -> Result<Self, Self::Error> {
        if value.typed_type != TYPE_NAME {
            return Err(format!(
                "Invalid typed-type for Version: {}",
                value.typed_type
            ));
        }

        Ok(Self::new(
            value.major,
            value.minor,
            value.patch,
            value.revision,
            value.build,
        ))
    }
}

impl FromBytes for Version {
    fn from_bytes(bytes: &[u8]) -> Result<Self, String> {
        ciborium::from_reader(bytes).map_err(|err| err.to_string())
    }
}
