use std::collections::BTreeMap;
use std::sync::Mutex;

use sociarium_core::{ProfileId, SurfaceId};
use thiserror::Error;

const SERVICE_NAME: &str = "sociarium";

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CredentialKey {
    pub surface: SurfaceId,
    pub profile_id: ProfileId,
    pub slot: String,
}

impl CredentialKey {
    pub fn new(
        surface: SurfaceId,
        profile_id: ProfileId,
        slot: impl Into<String>,
    ) -> Result<Self, CredentialError> {
        let slot = slot.into();
        if slot.trim().is_empty() {
            return Err(CredentialError::InvalidSlot);
        }
        Ok(Self {
            surface,
            profile_id,
            slot,
        })
    }

    pub fn oauth_tokens(surface: SurfaceId, profile_id: ProfileId) -> Self {
        Self {
            surface,
            profile_id,
            slot: "oauth_tokens".to_owned(),
        }
    }

    fn account_name(&self) -> String {
        format!(
            "surface={};profile={};slot={}",
            encode_component(self.surface.as_str()),
            encode_component(self.profile_id.as_str()),
            encode_component(&self.slot)
        )
    }
}

pub trait CredentialStore: Send + Sync {
    fn load(&self, key: &CredentialKey) -> Result<Option<Vec<u8>>, CredentialError>;
    fn save(&self, key: &CredentialKey, secret: &[u8]) -> Result<(), CredentialError>;
    fn delete(&self, key: &CredentialKey) -> Result<bool, CredentialError>;
}

#[derive(Debug, Default)]
pub struct MemoryCredentialStore {
    values: Mutex<BTreeMap<CredentialKey, Vec<u8>>>,
}

impl CredentialStore for MemoryCredentialStore {
    fn load(&self, key: &CredentialKey) -> Result<Option<Vec<u8>>, CredentialError> {
        Ok(self
            .values
            .lock()
            .map_err(|_| CredentialError::Poisoned)?
            .get(key)
            .cloned())
    }

    fn save(&self, key: &CredentialKey, secret: &[u8]) -> Result<(), CredentialError> {
        self.values
            .lock()
            .map_err(|_| CredentialError::Poisoned)?
            .insert(key.clone(), secret.to_vec());
        Ok(())
    }

    fn delete(&self, key: &CredentialKey) -> Result<bool, CredentialError> {
        Ok(self
            .values
            .lock()
            .map_err(|_| CredentialError::Poisoned)?
            .remove(key)
            .is_some())
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct NativeCredentialStore;

impl NativeCredentialStore {
    pub fn new() -> Result<Self, CredentialError> {
        #[cfg(windows)]
        {
            let probe = CredentialKey::new(
                SurfaceId::new("sociarium-probe")
                    .map_err(|error| CredentialError::Backend(error.to_string()))?,
                ProfileId::new("sociarium-probe")
                    .map_err(|error| CredentialError::Backend(error.to_string()))?,
                "probe",
            )?;
            entry_for(&probe)?;
            Ok(Self)
        }

        #[cfg(not(windows))]
        {
            Err(CredentialError::UnsupportedPlatform)
        }
    }
}

#[cfg(windows)]
impl CredentialStore for NativeCredentialStore {
    fn load(&self, key: &CredentialKey) -> Result<Option<Vec<u8>>, CredentialError> {
        let entry = entry_for(key)?;
        match entry.get_secret() {
            Ok(secret) => Ok(Some(secret)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(error) => Err(CredentialError::Backend(error.to_string())),
        }
    }

    fn save(&self, key: &CredentialKey, secret: &[u8]) -> Result<(), CredentialError> {
        entry_for(key)?
            .set_secret(secret)
            .map_err(|error| CredentialError::Backend(error.to_string()))
    }

    fn delete(&self, key: &CredentialKey) -> Result<bool, CredentialError> {
        let entry = entry_for(key)?;
        match entry.delete_credential() {
            Ok(()) => Ok(true),
            Err(keyring::Error::NoEntry) => Ok(false),
            Err(error) => Err(CredentialError::Backend(error.to_string())),
        }
    }
}

#[cfg(not(windows))]
impl CredentialStore for NativeCredentialStore {
    fn load(&self, _key: &CredentialKey) -> Result<Option<Vec<u8>>, CredentialError> {
        Err(CredentialError::UnsupportedPlatform)
    }

    fn save(&self, _key: &CredentialKey, _secret: &[u8]) -> Result<(), CredentialError> {
        Err(CredentialError::UnsupportedPlatform)
    }

    fn delete(&self, _key: &CredentialKey) -> Result<bool, CredentialError> {
        Err(CredentialError::UnsupportedPlatform)
    }
}

#[cfg(windows)]
fn entry_for(key: &CredentialKey) -> Result<keyring::Entry, CredentialError> {
    keyring::Entry::new(SERVICE_NAME, &key.account_name())
        .map_err(|error| CredentialError::Backend(error.to_string()))
}

fn encode_component(value: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.') {
            encoded.push(char::from(byte));
        } else {
            encoded.push('%');
            encoded.push(char::from(HEX[(byte >> 4) as usize]));
            encoded.push(char::from(HEX[(byte & 0x0f) as usize]));
        }
    }
    encoded
}

#[derive(Debug, Error)]
pub enum CredentialError {
    #[error("credential slot cannot be blank")]
    InvalidSlot,
    #[error("native credential storage is not implemented for this platform")]
    UnsupportedPlatform,
    #[error("credential store lock was poisoned")]
    Poisoned,
    #[error("credential store backend error: {0}")]
    Backend(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(profile: &str) -> CredentialKey {
        CredentialKey::oauth_tokens(
            SurfaceId::new("x").unwrap(),
            ProfileId::new(profile).unwrap(),
        )
    }

    #[test]
    fn memory_store_is_profile_scoped() {
        let store = MemoryCredentialStore::default();
        let first = key("x-main");
        let second = key("x-alt");

        store.save(&first, b"first").unwrap();
        store.save(&second, b"second").unwrap();

        assert_eq!(store.load(&first).unwrap(), Some(b"first".to_vec()));
        assert_eq!(store.load(&second).unwrap(), Some(b"second".to_vec()));
        assert!(store.delete(&first).unwrap());
        assert_eq!(store.load(&first).unwrap(), None);
        assert_eq!(store.load(&second).unwrap(), Some(b"second".to_vec()));
    }

    #[test]
    fn account_names_escape_user_controlled_separators() {
        let key = CredentialKey::new(
            SurfaceId::new("some:surface").unwrap(),
            ProfileId::new("profile;one").unwrap(),
            "oauth/tokens",
        )
        .unwrap();

        assert_eq!(
            key.account_name(),
            "surface=some%3Asurface;profile=profile%3Bone;slot=oauth%2Ftokens"
        );
    }

    #[test]
    fn oauth_key_has_stable_slot() {
        let key = key("x-main");
        assert_eq!(key.slot, "oauth_tokens");
        assert!(key.account_name().contains("profile=x-main"));
    }
}
