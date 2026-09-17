use std::collections::BTreeMap;
use std::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

macro_rules! string_id {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(value: impl Into<String>) -> Result<Self, InvalidId> {
                let value = value.into();
                if value.trim().is_empty() {
                    return Err(InvalidId(stringify!($name)));
                }
                Ok(Self(value))
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
    };
}

#[derive(Clone, Debug, Error, Eq, PartialEq)]
#[error("{0} cannot be empty")]
pub struct InvalidId(&'static str);

string_id!(SurfaceId);
string_id!(ProfileId);
string_id!(RemoteId);
string_id!(ObjectId);

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileOwnership {
    SelfOwned,
    External,
    Project,
    Organization,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TrackedProfile {
    pub id: ProfileId,
    pub surface: SurfaceId,
    pub remote_id: Option<RemoteId>,
    pub handle: Option<String>,
    pub ownership: ProfileOwnership,
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ObservationMeta {
    pub surface: SurfaceId,
    pub observed_at: DateTime<Utc>,
    pub acquisition_id: String,
    pub schema_version: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProfileSnapshot {
    pub profile_id: ProfileId,
    pub remote_id: RemoteId,
    pub handle: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub metrics: BTreeMap<String, i64>,
    pub observation: ObservationMeta,
    #[serde(default)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub id: ObjectId,
    pub profile_id: ProfileId,
    pub remote_id: RemoteId,
    pub created_at: Option<DateTime<Utc>>,
    pub text: String,
    pub reply_to: Option<ObjectId>,
    pub quote_of: Option<ObjectId>,
    pub canonical_url: Option<String>,
    pub observation: ObservationMeta,
    #[serde(default)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "record", rename_all = "snake_case")]
pub enum NormalizedRecord {
    ProfileSnapshot(ProfileSnapshot),
    Post(Post),
}

impl NormalizedRecord {
    pub fn profile_id(&self) -> &ProfileId {
        match self {
            Self::ProfileSnapshot(record) => &record.profile_id,
            Self::Post(record) => &record.profile_id,
        }
    }

    pub fn observation(&self) -> &ObservationMeta {
        match self {
            Self::ProfileSnapshot(record) => &record.observation,
            Self::Post(record) => &record.observation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_reject_blank_values() {
        assert!(SurfaceId::new("   ").is_err());
    }

    #[test]
    fn profile_and_person_are_not_collapsed() {
        let profile = TrackedProfile {
            id: ProfileId::new("x-main").unwrap(),
            surface: SurfaceId::new("x").unwrap(),
            remote_id: None,
            handle: Some("sguzman".into()),
            ownership: ProfileOwnership::SelfOwned,
            enabled: true,
        };

        assert_eq!(profile.surface.as_str(), "x");
        assert_eq!(profile.handle.as_deref(), Some("sguzman"));
    }
}
