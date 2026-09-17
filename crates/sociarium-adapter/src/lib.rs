use std::collections::BTreeSet;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sociarium_core::{NormalizedRecord, SurfaceId, TrackedProfile};
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

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RawEvidence {
    pub media_type: String,
    pub bytes: Vec<u8>,
    pub suggested_path: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SyncBatch {
    pub records: Vec<NormalizedRecord>,
    pub raw: Vec<RawEvidence>,
    pub next_cursor: Option<String>,
}

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

    async fn sync_profile(
        &self,
        profile: &TrackedProfile,
        cursor: Option<&str>,
    ) -> Result<SyncBatch, AdapterError>;
}
