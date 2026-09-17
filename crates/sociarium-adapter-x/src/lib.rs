mod auth;
mod client;
mod models;
mod normalize;

use std::cmp::Ordering;
use std::collections::BTreeSet;

use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use sociarium_adapter::{AdapterError, Capability, RawEvidence, SocialAdapter, SyncBatch};
use sociarium_core::{ObservationMeta, ProfileOwnership, SurfaceId, TrackedProfile};

pub use auth::{XOAuthConfig, XOAuthError, XOAuthSession, XStoredTokens, XTokenSet};
pub use client::{XApiClient, XApiError};

const X_CURSOR_VERSION: u32 = 1;

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

    fn cursor_has_more(&self, cursor: Option<&str>) -> Result<bool, AdapterError> {
        Ok(parse_cursor(cursor)
            .map_err(AdapterError::Data)?
            .pagination_token
            .is_some())
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
        let cursor = parse_cursor(cursor).map_err(AdapterError::Data)?;
        let observed_at = Utc::now();
        let acquisition_id = format!("x-{}-{}", profile.id, observed_at.timestamp_micros());
        let observation = ObservationMeta {
            surface: self.surface_id(),
            observed_at,
            acquisition_id,
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
            .get_user_posts(
                &user_id,
                cursor.pagination_token.as_deref(),
                cursor.since_id.as_deref(),
            )
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

        let newest_id = newest_post_id(
            cursor.newest_id.as_deref().or(cursor.since_id.as_deref()),
            posts.value.data.iter().map(|post| post.id.as_str()),
        )?;
        let pagination_token = posts.value.meta.next_token.clone();
        let next_state = if pagination_token.is_some() {
            XSyncCursor {
                version: X_CURSOR_VERSION,
                since_id: cursor.since_id,
                pagination_token,
                newest_id,
            }
        } else {
            let high_water = newest_id.or(cursor.since_id);
            XSyncCursor {
                version: X_CURSOR_VERSION,
                since_id: high_water.clone(),
                pagination_token: None,
                newest_id: high_water,
            }
        };
        let next_cursor = Some(encode_cursor(&next_state)?);

        let raw = vec![
            RawEvidence {
                media_type: "application/json".into(),
                bytes: me.bytes,
                suggested_path: Some("authenticated-user.json".into()),
            },
            RawEvidence {
                media_type: "application/json".into(),
                bytes: posts.bytes,
                suggested_path: Some("posts.json".into()),
            },
        ];

        Ok(SyncBatch {
            records,
            raw,
            next_cursor,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct XSyncCursor {
    version: u32,
    #[serde(default)]
    since_id: Option<String>,
    #[serde(default)]
    pagination_token: Option<String>,
    #[serde(default)]
    newest_id: Option<String>,
}

impl Default for XSyncCursor {
    fn default() -> Self {
        Self {
            version: X_CURSOR_VERSION,
            since_id: None,
            pagination_token: None,
            newest_id: None,
        }
    }
}

fn parse_cursor(value: Option<&str>) -> Result<XSyncCursor, String> {
    let Some(value) = value else {
        return Ok(XSyncCursor::default());
    };
    let cursor: XSyncCursor = serde_json::from_str(value)
        .map_err(|error| format!("invalid X sync cursor JSON: {error}"))?;
    if cursor.version != X_CURSOR_VERSION {
        return Err(format!(
            "unsupported X sync cursor version {}; expected {}",
            cursor.version, X_CURSOR_VERSION
        ));
    }
    for (name, id) in [
        ("since_id", cursor.since_id.as_deref()),
        ("newest_id", cursor.newest_id.as_deref()),
    ] {
        if let Some(id) = id {
            if !is_snowflake(id) {
                return Err(format!("invalid X cursor {name}: {id}"));
            }
        }
    }
    if cursor
        .pagination_token
        .as_deref()
        .is_some_and(|token| token.trim().is_empty())
    {
        return Err("X cursor pagination_token cannot be blank".to_owned());
    }
    Ok(cursor)
}

fn encode_cursor(cursor: &XSyncCursor) -> Result<String, AdapterError> {
    serde_json::to_string(cursor)
        .map_err(|error| AdapterError::Data(format!("failed to encode X sync cursor: {error}")))
}

fn newest_post_id<'a>(
    current: Option<&str>,
    ids: impl Iterator<Item = &'a str>,
) -> Result<Option<String>, AdapterError> {
    let mut newest = current.map(str::to_owned);
    for id in ids {
        if !is_snowflake(id) {
            return Err(AdapterError::Data(format!(
                "invalid X post id in response: {id}"
            )));
        }
        let replace = newest
            .as_deref()
            .is_none_or(|existing| compare_numeric_strings(id, existing) == Ordering::Greater);
        if replace {
            newest = Some(id.to_owned());
        }
    }
    Ok(newest)
}

fn compare_numeric_strings(left: &str, right: &str) -> Ordering {
    left.len().cmp(&right.len()).then_with(|| left.cmp(right))
}

fn is_snowflake(value: &str) -> bool {
    !value.is_empty() && value.len() <= 19 && value.bytes().all(|byte| byte.is_ascii_digit())
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

    #[test]
    fn x_cursor_distinguishes_pagination_from_terminal_high_water() {
        let adapter = XAdapter::new();
        let paging = XSyncCursor {
            version: X_CURSOR_VERSION,
            since_id: Some("100".to_owned()),
            pagination_token: Some("NEXT".to_owned()),
            newest_id: Some("150".to_owned()),
        };
        let terminal = XSyncCursor {
            version: X_CURSOR_VERSION,
            since_id: Some("150".to_owned()),
            pagination_token: None,
            newest_id: Some("150".to_owned()),
        };

        assert!(
            adapter
                .cursor_has_more(Some(&encode_cursor(&paging).unwrap()))
                .unwrap()
        );
        assert!(
            !adapter
                .cursor_has_more(Some(&encode_cursor(&terminal).unwrap()))
                .unwrap()
        );
    }

    #[test]
    fn newest_post_id_uses_numeric_order_not_lexical_order() {
        assert_eq!(
            newest_post_id(Some("99"), ["100", "7"].into_iter()).unwrap(),
            Some("100".to_owned())
        );
    }
}
