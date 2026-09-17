use serde::de::DeserializeOwned;
use thiserror::Error;
use url::Url;

use crate::models::{XPostsEnvelope, XUserEnvelope};
use crate::remote_error::safe_remote_category;

const API_BASE: &str = "https://api.x.com/2/";
const USER_POST_FIELDS: &str = "created_at,referenced_tweets,note_tweet";

#[derive(Clone)]
pub struct XApiClient {
    http: reqwest::Client,
    access_token: String,
}

impl XApiClient {
    pub fn new(access_token: impl Into<String>) -> Result<Self, XApiError> {
        let access_token = access_token.into();
        if access_token.trim().is_empty() {
            return Err(XApiError::EmptyAccessToken);
        }

        Ok(Self {
            http: reqwest::Client::new(),
            access_token,
        })
    }

    pub(crate) async fn get_me(&self) -> Result<RawResponse<XUserEnvelope>, XApiError> {
        let mut url = Url::parse(&format!("{API_BASE}users/me"))?;
        url.query_pairs_mut().append_pair(
            "user.fields",
            "id,name,username,description,profile_image_url,public_metrics",
        );
        self.get_json(url).await
    }

    pub(crate) async fn get_user_posts(
        &self,
        user_id: &str,
        pagination_token: Option<&str>,
        since_id: Option<&str>,
    ) -> Result<RawResponse<XPostsEnvelope>, XApiError> {
        let url = build_user_posts_url(user_id, pagination_token, since_id)?;
        self.get_json(url).await
    }

    async fn get_json<T>(&self, url: Url) -> Result<RawResponse<T>, XApiError>
    where
        T: DeserializeOwned,
    {
        let response = self
            .http
            .get(url)
            .bearer_auth(&self.access_token)
            .send()
            .await?;
        let status = response.status();
        let bytes = response.bytes().await?.to_vec();

        if !status.is_success() {
            let status = status.as_u16();
            return Err(XApiError::Remote {
                status,
                category: safe_remote_category(status, &bytes),
            });
        }

        let value = serde_json::from_slice(&bytes)?;
        Ok(RawResponse { value, bytes })
    }
}

fn build_user_posts_url(
    user_id: &str,
    pagination_token: Option<&str>,
    since_id: Option<&str>,
) -> Result<Url, XApiError> {
    validate_user_id(user_id)?;
    if let Some(since_id) = since_id {
        validate_post_id(since_id)?;
    }

    let mut url = Url::parse(&format!("{API_BASE}users/{user_id}/tweets"))?;
    {
        let mut query = url.query_pairs_mut();
        query.append_pair("max_results", "100");
        query.append_pair("tweet.fields", USER_POST_FIELDS);
        query.append_pair("exclude", "retweets");
        if let Some(pagination_token) = pagination_token {
            query.append_pair("pagination_token", pagination_token);
        }
        if let Some(since_id) = since_id {
            query.append_pair("since_id", since_id);
        }
    }
    Ok(url)
}

pub(crate) struct RawResponse<T> {
    pub value: T,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Error)]
pub enum XApiError {
    #[error("X access token cannot be empty")]
    EmptyAccessToken,
    #[error("invalid X user id: {0}")]
    InvalidUserId(String),
    #[error("invalid X post id: {0}")]
    InvalidPostId(String),
    #[error("X URL error: {0}")]
    Url(#[from] url::ParseError),
    #[error("X transport error: {0}")]
    Transport(#[from] reqwest::Error),
    #[error("X response JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("X API returned HTTP {status} ({category})")]
    Remote {
        status: u16,
        category: &'static str,
    },
}

fn validate_user_id(value: &str) -> Result<(), XApiError> {
    if !is_snowflake(value) {
        return Err(XApiError::InvalidUserId(value.to_owned()));
    }
    Ok(())
}

fn validate_post_id(value: &str) -> Result<(), XApiError> {
    if !is_snowflake(value) {
        return Err(XApiError::InvalidPostId(value.to_owned()));
    }
    Ok(())
}

fn is_snowflake(value: &str) -> bool {
    !value.is_empty() && value.len() <= 19 && value.bytes().all(|byte| byte.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    #[test]
    fn rejects_non_numeric_x_user_ids() {
        assert!(matches!(
            validate_user_id("not-a-number"),
            Err(XApiError::InvalidUserId(_))
        ));
    }

    #[test]
    fn accepts_x_snowflake_shaped_ids() {
        validate_user_id("2244994945").unwrap();
        validate_post_id("1844674407370955161").unwrap();
    }

    #[test]
    fn remote_api_error_display_is_safe_by_construction() {
        let body = br#"{
            "title":"Too Many Requests",
            "detail":"private-post-text access-secret refresh-secret"
        }"#;
        let error = XApiError::Remote {
            status: 429,
            category: safe_remote_category(429, body),
        };
        let rendered = error.to_string();

        assert_eq!(rendered, "X API returned HTTP 429 (rate_limit)");
        assert!(!rendered.contains("private-post-text"));
        assert!(!rendered.contains("access-secret"));
        assert!(!rendered.contains("refresh-secret"));
    }

    #[test]
    fn user_posts_url_uses_current_x_wire_fields_and_excludes_retweets() {
        let url = build_user_posts_url(
            "2244994945",
            Some("NEXT TOKEN"),
            Some("1844674407370955161"),
        )
        .unwrap();
        let query = url.query_pairs().into_owned().collect::<BTreeMap<_, _>>();

        assert_eq!(query.get("max_results").map(String::as_str), Some("100"));
        assert_eq!(
            query.get("tweet.fields").map(String::as_str),
            Some(USER_POST_FIELDS)
        );
        assert!(!query.contains_key("post.fields"));
        assert_eq!(query.get("exclude").map(String::as_str), Some("retweets"));
        assert_eq!(
            query.get("pagination_token").map(String::as_str),
            Some("NEXT TOKEN")
        );
        assert_eq!(
            query.get("since_id").map(String::as_str),
            Some("1844674407370955161")
        );
    }
}
