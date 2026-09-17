use std::collections::BTreeMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct XUserEnvelope {
    pub data: XUser,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct XUser {
    pub id: String,
    pub username: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub profile_image_url: Option<String>,
    #[serde(default)]
    pub public_metrics: BTreeMap<String, i64>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct XPostsEnvelope {
    #[serde(default)]
    pub data: Vec<XPost>,
    #[serde(default)]
    pub meta: XPostsMeta,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub(crate) struct XPostsMeta {
    #[serde(default)]
    pub next_token: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct XPost {
    pub id: String,
    pub text: String,
    #[serde(default)]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(default, alias = "referenced_tweets")]
    pub referenced_posts: Vec<XPostReference>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct XPostReference {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
}
