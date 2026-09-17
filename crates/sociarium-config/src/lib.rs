use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use sociarium_core::TrackedProfile;
use thiserror::Error;

pub const CONFIG_SCHEMA_VERSION: u32 = 1;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SociariumConfig {
    pub schema_version: u32,
    #[serde(default)]
    pub profiles: Vec<TrackedProfile>,
}

impl SociariumConfig {
    pub fn from_toml(input: &str) -> Result<Self, ConfigError> {
        let config: Self = toml::from_str(input)?;
        config.validate()?;
        Ok(config)
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        let input = fs::read_to_string(path)?;
        Self::from_toml(&input)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.schema_version != CONFIG_SCHEMA_VERSION {
            return Err(ConfigError::UnsupportedSchemaVersion(self.schema_version));
        }

        let mut ids = BTreeSet::new();
        for profile in &self.profiles {
            if profile.id.as_str().trim().is_empty() {
                return Err(ConfigError::BlankField("profiles[].id"));
            }
            if profile.surface.as_str().trim().is_empty() {
                return Err(ConfigError::BlankField("profiles[].surface"));
            }
            if let Some(remote_id) = &profile.remote_id {
                if remote_id.as_str().trim().is_empty() {
                    return Err(ConfigError::BlankField("profiles[].remote_id"));
                }
            }
            if let Some(handle) = &profile.handle {
                if handle.trim().is_empty() {
                    return Err(ConfigError::BlankField("profiles[].handle"));
                }
            }
            if !ids.insert(profile.id.clone()) {
                return Err(ConfigError::DuplicateProfileId(profile.id.to_string()));
            }
        }

        Ok(())
    }

    pub fn enabled_profiles(&self) -> impl Iterator<Item = &TrackedProfile> {
        self.profiles.iter().filter(|profile| profile.enabled)
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("configuration file error: {0}")]
    Io(#[from] std::io::Error),
    #[error("configuration TOML error: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("unsupported configuration schema version: {0}")]
    UnsupportedSchemaVersion(u32),
    #[error("configuration field cannot be blank: {0}")]
    BlankField(&'static str),
    #[error("duplicate profile id: {0}")]
    DuplicateProfileId(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_multiple_surfaces_without_singleton_assumptions() {
        let config = SociariumConfig::from_toml(
            r#"
schema_version = 1

[[profiles]]
id = "x-main"
surface = "x"
handle = "sguzman"
ownership = "self_owned"
enabled = true

[[profiles]]
id = "reddit-main"
surface = "reddit"
handle = "example"
ownership = "self_owned"
enabled = false
"#,
        )
        .unwrap();

        assert_eq!(config.profiles.len(), 2);
        assert_eq!(config.enabled_profiles().count(), 1);
        assert_eq!(config.profiles[0].surface.as_str(), "x");
        assert_eq!(config.profiles[1].surface.as_str(), "reddit");
    }

    #[test]
    fn rejects_duplicate_profile_ids() {
        let error = SociariumConfig::from_toml(
            r#"
schema_version = 1

[[profiles]]
id = "same"
surface = "x"
ownership = "self_owned"
enabled = true

[[profiles]]
id = "same"
surface = "reddit"
ownership = "external"
enabled = true
"#,
        )
        .unwrap_err();

        assert!(matches!(error, ConfigError::DuplicateProfileId(id) if id == "same"));
    }
}
