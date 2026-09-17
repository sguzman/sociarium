use std::collections::BTreeSet;
use std::fmt;

use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use rand::{Rng, distributions::Alphanumeric};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use thiserror::Error;
use url::Url;

const AUTHORIZE_URL: &str = "https://x.com/i/oauth2/authorize";
const TOKEN_URL: &str = "https://api.x.com/2/oauth2/token";

#[derive(Clone, Debug)]
pub struct XOAuthConfig {
    client_id: String,
    redirect_uri: Url,
    scopes: BTreeSet<String>,
}

impl XOAuthConfig {
    pub fn new(
        client_id: impl Into<String>,
        redirect_uri: impl AsRef<str>,
    ) -> Result<Self, XOAuthError> {
        let client_id = client_id.into();
        if client_id.trim().is_empty() {
            return Err(XOAuthError::InvalidConfig("client_id cannot be blank"));
        }

        let redirect_uri = Url::parse(redirect_uri.as_ref())?;
        let scopes = ["offline.access", "tweet.read", "users.read"]
            .into_iter()
            .map(str::to_owned)
            .collect();

        Ok(Self {
            client_id,
            redirect_uri,
            scopes,
        })
    }

    pub fn with_scope(mut self, scope: impl Into<String>) -> Result<Self, XOAuthError> {
        let scope = scope.into();
        if scope.trim().is_empty() {
            return Err(XOAuthError::InvalidConfig("scope cannot be blank"));
        }
        self.scopes.insert(scope);
        Ok(self)
    }

    pub fn begin(&self) -> Result<XOAuthSession, XOAuthError> {
        XOAuthSession::from_parts(self, random_ascii(32), random_ascii(64))
    }

    pub async fn exchange_code(
        &self,
        session: &XOAuthSession,
        code: &str,
    ) -> Result<XTokenSet, XOAuthError> {
        if code.trim().is_empty() {
            return Err(XOAuthError::InvalidCallback("authorization code is blank"));
        }

        let params = [
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", self.redirect_uri.as_str()),
            ("code_verifier", session.code_verifier.as_str()),
            ("client_id", self.client_id.as_str()),
        ];
        self.request_token(&params).await
    }

    pub async fn refresh(&self, refresh_token: &str) -> Result<XTokenSet, XOAuthError> {
        if refresh_token.trim().is_empty() {
            return Err(XOAuthError::InvalidCallback("refresh token is blank"));
        }

        let params = [
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id", self.client_id.as_str()),
        ];
        self.request_token(&params).await
    }

    async fn request_token(&self, params: &[(&str, &str)]) -> Result<XTokenSet, XOAuthError> {
        let response = reqwest::Client::new()
            .post(TOKEN_URL)
            .form(params)
            .send()
            .await?;
        let status = response.status();
        let bytes = response.bytes().await?;

        if !status.is_success() {
            let body = String::from_utf8_lossy(&bytes)
                .chars()
                .take(512)
                .collect::<String>();
            return Err(XOAuthError::Remote {
                status: status.as_u16(),
                body,
            });
        }

        let response: TokenResponse = serde_json::from_slice(&bytes)?;
        Ok(XTokenSet {
            access_token: response.access_token,
            refresh_token: response.refresh_token,
            token_type: response.token_type,
            expires_in_seconds: response.expires_in,
            scope: response.scope,
        })
    }
}

#[derive(Clone)]
pub struct XOAuthSession {
    state: String,
    code_verifier: String,
    authorize_url: Url,
}

impl XOAuthSession {
    fn from_parts(
        config: &XOAuthConfig,
        state: String,
        code_verifier: String,
    ) -> Result<Self, XOAuthError> {
        let digest = Sha256::digest(code_verifier.as_bytes());
        let challenge = URL_SAFE_NO_PAD.encode(digest);
        let scope = config.scopes.iter().cloned().collect::<Vec<_>>().join(" ");

        let mut authorize_url = Url::parse(AUTHORIZE_URL)?;
        authorize_url
            .query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("client_id", &config.client_id)
            .append_pair("redirect_uri", config.redirect_uri.as_str())
            .append_pair("scope", &scope)
            .append_pair("state", &state)
            .append_pair("code_challenge", &challenge)
            .append_pair("code_challenge_method", "S256");

        Ok(Self {
            state,
            code_verifier,
            authorize_url,
        })
    }

    pub fn state(&self) -> &str {
        &self.state
    }

    pub fn authorize_url(&self) -> &Url {
        &self.authorize_url
    }

    pub fn state_matches(&self, received_state: &str) -> bool {
        self.state == received_state
    }
}

impl fmt::Debug for XOAuthSession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("XOAuthSession")
            .field("state", &"[redacted]")
            .field("code_verifier", &"[redacted]")
            .field("authorize_url", &self.authorize_url)
            .finish()
    }
}

#[derive(Clone)]
pub struct XTokenSet {
    access_token: String,
    refresh_token: Option<String>,
    token_type: String,
    expires_in_seconds: u64,
    scope: Option<String>,
}

impl XTokenSet {
    pub fn access_token(&self) -> &str {
        &self.access_token
    }

    pub fn refresh_token(&self) -> Option<&str> {
        self.refresh_token.as_deref()
    }

    pub fn expires_in_seconds(&self) -> u64 {
        self.expires_in_seconds
    }

    pub fn scope(&self) -> Option<&str> {
        self.scope.as_deref()
    }
}

impl fmt::Debug for XTokenSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("XTokenSet")
            .field("access_token", &"[redacted]")
            .field(
                "refresh_token",
                &self.refresh_token.as_ref().map(|_| "[redacted]"),
            )
            .field("token_type", &self.token_type)
            .field("expires_in_seconds", &self.expires_in_seconds)
            .field("scope", &self.scope)
            .finish()
    }
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    access_token: String,
    #[serde(default)]
    refresh_token: Option<String>,
    #[serde(default = "default_token_type")]
    token_type: String,
    #[serde(default)]
    expires_in: u64,
    #[serde(default)]
    scope: Option<String>,
}

fn default_token_type() -> String {
    "bearer".to_owned()
}

fn random_ascii(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

#[derive(Debug, Error)]
pub enum XOAuthError {
    #[error("invalid X OAuth configuration: {0}")]
    InvalidConfig(&'static str),
    #[error("invalid X OAuth callback: {0}")]
    InvalidCallback(&'static str),
    #[error("X OAuth URL error: {0}")]
    Url(#[from] url::ParseError),
    #[error("X OAuth transport error: {0}")]
    Transport(#[from] reqwest::Error),
    #[error("X OAuth response JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("X OAuth endpoint returned HTTP {status}: {body}")]
    Remote { status: u16, body: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authorization_url_uses_pkce_and_minimal_read_scopes() {
        let config =
            XOAuthConfig::new("client", "http://127.0.0.1:49152/oauth/x/callback").unwrap();
        let session = XOAuthSession::from_parts(
            &config,
            "known-state".to_owned(),
            "known-verifier-that-is-long-enough-for-a-pkce-test-value".to_owned(),
        )
        .unwrap();

        let params = session
            .authorize_url()
            .query_pairs()
            .into_owned()
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(
            params.get("response_type").map(String::as_str),
            Some("code")
        );
        assert_eq!(
            params.get("code_challenge_method").map(String::as_str),
            Some("S256")
        );
        assert!(params["scope"].contains("tweet.read"));
        assert!(params["scope"].contains("users.read"));
        assert!(params["scope"].contains("offline.access"));
        assert!(session.state_matches("known-state"));
    }

    #[test]
    fn debug_output_redacts_transient_secrets() {
        let config =
            XOAuthConfig::new("client", "http://127.0.0.1:49152/oauth/x/callback").unwrap();
        let session = config.begin().unwrap();
        let rendered = format!("{session:?}");
        assert!(rendered.contains("[redacted]"));
        assert!(!rendered.contains(&session.code_verifier));
    }
}
