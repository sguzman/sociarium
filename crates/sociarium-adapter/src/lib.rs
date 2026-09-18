use std::collections::BTreeSet;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sociarium_core::{SurfaceId, TrackedProfile};
use thiserror::Error;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    ProfileSnapshots,
    Posts,
    Replies,
    Reactions,
    Relationships,
    Collections,
    RemoteWrites,
}

pub use sociarium_acquisition::{AcquisitionBatch, RawEvidence};

/// Backward-compatible live-adapter name for the source-neutral acquisition envelope.
pub type SyncBatch = AcquisitionBatch;

#[derive(Debug, Error)]
pub enum AdapterError {
    #[error("adapter capability is not implemented: {0}")]
    Unsupported(String),
    #[error("authentication is required")]
    AuthenticationRequired,
    #[error("remote surface error: {0}")]
    Remote(String),
    #[error("adapter data error: {0}")]
    Data(String),
}

#[async_trait]
pub trait SocialAdapter: Send + Sync {
    fn surface_id(&self) -> SurfaceId;
    fn capabilities(&self) -> BTreeSet<Capability>;

    /// Whether a durable cursor returned by `sync_profile` means the current traversal has more
    /// pages to acquire immediately.
    ///
    /// The default preserves the simple adapter convention where any cursor means another page.
    /// Adapters with incremental high-water checkpoints may override this so a non-empty cursor can
    /// represent a completed traversal and seed the next incremental sync.
    fn cursor_has_more(&self, cursor: Option<&str>) -> Result<bool, AdapterError> {
        Ok(cursor.is_some())
    }

    async fn sync_profile(
        &self,
        profile: &TrackedProfile,
        cursor: Option<&str>,
    ) -> Result<SyncBatch, AdapterError>;
}
