use serde::de::DeserializeOwned;
use thiserror::Error;
use url::Url;

use crate::models::{XPostsEnvelope, XUserEnvelope};

const API_BASE: &str = "https://api.x.com/2/";

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
        validate_user_id(user_id)?;
        if let Some(since_id) = since_id {
            validate_post_id(since_id)?;
        }

        let mut url = Url::parse(&format!("{API_BASE}users/{user_id}/tweets"))?;
        {
            let mut query = url.query_pairs_mut();
            query.append_pair("max_results", "100");
            query.append_pair("post.fields", "created_at");
            if let Some(pagination_token) = pagination_token {
                query.append_pair("pagination_token", pagination_token);
            }
            if let Some(since_id) = since_id {
                query.append_pair("since_id", since_id);
            }
        }
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
            let body = String::from_utf8_lossy(&bytes)
                .chars()
                .take(512)
                .collect::<String>();
            return Err(XApiError::Remote {
                status: status.as_u16(),
                body,
            });
        }

        let value = serde_json::from_slice(&bytes)?;
        Ok(RawResponse { value, bytes })
    }
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
    #[error("X API returned HTTP {status}: {body}")]
    Remote { status: u16, body: String },
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
}
