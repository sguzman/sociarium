use std::collections::BTreeSet;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sociarium_acquisition::AcquisitionBatch;
use sociarium_core::{NormalizedRecord, ProfileId, RemoteId, SurfaceId, TrackedProfile};
use thiserror::Error;

pub const STORE_SCHEMA_VERSION: u32 = 1;
pub const CORPUS_REPOSITORY_SCHEMA_VERSION: u32 = 1;
pub const CORPUS_MARKER_FILE: &str = "sociarium-corpus.json";
pub const CORPUS_GITIGNORE_RULES: &[&str] = &["/indexes/", "/derived/", "**/.pending-*/"];

const CORPUS_KIND: &str = "sociarium-corpus";

static STAGING_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug)]
pub struct CorpusLayout {
    root: PathBuf,
}

impl CorpusLayout {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn acquisitions_dir(&self) -> PathBuf {
        self.root.join("acquisitions")
    }

    pub fn state_dir(&self) -> PathBuf {
        self.root.join("state")
    }

    pub fn derived_dir(&self) -> PathBuf {
        self.root.join("derived")
    }

    pub fn indexes_dir(&self) -> PathBuf {
        self.root.join("indexes")
    }

    pub fn marker_path(&self) -> PathBuf {
        self.root.join(CORPUS_MARKER_FILE)
    }

    pub fn ensure_dirs(&self) -> Result<(), StoreError> {
        for path in [
            self.acquisitions_dir(),
            self.state_dir().join("profiles"),
            self.derived_dir(),
            self.indexes_dir(),
        ] {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }

    fn acquisition_profile_dir(&self, profile: &TrackedProfile) -> PathBuf {
        self.acquisitions_dir()
            .join(encode_component(profile.surface.as_str()))
            .join(encode_component(profile.id.as_str()))
    }

    fn checkpoint_profile_dir(&self, profile: &TrackedProfile) -> PathBuf {
        self.state_dir()
            .join("profiles")
            .join(encode_component(profile.id.as_str()))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CorpusRepositoryMetadata {
    pub schema_version: u32,
    pub kind: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProfileBinding {
    Unbound,
    Bound(RemoteId),
    Conflicted(Vec<RemoteId>),
}

#[derive(Clone, Debug)]
pub struct CorpusStore {
    layout: CorpusLayout,
}

impl CorpusStore {
    pub fn open(root: impl Into<PathBuf>) -> Result<Self, StoreError> {
        let layout = CorpusLayout::new(root);
        layout.ensure_dirs()?;
        Ok(Self { layout })
    }

    pub fn initialize(root: impl Into<PathBuf>) -> Result<Self, StoreError> {
        let layout = CorpusLayout::new(root);
        let root = layout.root();

        if root.exists() && !root.is_dir() {
            return Err(StoreError::CorpusRootNotDirectory(root.to_path_buf()));
        }
        fs::create_dir_all(root)?;

        if layout.marker_path().exists() {
            validate_corpus_marker(&layout)?;
        } else {
            validate_initialization_target(root)?;
            let metadata = CorpusRepositoryMetadata {
                schema_version: CORPUS_REPOSITORY_SCHEMA_VERSION,
                kind: CORPUS_KIND.to_owned(),
            };
            write_new_json(&layout.marker_path(), &metadata)?;
        }

        layout.ensure_dirs()?;
        ensure_corpus_gitignore(root)?;
        Ok(Self { layout })
    }

    pub fn open_initialized(root: impl Into<PathBuf>) -> Result<Self, StoreError> {
        let layout = CorpusLayout::new(root);
        validate_corpus_marker(&layout)?;
        layout.ensure_dirs()?;
        Ok(Self { layout })
    }

    pub fn validate_initialized(
        root: impl Into<PathBuf>,
    ) -> Result<CorpusRepositoryMetadata, StoreError> {
        let layout = CorpusLayout::new(root);
        validate_corpus_marker(&layout)
    }

    pub fn layout(&self) -> &CorpusLayout {
        &self.layout
    }

    pub fn profile_binding(&self, profile: &TrackedProfile) -> Result<ProfileBinding, StoreError> {
        let profile_dir = self.layout.acquisition_profile_dir(profile);
        if !profile_dir.exists() {
            return Ok(ProfileBinding::Unbound);
        }

        let mut remote_ids = BTreeSet::new();
        for entry in fs::read_dir(profile_dir)? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }
            let name = entry.file_name();
            if name.to_string_lossy().starts_with(".pending-") {
                continue;
            }

            let acquisition_dir = entry.path();
            let manifest_path = acquisition_dir.join("manifest.json");
            if !manifest_path.is_file() {
                continue;
            }
            let manifest: AcquisitionManifest = read_json(&manifest_path)?;
            if manifest.profile_id != profile.id || manifest.surface != profile.surface {
                return Err(StoreError::CorruptManifest(manifest_path));
            }

            let records_path = acquisition_dir.join("normalized").join("records.jsonl");
            let reader = BufReader::new(File::open(&records_path)?);
            for line in reader.lines() {
                let line = line?;
                if line.trim().is_empty() {
                    continue;
                }
                let record: NormalizedRecord = serde_json::from_str(&line)?;
                let observation = record.observation();
                if record.profile_id() != &profile.id
                    || observation.surface != profile.surface
                    || observation.acquisition_id != manifest.acquisition_id
                    || observation.observed_at != manifest.observed_at
                {
                    return Err(StoreError::CorruptNormalizedRecord(records_path.clone()));
                }
                if let NormalizedRecord::ProfileSnapshot(snapshot) = record {
                    remote_ids.insert(snapshot.remote_id);
                }
            }
        }

        Ok(match remote_ids.len() {
            0 => ProfileBinding::Unbound,
            1 => ProfileBinding::Bound(
                remote_ids
                    .into_iter()
                    .next()
                    .expect("one binding exists after length check"),
            ),
            _ => ProfileBinding::Conflicted(remote_ids.into_iter().collect()),
        })
    }

    pub fn persist_acquisition_batch(
        &self,
        profile: &TrackedProfile,
        batch: &AcquisitionBatch,
        acquisition_source: &str,
    ) -> Result<PersistedAcquisition, StoreError> {
        self.persist_acquisition_files(profile, batch, acquisition_source)
    }

    fn persist_acquisition_files(
        &self,
        profile: &TrackedProfile,
        batch: &AcquisitionBatch,
        acquisition_source: &str,
    ) -> Result<PersistedAcquisition, StoreError> {
        if acquisition_source.trim().is_empty() {
            return Err(StoreError::InvalidAcquisitionSource);
        }
        let batch_meta = validate_batch(profile, batch)?;
        self.validate_profile_binding(profile, batch)?;
        let profile_dir = self.layout.acquisition_profile_dir(profile);
        fs::create_dir_all(&profile_dir)?;

        let encoded_acquisition = encode_component(&batch_meta.acquisition_id);
        let final_dir = profile_dir.join(&encoded_acquisition);
        if final_dir.exists() {
            return Err(StoreError::AcquisitionExists(
                batch_meta.acquisition_id.clone(),
            ));
        }

        let staging_dir = profile_dir.join(format!(
            ".pending-{encoded_acquisition}-{}-{}",
            std::process::id(),
            STAGING_COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&staging_dir)?;

        let result =
            self.write_staging_batch(&staging_dir, profile, batch, batch_meta, acquisition_source);
        let manifest = match result {
            Ok(manifest) => manifest,
            Err(error) => {
                let _ = fs::remove_dir_all(&staging_dir);
                return Err(error);
            }
        };

        fs::rename(&staging_dir, &final_dir)?;

        Ok(PersistedAcquisition {
            acquisition_dir: final_dir,
            manifest,
        })
    }

    /// Persist a live-adapter acquisition and advance its recoverable sync state.
    pub fn persist_sync_batch(
        &self,
        profile: &TrackedProfile,
        batch: &AcquisitionBatch,
    ) -> Result<PersistedBatch, StoreError> {
        let persisted = self.persist_acquisition_files(profile, batch, "live_adapter")?;
        let encoded_acquisition = encode_component(&persisted.manifest.acquisition_id);
        let state = ProfileSyncState::from_manifest(&persisted.manifest);
        let state_dir = self.layout.checkpoint_profile_dir(profile);
        fs::create_dir_all(&state_dir)?;
        let state_path = state_dir.join(format!("{encoded_acquisition}.json"));
        write_new_json(&state_path, &state)?;

        Ok(PersistedBatch {
            acquisition_dir: persisted.acquisition_dir,
            state_path,
            manifest: persisted.manifest,
        })
    }

    pub fn profile_state(
        &self,
        profile: &TrackedProfile,
    ) -> Result<Option<ProfileSyncState>, StoreError> {
        let profile_dir = self.layout.acquisition_profile_dir(profile);
        if !profile_dir.exists() {
            return Ok(None);
        }

        let mut latest: Option<ProfileSyncState> = None;
        for entry in fs::read_dir(profile_dir)? {
            let entry = entry?;
            if !entry.file_type()?.is_dir() {
                continue;
            }
            let name = entry.file_name();
            if name.to_string_lossy().starts_with(".pending-") {
                continue;
            }

            let manifest_path = entry.path().join("manifest.json");
            if !manifest_path.is_file() {
                continue;
            }
            let manifest: AcquisitionManifest = read_json(&manifest_path)?;
            if manifest.profile_id != profile.id || manifest.surface != profile.surface {
                return Err(StoreError::CorruptManifest(manifest_path));
            }
            if !is_sync_acquisition_source(&manifest.acquisition_source) {
                continue;
            }

            let candidate = ProfileSyncState::from_manifest(&manifest);
            let should_replace = latest.as_ref().is_none_or(|current| {
                (candidate.updated_at, candidate.last_acquisition_id.as_str())
                    > (current.updated_at, current.last_acquisition_id.as_str())
            });
            if should_replace {
                latest = Some(candidate);
            }
        }

        Ok(latest)
    }

    fn validate_profile_binding(
        &self,
        profile: &TrackedProfile,
        batch: &AcquisitionBatch,
    ) -> Result<(), StoreError> {
        let durable = match self.profile_binding(profile)? {
            ProfileBinding::Unbound => None,
            ProfileBinding::Bound(remote_id) => Some(remote_id),
            ProfileBinding::Conflicted(remote_ids) => {
                return Err(StoreError::ProfileBindingConflict {
                    profile_id: profile.id.to_string(),
                    remote_ids: remote_ids
                        .into_iter()
                        .map(|remote_id| remote_id.to_string())
                        .collect(),
                });
            }
        };

        if let (Some(configured), Some(durable)) = (&profile.remote_id, durable.as_ref()) {
            if configured != durable {
                return Err(StoreError::ConfiguredRemoteIdConflict {
                    profile_id: profile.id.to_string(),
                    configured: configured.to_string(),
                    durable: durable.to_string(),
                });
            }
        }

        let expected = profile.remote_id.as_ref().or(durable.as_ref());
        let batch_remote_ids = batch
            .records
            .iter()
            .filter_map(|record| match record {
                NormalizedRecord::ProfileSnapshot(snapshot) => Some(snapshot.remote_id.clone()),
                NormalizedRecord::Post(_) => None,
            })
            .collect::<BTreeSet<_>>();

        if batch_remote_ids.len() > 1 {
            return Err(StoreError::BatchProfileBindingConflict {
                profile_id: profile.id.to_string(),
                remote_ids: batch_remote_ids
                    .into_iter()
                    .map(|remote_id| remote_id.to_string())
                    .collect(),
            });
        }

        if let (Some(expected), Some(actual)) = (expected, batch_remote_ids.iter().next()) {
            if expected != actual {
                return Err(StoreError::ProfileRemoteIdMismatch {
                    profile_id: profile.id.to_string(),
                    expected: expected.to_string(),
                    actual: actual.to_string(),
                });
            }
        }

        Ok(())
    }

    fn write_staging_batch(
        &self,
        staging_dir: &Path,
        profile: &TrackedProfile,
        batch: &AcquisitionBatch,
        batch_meta: BatchMeta,
        acquisition_source: &str,
    ) -> Result<AcquisitionManifest, StoreError> {
        let raw_dir = staging_dir.join("raw");
        let normalized_dir = staging_dir.join("normalized");
        fs::create_dir_all(&raw_dir)?;
        fs::create_dir_all(&normalized_dir)?;

        let mut raw_paths = BTreeSet::new();
        let mut raw_files = Vec::with_capacity(batch.raw.len());
        for (index, evidence) in batch.raw.iter().enumerate() {
            let relative_path = evidence
                .suggested_path
                .as_deref()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from(format!("raw-{index:04}.bin")));
            validate_relative_path(&relative_path)?;
            if !raw_paths.insert(relative_path.clone()) {
                return Err(StoreError::DuplicateRawPath(relative_path));
            }

            let target = raw_dir.join(&relative_path);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&target, &evidence.bytes)?;

            raw_files.push(RawFileManifest {
                path: portable_path(&relative_path),
                media_type: evidence.media_type.clone(),
                byte_len: evidence.bytes.len() as u64,
                sha256: sha256_hex(&evidence.bytes),
            });
        }

        let records_path = normalized_dir.join("records.jsonl");
        let records_file = File::create(&records_path)?;
        let mut writer = BufWriter::new(records_file);
        for record in &batch.records {
            serde_json::to_writer(&mut writer, record)?;
            writer.write_all(b"\n")?;
        }
        writer.flush()?;
        writer.get_ref().sync_all()?;

        let manifest = AcquisitionManifest {
            schema_version: STORE_SCHEMA_VERSION,
            acquisition_source: acquisition_source.to_owned(),
            acquisition_id: batch_meta.acquisition_id,
            profile_id: profile.id.clone(),
            surface: profile.surface.clone(),
            observed_at: batch_meta.observed_at,
            record_count: batch.records.len(),
            raw_files,
            next_cursor: batch.next_cursor.clone(),
        };
        write_new_json(&staging_dir.join("manifest.json"), &manifest)?;
        Ok(manifest)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RawFileManifest {
    pub path: String,
    pub media_type: String,
    pub byte_len: u64,
    pub sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AcquisitionManifest {
    pub schema_version: u32,
    #[serde(default = "legacy_acquisition_source")]
    pub acquisition_source: String,
    pub acquisition_id: String,
    pub profile_id: ProfileId,
    pub surface: SurfaceId,
    pub observed_at: DateTime<Utc>,
    pub record_count: usize,
    pub raw_files: Vec<RawFileManifest>,
    pub next_cursor: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ProfileSyncState {
    pub schema_version: u32,
    pub profile_id: ProfileId,
    pub surface: SurfaceId,
    pub last_acquisition_id: String,
    pub updated_at: DateTime<Utc>,
    pub cursor: Option<String>,
}

impl ProfileSyncState {
    fn from_manifest(manifest: &AcquisitionManifest) -> Self {
        Self {
            schema_version: STORE_SCHEMA_VERSION,
            profile_id: manifest.profile_id.clone(),
            surface: manifest.surface.clone(),
            last_acquisition_id: manifest.acquisition_id.clone(),
            updated_at: manifest.observed_at,
            cursor: manifest.next_cursor.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PersistedAcquisition {
    pub acquisition_dir: PathBuf,
    pub manifest: AcquisitionManifest,
}

#[derive(Clone, Debug)]
pub struct PersistedBatch {
    pub acquisition_dir: PathBuf,
    pub state_path: PathBuf,
    pub manifest: AcquisitionManifest,
}

#[derive(Clone, Debug)]
struct BatchMeta {
    acquisition_id: String,
    observed_at: DateTime<Utc>,
}

fn legacy_acquisition_source() -> String {
    "legacy_sync".to_owned()
}

fn is_sync_acquisition_source(value: &str) -> bool {
    matches!(value, "legacy_sync" | "live_adapter")
}

fn validate_corpus_marker(layout: &CorpusLayout) -> Result<CorpusRepositoryMetadata, StoreError> {
    let root = layout.root();
    if !root.is_dir() {
        return Err(StoreError::CorpusNotInitialized(root.to_path_buf()));
    }
    let marker = layout.marker_path();
    if !marker.is_file() {
        return Err(StoreError::CorpusNotInitialized(root.to_path_buf()));
    }
    let metadata: CorpusRepositoryMetadata = read_json(&marker)?;
    if metadata.kind != CORPUS_KIND {
        return Err(StoreError::InvalidCorpusKind {
            path: marker,
            found: metadata.kind,
        });
    }
    if metadata.schema_version != CORPUS_REPOSITORY_SCHEMA_VERSION {
        return Err(StoreError::UnsupportedCorpusRepositorySchema {
            path: marker,
            found: metadata.schema_version,
            expected: CORPUS_REPOSITORY_SCHEMA_VERSION,
        });
    }
    Ok(metadata)
}

fn validate_initialization_target(root: &Path) -> Result<(), StoreError> {
    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let name = entry.file_name();
        if name == ".git" || name == ".gitignore" {
            continue;
        }
        return Err(StoreError::CorpusInitTargetNotEmpty(root.to_path_buf()));
    }
    Ok(())
}

fn ensure_corpus_gitignore(root: &Path) -> Result<(), StoreError> {
    let path = root.join(".gitignore");
    let mut content = if path.exists() {
        fs::read_to_string(&path)?
    } else {
        String::new()
    };

    let mut changed = false;
    for rule in CORPUS_GITIGNORE_RULES {
        if !content.lines().any(|line| line.trim() == *rule) {
            if !content.is_empty() && !content.ends_with('\n') {
                content.push('\n');
            }
            content.push_str(rule);
            content.push('\n');
            changed = true;
        }
    }

    if changed || !path.exists() {
        fs::write(path, content)?;
    }
    Ok(())
}

fn validate_batch(
    profile: &TrackedProfile,
    batch: &AcquisitionBatch,
) -> Result<BatchMeta, StoreError> {
    let first = batch.records.first().ok_or(StoreError::EmptyBatch)?;
    let first_observation = first.observation();
    if first_observation.acquisition_id.trim().is_empty() {
        return Err(StoreError::InvalidAcquisitionId);
    }

    for record in &batch.records {
        if record.profile_id() != &profile.id {
            return Err(StoreError::ProfileMismatch {
                expected: profile.id.to_string(),
                actual: record.profile_id().to_string(),
            });
        }
        let observation = record.observation();
        if observation.surface != profile.surface {
            return Err(StoreError::SurfaceMismatch {
                expected: profile.surface.to_string(),
                actual: observation.surface.to_string(),
            });
        }
        if observation.acquisition_id != first_observation.acquisition_id
            || observation.observed_at != first_observation.observed_at
        {
            return Err(StoreError::MixedAcquisition);
        }
    }

    Ok(BatchMeta {
        acquisition_id: first_observation.acquisition_id.clone(),
        observed_at: first_observation.observed_at,
    })
}

fn validate_relative_path(path: &Path) -> Result<(), StoreError> {
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(StoreError::UnsafeRawPath(path.to_path_buf()));
    }
    if path
        .components()
        .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(StoreError::UnsafeRawPath(path.to_path_buf()));
    }
    Ok(())
}

fn portable_path(path: &Path) -> String {
    path.components()
        .filter_map(|component| match component {
            Component::Normal(value) => Some(value.to_string_lossy()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("/")
}

fn encode_component(value: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_') {
            encoded.push(char::from(byte));
        } else {
            encoded.push('%');
            encoded.push(char::from(HEX[(byte >> 4) as usize]));
            encoded.push(char::from(HEX[(byte & 0x0f) as usize]));
        }
    }
    encoded
}

fn sha256_hex(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn write_new_json(path: &Path, value: &impl Serialize) -> Result<(), StoreError> {
    if path.exists() {
        return Err(StoreError::PathExists(path.to_path_buf()));
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let bytes = serde_json::to_vec_pretty(value)?;
    fs::write(path, bytes)?;
    Ok(())
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, StoreError> {
    let bytes = fs::read(path)?;
    Ok(serde_json::from_slice(&bytes)?)
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("filesystem error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("corpus root is not a directory: {}", .0.display())]
    CorpusRootNotDirectory(PathBuf),
    #[error("directory is not an initialized Sociarium corpus: {}", .0.display())]
    CorpusNotInitialized(PathBuf),
    #[error("refusing to initialize non-empty corpus target: {}", .0.display())]
    CorpusInitTargetNotEmpty(PathBuf),
    #[error("invalid Sociarium corpus kind in {}: {found}", path.display())]
    InvalidCorpusKind { path: PathBuf, found: String },
    #[error(
        "unsupported Sociarium corpus repository schema {found} in {}; expected {expected}",
        path.display()
    )]
    UnsupportedCorpusRepositorySchema {
        path: PathBuf,
        found: u32,
        expected: u32,
    },
    #[error("acquisition batch contains no normalized records")]
    EmptyBatch,
    #[error("acquisition batch acquisition id cannot be blank")]
    InvalidAcquisitionId,
    #[error("acquisition source cannot be blank")]
    InvalidAcquisitionSource,
    #[error("acquisition batch mixes records from different acquisitions")]
    MixedAcquisition,
    #[error("acquisition batch profile mismatch: expected {expected}, got {actual}")]
    ProfileMismatch { expected: String, actual: String },
    #[error("acquisition batch surface mismatch: expected {expected}, got {actual}")]
    SurfaceMismatch { expected: String, actual: String },
    #[error("unsafe raw evidence path: {}", .0.display())]
    UnsafeRawPath(PathBuf),
    #[error("duplicate raw evidence path: {}", .0.display())]
    DuplicateRawPath(PathBuf),
    #[error("acquisition already exists: {0}")]
    AcquisitionExists(String),
    #[error("path already exists: {}", .0.display())]
    PathExists(PathBuf),
    #[error("acquisition manifest does not match its profile directory: {}", .0.display())]
    CorruptManifest(PathBuf),
    #[error("normalized record does not match its acquisition/profile scope: {}", .0.display())]
    CorruptNormalizedRecord(PathBuf),
    #[error("durable profile binding is conflicted for {profile_id}: {remote_ids:?}")]
    ProfileBindingConflict {
        profile_id: String,
        remote_ids: Vec<String>,
    },
    #[error("sync batch contains conflicting remote profile ids for {profile_id}: {remote_ids:?}")]
    BatchProfileBindingConflict {
        profile_id: String,
        remote_ids: Vec<String>,
    },
    #[error(
        "configured remote profile id conflicts with durable binding for {profile_id}: configured={configured} durable={durable}"
    )]
    ConfiguredRemoteIdConflict {
        profile_id: String,
        configured: String,
        durable: String,
    },
    #[error("remote profile id mismatch for {profile_id}: expected={expected} actual={actual}")]
    ProfileRemoteIdMismatch {
        profile_id: String,
        expected: String,
        actual: String,
    },
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::fs::OpenOptions;

    use chrono::{TimeZone, Utc};
    use sociarium_acquisition::RawEvidence;
    use sociarium_core::{
        NormalizedRecord, ObservationMeta, ProfileOwnership, ProfileSnapshot, RemoteId,
    };

    use super::*;

    struct TempCorpus(PathBuf);

    impl TempCorpus {
        fn new(name: &str) -> Self {
            let root = std::env::temp_dir().join(format!(
                "sociarium-{name}-{}-{}",
                std::process::id(),
                STAGING_COUNTER.fetch_add(1, Ordering::Relaxed)
            ));
            let _ = fs::remove_dir_all(&root);
            Self(root)
        }
    }

    impl Drop for TempCorpus {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn profile() -> TrackedProfile {
        TrackedProfile {
            id: ProfileId::new("x-main").unwrap(),
            surface: SurfaceId::new("x").unwrap(),
            remote_id: Some(RemoteId::new("6679733").unwrap()),
            handle: Some("sguzman".to_owned()),
            ownership: ProfileOwnership::SelfOwned,
            enabled: true,
        }
    }

    fn unbound_profile() -> TrackedProfile {
        let mut profile = profile();
        profile.remote_id = None;
        profile
    }

    fn batch_for(
        raw_path: &str,
        remote_id: &str,
        handle: &str,
        acquisition_id: &str,
        hour: u32,
    ) -> AcquisitionBatch {
        let observation = ObservationMeta {
            surface: SurfaceId::new("x").unwrap(),
            observed_at: Utc.with_ymd_and_hms(2026, 9, 17, hour, 0, 0).unwrap(),
            acquisition_id: acquisition_id.to_owned(),
            schema_version: 1,
        };
        let record = NormalizedRecord::ProfileSnapshot(ProfileSnapshot {
            profile_id: ProfileId::new("x-main").unwrap(),
            remote_id: RemoteId::new(remote_id).unwrap(),
            handle: Some(handle.to_owned()),
            display_name: Some("Salvador".to_owned()),
            bio: None,
            avatar_url: None,
            metrics: BTreeMap::new(),
            observation,
            extensions: BTreeMap::new(),
        });

        AcquisitionBatch {
            records: vec![record],
            raw: vec![RawEvidence {
                media_type: "application/json".to_owned(),
                bytes: format!(r#"{{"data":{{"id":"{remote_id}"}}}}"#).into_bytes(),
                suggested_path: Some(raw_path.to_owned()),
            }],
            next_cursor: Some("NEXT-PAGE".to_owned()),
        }
    }

    fn batch(raw_path: &str) -> AcquisitionBatch {
        batch_for(raw_path, "6679733", "sguzman", "acq-001", 14)
    }

    #[test]
    fn initializes_git_safe_corpus_layout_and_marker() {
        let temp = TempCorpus::new("initialize");
        let store = CorpusStore::initialize(&temp.0).unwrap();

        assert!(store.layout().marker_path().is_file());
        assert!(store.layout().acquisitions_dir().is_dir());
        assert!(store.layout().state_dir().join("profiles").is_dir());
        assert!(store.layout().derived_dir().is_dir());
        assert!(store.layout().indexes_dir().is_dir());

        let metadata = CorpusStore::validate_initialized(&temp.0).unwrap();
        assert_eq!(metadata.schema_version, CORPUS_REPOSITORY_SCHEMA_VERSION);
        assert_eq!(metadata.kind, CORPUS_KIND);

        let gitignore = fs::read_to_string(temp.0.join(".gitignore")).unwrap();
        for rule in CORPUS_GITIGNORE_RULES {
            assert!(gitignore.lines().any(|line| line.trim() == *rule));
        }
        assert!(!gitignore.contains("acquisitions/"));
        assert!(!gitignore.contains("state/"));
    }

    #[test]
    fn corpus_initialization_is_idempotent_and_preserves_custom_gitignore() {
        let temp = TempCorpus::new("initialize-idempotent");
        fs::create_dir_all(&temp.0).unwrap();
        fs::create_dir(temp.0.join(".git")).unwrap();
        fs::write(temp.0.join(".gitignore"), "# operator rule\n*.bak\n").unwrap();

        CorpusStore::initialize(&temp.0).unwrap();
        CorpusStore::initialize(&temp.0).unwrap();

        let gitignore = fs::read_to_string(temp.0.join(".gitignore")).unwrap();
        assert!(gitignore.contains("# operator rule"));
        assert!(gitignore.contains("*.bak"));
        for rule in CORPUS_GITIGNORE_RULES {
            assert_eq!(
                gitignore
                    .lines()
                    .filter(|line| line.trim() == *rule)
                    .count(),
                1
            );
        }
    }

    #[test]
    fn refuses_to_initialize_unrelated_nonempty_directory() {
        let temp = TempCorpus::new("initialize-nonempty");
        fs::create_dir_all(&temp.0).unwrap();
        fs::write(temp.0.join("README.md"), "not a corpus").unwrap();

        let error = CorpusStore::initialize(&temp.0).unwrap_err();
        assert!(matches!(error, StoreError::CorpusInitTargetNotEmpty(_)));
        assert!(!temp.0.join(CORPUS_MARKER_FILE).exists());
    }

    #[test]
    fn open_initialized_rejects_unmarked_directory() {
        let temp = TempCorpus::new("open-uninitialized");
        CorpusStore::open(&temp.0).unwrap();

        let error = CorpusStore::open_initialized(&temp.0).unwrap_err();
        assert!(matches!(error, StoreError::CorpusNotInitialized(_)));
    }

    #[test]
    fn persists_one_atomic_acquisition_bundle_and_recovers_cursor_from_manifest() {
        let temp = TempCorpus::new("persist");
        let store = CorpusStore::open(&temp.0).unwrap();
        let profile = profile();
        let persisted = store
            .persist_sync_batch(&profile, &batch("api/me.json"))
            .unwrap();

        assert!(persisted.acquisition_dir.join("manifest.json").is_file());
        assert!(
            persisted
                .acquisition_dir
                .join("normalized/records.jsonl")
                .is_file()
        );
        assert!(persisted.acquisition_dir.join("raw/api/me.json").is_file());
        assert!(persisted.state_path.is_file());
        assert_eq!(persisted.manifest.raw_files[0].byte_len, 25);

        fs::remove_file(&persisted.state_path).unwrap();
        let recovered = store.profile_state(&profile).unwrap().unwrap();
        assert_eq!(recovered.cursor.as_deref(), Some("NEXT-PAGE"));
        assert_eq!(recovered.last_acquisition_id, "acq-001");
    }

    #[test]
    fn offline_acquisition_does_not_replace_live_sync_state() {
        let temp = TempCorpus::new("offline-does-not-replace-sync");
        let store = CorpusStore::open(&temp.0).unwrap();
        let profile = unbound_profile();

        let live = batch_for("api/me.json", "6679733", "sguzman", "live-acq", 14);
        store.persist_sync_batch(&profile, &live).unwrap();

        let mut archive = batch_for(
            "x-archive/account.js",
            "6679733",
            "sguzman",
            "archive-acq",
            15,
        );
        archive.next_cursor = None;
        let persisted = store
            .persist_acquisition_batch(&profile, &archive, "x_account_archive")
            .unwrap();
        assert_eq!(persisted.manifest.acquisition_source, "x_account_archive");

        let state = store.profile_state(&profile).unwrap().unwrap();
        assert_eq!(state.last_acquisition_id, "live-acq");
        assert_eq!(state.cursor.as_deref(), Some("NEXT-PAGE"));
    }

    #[test]
    fn reconstructs_binding_and_accepts_handle_change_for_same_remote_id() {
        let temp = TempCorpus::new("binding");
        let store = CorpusStore::open(&temp.0).unwrap();
        let profile = unbound_profile();

        assert_eq!(
            store.profile_binding(&profile).unwrap(),
            ProfileBinding::Unbound
        );

        store
            .persist_sync_batch(
                &profile,
                &batch_for("api/me-1.json", "6679733", "old-handle", "acq-001", 14),
            )
            .unwrap();
        assert_eq!(
            store.profile_binding(&profile).unwrap(),
            ProfileBinding::Bound(RemoteId::new("6679733").unwrap())
        );

        store
            .persist_sync_batch(
                &profile,
                &batch_for("api/me-2.json", "6679733", "new-handle", "acq-002", 15),
            )
            .unwrap();
        assert_eq!(
            store.profile_binding(&profile).unwrap(),
            ProfileBinding::Bound(RemoteId::new("6679733").unwrap())
        );
    }

    #[test]
    fn persistence_rejects_remote_identity_change_before_commit() {
        let temp = TempCorpus::new("binding-mismatch");
        let store = CorpusStore::open(&temp.0).unwrap();
        let profile = unbound_profile();

        store
            .persist_sync_batch(
                &profile,
                &batch_for("api/me-1.json", "6679733", "sguzman", "acq-001", 14),
            )
            .unwrap();

        let error = store
            .persist_sync_batch(
                &profile,
                &batch_for("api/me-2.json", "999999", "other", "acq-002", 15),
            )
            .unwrap_err();
        assert!(matches!(error, StoreError::ProfileRemoteIdMismatch { .. }));

        let completed = fs::read_dir(store.layout().acquisition_profile_dir(&profile))
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_dir()))
            .count();
        assert_eq!(completed, 1);
    }

    #[test]
    fn configured_remote_id_must_agree_with_durable_binding() {
        let temp = TempCorpus::new("configured-binding-mismatch");
        let store = CorpusStore::open(&temp.0).unwrap();
        let unbound = unbound_profile();

        store
            .persist_sync_batch(
                &unbound,
                &batch_for("api/me-1.json", "6679733", "sguzman", "acq-001", 14),
            )
            .unwrap();

        let mut configured = unbound.clone();
        configured.remote_id = Some(RemoteId::new("999999").unwrap());
        let error = store
            .persist_sync_batch(
                &configured,
                &batch_for("api/me-2.json", "999999", "other", "acq-002", 15),
            )
            .unwrap_err();

        assert!(matches!(
            error,
            StoreError::ConfiguredRemoteIdConflict { .. }
        ));
    }

    #[test]
    fn contradictory_durable_profile_snapshots_are_reported_as_conflicted() {
        let temp = TempCorpus::new("binding-conflict");
        let store = CorpusStore::open(&temp.0).unwrap();
        let profile = unbound_profile();
        let persisted = store
            .persist_sync_batch(
                &profile,
                &batch_for("api/me-1.json", "6679733", "sguzman", "acq-001", 14),
            )
            .unwrap();

        let conflicting = batch_for("unused.json", "999999", "other", "acq-001", 14)
            .records
            .into_iter()
            .next()
            .unwrap();
        let records_path = persisted
            .acquisition_dir
            .join("normalized")
            .join("records.jsonl");
        let mut file = OpenOptions::new().append(true).open(records_path).unwrap();
        serde_json::to_writer(&mut file, &conflicting).unwrap();
        file.write_all(b"\n").unwrap();

        let ProfileBinding::Conflicted(remote_ids) = store.profile_binding(&profile).unwrap()
        else {
            panic!("expected conflicted profile binding");
        };
        assert_eq!(
            remote_ids,
            vec![
                RemoteId::new("6679733").unwrap(),
                RemoteId::new("999999").unwrap()
            ]
        );
    }

    #[test]
    fn rejects_raw_path_traversal_and_cleans_staging_directory() {
        let temp = TempCorpus::new("traversal");
        let store = CorpusStore::open(&temp.0).unwrap();
        let profile = profile();

        let error = store
            .persist_sync_batch(&profile, &batch("../escape.json"))
            .unwrap_err();
        assert!(matches!(error, StoreError::UnsafeRawPath(_)));
        assert!(!temp.0.join("escape.json").exists());

        let acquisition_parent = store.layout().acquisition_profile_dir(&profile);
        let leftovers = fs::read_dir(acquisition_parent)
            .unwrap()
            .filter_map(Result::ok)
            .count();
        assert_eq!(leftovers, 0);
    }
}
