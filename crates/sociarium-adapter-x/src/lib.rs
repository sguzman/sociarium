mod auth;
mod client;
mod models;
mod normalize;

use std::collections::BTreeSet;

use async_trait::async_trait;
use chrono::Utc;
use sociarium_adapter::{AdapterError, Capability, RawEvidence, SocialAdapter, SyncBatch};
use sociarium_core::{ObservationMeta, ProfileOwnership, SurfaceId, TrackedProfile};

pub use auth::{XOAuthConfig, XOAuthError, XOAuthSession, XTokenSet};
pub use client::{XApiClient, XApiError};

#[derive(Clone)]
pub struct XAdapter {
    client: Option<XApiClient>,
}

impl XAdapter {
    pub fn new() -> Self {
        Self { client: None }
    }

    pub fn authenticated(access_token: impl Into<String>) -> Result<Self, XApiError> {
        Ok(Self {
            client: Some(XApiClient::new(access_token)?),
        })
    }
}

impl Default for XAdapter {
    fn default() -> Self {
        Self::new()
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
        profile: &TrackedProfile,
        cursor: Option<&str>,
    ) -> Result<SyncBatch, AdapterError> {
        if profile.surface.as_str() != "x" {
            return Err(AdapterError::Data(format!(
                "X adapter cannot sync surface {}",
                profile.surface
            )));
        }
        if profile.ownership != ProfileOwnership::SelfOwned {
            return Err(AdapterError::Unsupported(
                "M0 X synchronization is limited to the authenticated self-owned profile".into(),
            ));
        }

        let client = self
            .client
            .as_ref()
            .ok_or(AdapterError::AuthenticationRequired)?;
        let observed_at = Utc::now();
        let acquisition_id = format!("x-{}-{}", profile.id, observed_at.timestamp_micros());
        let observation = ObservationMeta {
            surface: self.surface_id(),
            observed_at,
            acquisition_id: acquisition_id.clone(),
            schema_version: 1,
        };

        let me = client.get_me().await.map_err(map_api_error)?;
        if let Some(configured_remote_id) = &profile.remote_id {
            if configured_remote_id.as_str() != me.value.data.id.as_str() {
                return Err(AdapterError::Data(format!(
                    "configured X remote id {} does not match authenticated user {}",
                    configured_remote_id, me.value.data.id
                )));
            }
        }

        let user_id = me.value.data.id.clone();
        let username = me.value.data.username.clone();
        let posts = client
            .get_user_posts(&user_id, cursor)
            .await
            .map_err(map_api_error)?;

        let mut records = vec![
            normalize::normalize_profile(profile, &me.value.data, observation.clone())
                .map_err(AdapterError::Data)?,
        ];
        records.extend(
            normalize::normalize_posts(profile, &username, &posts.value, &observation)
                .map_err(AdapterError::Data)?,
        );

        let next_cursor = posts.value.meta.next_token.clone();
        let raw = vec![
            RawEvidence {
                media_type: "application/json".into(),
                bytes: me.bytes,
                suggested_path: Some(format!(
                    "x/{}/{acquisition_id}/authenticated-user.json",
                    profile.id
                )),
            },
            RawEvidence {
                media_type: "application/json".into(),
                bytes: posts.bytes,
                suggested_path: Some(format!("x/{}/{acquisition_id}/posts.json", profile.id)),
            },
        ];

        Ok(SyncBatch {
            records,
            raw,
            next_cursor,
        })
    }
}

fn map_api_error(error: XApiError) -> AdapterError {
    AdapterError::Remote(error.to_string())
}

#[cfg(test)]
mod tests {
    use sociarium_adapter::{Capability, SocialAdapter};
    use sociarium_core::{ProfileId, ProfileOwnership, SurfaceId};

    use super::*;

    #[test]
    fn x_is_only_an_adapter_identity() {
        let adapter = XAdapter::new();
        assert_eq!(adapter.surface_id().as_str(), "x");
        assert!(adapter.capabilities().contains(&Capability::Posts));
    }

    #[tokio::test]
    async fn unauthenticated_adapter_fails_before_network_access() {
        let adapter = XAdapter::new();
        let profile = TrackedProfile {
            id: ProfileId::new("x-main").unwrap(),
            surface: SurfaceId::new("x").unwrap(),
            remote_id: None,
            handle: Some("sguzman".into()),
            ownership: ProfileOwnership::SelfOwned,
            enabled: true,
        };

        assert!(matches!(
            adapter.sync_profile(&profile, None).await,
            Err(AdapterError::AuthenticationRequired)
        ));
    }
}
