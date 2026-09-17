use std::collections::BTreeSet;

use async_trait::async_trait;
use sociarium_adapter::{AdapterError, Capability, SocialAdapter, SyncBatch};
use sociarium_core::{SurfaceId, TrackedProfile};

#[derive(Clone, Debug, Default)]
pub struct XAdapter;

impl XAdapter {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl SocialAdapter for XAdapter {
    fn surface_id(&self) -> SurfaceId {
        SurfaceId::new("x").expect("static surface id is valid")
    }

    fn capabilities(&self) -> BTreeSet<Capability> {
        [Capability::ProfileSnapshots, Capability::Posts]
            .into_iter()
            .collect()
    }

    async fn sync_profile(
        &self,
        _profile: &TrackedProfile,
        _cursor: Option<&str>,
    ) -> Result<SyncBatch, AdapterError> {
        Err(AdapterError::Unsupported(
            "X authenticated sync is the next M0 implementation step".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use sociarium_adapter::{Capability, SocialAdapter};

    use super::*;

    #[test]
    fn x_is_only_an_adapter_identity() {
        let adapter = XAdapter::new();
        assert_eq!(adapter.surface_id().as_str(), "x");
        assert!(adapter.capabilities().contains(&Capability::Posts));
    }
}
