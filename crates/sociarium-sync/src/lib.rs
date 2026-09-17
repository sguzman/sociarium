use std::collections::BTreeSet;

use sociarium_adapter::{AdapterError, SocialAdapter};
use sociarium_core::TrackedProfile;
use sociarium_store::{CorpusStore, StoreError};
use thiserror::Error;

const DEFAULT_MAX_PAGES: usize = 10_000;

#[derive(Clone, Copy, Debug)]
pub struct SyncOptions {
    pub max_pages: usize,
}

impl Default for SyncOptions {
    fn default() -> Self {
        Self {
            max_pages: DEFAULT_MAX_PAGES,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SyncReport {
    pub pages_persisted: usize,
    pub records_persisted: usize,
    pub raw_evidence_objects: usize,
    pub started_from_prior_state: bool,
    pub final_cursor: Option<String>,
}

pub async fn sync_profile<A: SocialAdapter>(
    adapter: &A,
    profile: &TrackedProfile,
    store: &CorpusStore,
    options: SyncOptions,
) -> Result<SyncReport, SyncError> {
    if !profile.enabled {
        return Err(SyncError::ProfileDisabled(profile.id.to_string()));
    }
    if adapter.surface_id() != profile.surface {
        return Err(SyncError::SurfaceMismatch {
            profile: profile.surface.to_string(),
            adapter: adapter.surface_id().to_string(),
        });
    }
    if options.max_pages == 0 {
        return Err(SyncError::InvalidMaxPages);
    }

    let prior_state = store.profile_state(profile)?;
    let started_from_prior_state = prior_state.is_some();
    let mut cursor = prior_state.and_then(|state| state.cursor);
    let mut seen_cursors = BTreeSet::new();
    let mut pages_persisted = 0;
    let mut records_persisted = 0;
    let mut raw_evidence_objects = 0;

    for _ in 0..options.max_pages {
        if let Some(value) = cursor.as_ref()
            && !seen_cursors.insert(value.clone())
        {
            return Err(SyncError::CursorCycle(value.clone()));
        }

        let input_cursor = cursor.clone();
        let batch = adapter
            .sync_profile(profile, input_cursor.as_deref())
            .await?;
        let has_more = adapter.cursor_has_more(batch.next_cursor.as_deref())?;

        if has_more && batch.next_cursor == input_cursor {
            return Err(SyncError::CursorDidNotAdvance(
                input_cursor.unwrap_or_else(|| "<none>".to_owned()),
            ));
        }

        records_persisted += batch.records.len();
        raw_evidence_objects += batch.raw.len();
        cursor = batch.next_cursor.clone();
        store.persist_sync_batch(profile, &batch)?;
        pages_persisted += 1;

        if !has_more {
            return Ok(SyncReport {
                pages_persisted,
                records_persisted,
                raw_evidence_objects,
                started_from_prior_state,
                final_cursor: cursor,
            });
        }
    }

    Err(SyncError::MaxPagesExceeded(options.max_pages))
}

#[derive(Debug, Error)]
pub enum SyncError {
    #[error("profile is disabled: {0}")]
    ProfileDisabled(String),
    #[error("adapter surface {adapter} does not match profile surface {profile}")]
    SurfaceMismatch { profile: String, adapter: String },
    #[error("max_pages must be greater than zero")]
    InvalidMaxPages,
    #[error("adapter cursor did not advance: {0}")]
    CursorDidNotAdvance(String),
    #[error("adapter cursor cycle detected: {0}")]
    CursorCycle(String),
    #[error("sync exceeded configured page limit: {0}")]
    MaxPagesExceeded(usize),
    #[error(transparent)]
    Adapter(#[from] AdapterError),
    #[error(transparent)]
    Store(#[from] StoreError),
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    use async_trait::async_trait;
    use chrono::{TimeZone, Utc};
    use sociarium_adapter::{Capability, RawEvidence, SyncBatch};
    use sociarium_core::{
        NormalizedRecord, ObservationMeta, ProfileId, ProfileOwnership, ProfileSnapshot, RemoteId,
        SurfaceId,
    };

    use super::*;

    static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

    struct TempCorpus(PathBuf);

    impl TempCorpus {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "sociarium-sync-test-{}-{}",
                std::process::id(),
                TEST_COUNTER.fetch_add(1, Ordering::Relaxed)
            ));
            let _ = std::fs::remove_dir_all(&path);
            Self(path)
        }
    }

    impl Drop for TempCorpus {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.0);
        }
    }

    struct FakeAdapter;

    #[async_trait]
    impl SocialAdapter for FakeAdapter {
        fn surface_id(&self) -> SurfaceId {
            SurfaceId::new("fake").unwrap()
        }

        fn capabilities(&self) -> BTreeSet<Capability> {
            [Capability::ProfileSnapshots].into_iter().collect()
        }

        fn cursor_has_more(&self, cursor: Option<&str>) -> Result<bool, AdapterError> {
            Ok(cursor != Some("done"))
        }

        async fn sync_profile(
            &self,
            profile: &TrackedProfile,
            cursor: Option<&str>,
        ) -> Result<SyncBatch, AdapterError> {
            let (acquisition_id, next_cursor, hour) = match cursor {
                None => ("acq-1", "page-2", 13),
                Some("page-2") => ("acq-2", "done", 14),
                other => {
                    return Err(AdapterError::Data(format!(
                        "unexpected test cursor: {other:?}"
                    )));
                }
            };
            let observation = ObservationMeta {
                surface: profile.surface.clone(),
                observed_at: Utc.with_ymd_and_hms(2026, 9, 17, hour, 0, 0).unwrap(),
                acquisition_id: acquisition_id.to_owned(),
                schema_version: 1,
            };
            let record = NormalizedRecord::ProfileSnapshot(ProfileSnapshot {
                profile_id: profile.id.clone(),
                remote_id: RemoteId::new("42").unwrap(),
                handle: Some("example".to_owned()),
                display_name: None,
                bio: None,
                avatar_url: None,
                metrics: BTreeMap::new(),
                observation,
                extensions: BTreeMap::new(),
            });

            Ok(SyncBatch {
                records: vec![record],
                raw: vec![RawEvidence {
                    media_type: "application/json".to_owned(),
                    bytes: b"{}".to_vec(),
                    suggested_path: Some("response.json".to_owned()),
                }],
                next_cursor: Some(next_cursor.to_owned()),
            })
        }
    }

    fn profile() -> TrackedProfile {
        TrackedProfile {
            id: ProfileId::new("fake-main").unwrap(),
            surface: SurfaceId::new("fake").unwrap(),
            remote_id: None,
            handle: Some("example".to_owned()),
            ownership: ProfileOwnership::SelfOwned,
            enabled: true,
        }
    }

    #[tokio::test]
    async fn persists_each_page_before_advancing_to_terminal_checkpoint() {
        let temp = TempCorpus::new();
        let store = CorpusStore::open(&temp.0).unwrap();
        let profile = profile();

        let report = sync_profile(&FakeAdapter, &profile, &store, SyncOptions::default())
            .await
            .unwrap();

        assert_eq!(report.pages_persisted, 2);
        assert_eq!(report.records_persisted, 2);
        assert_eq!(report.raw_evidence_objects, 2);
        assert_eq!(report.final_cursor.as_deref(), Some("done"));

        let state = store.profile_state(&profile).unwrap().unwrap();
        assert_eq!(state.cursor.as_deref(), Some("done"));
        assert_eq!(state.last_acquisition_id, "acq-2");
    }
}
