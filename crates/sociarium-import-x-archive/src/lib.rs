use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha256};
use sociarium_acquisition::{AcquisitionBatch, RawEvidence};
use sociarium_core::{
    NormalizedRecord, ObjectId, ObservationMeta, Post, ProfileOwnership, ProfileSnapshot, RemoteId,
    SurfaceId, TrackedProfile,
};
use thiserror::Error;

const SURFACE_X: &str = "x";

#[derive(Debug, Error)]
pub enum XArchiveError {
    #[error("filesystem error: {0}")]
    Io(#[from] std::io::Error),
    #[error("archive root is not a directory: {}", .0.display())]
    NotDirectory(PathBuf),
    #[error("X archive account.js was not found under {}", .0.display())]
    MissingAccount(PathBuf),
    #[error("X archive tweet data was not found under {}", .0.display())]
    MissingTweets(PathBuf),
    #[error("archive file {} did not contain a JSON array payload", .0.display())]
    MissingJsonArray(PathBuf),
    #[error("archive JSON error in {}: {source}", path.display())]
    Json {
        path: PathBuf,
        #[source]
        source: serde_json::Error,
    },
    #[error("invalid X archive account data: {0}")]
    Account(String),
    #[error("invalid X archive tweet data: {0}")]
    Tweet(String),
    #[error("configured profile {0} is not an enabled self-owned X profile")]
    InvalidProfile(String),
    #[error(
        "archive remote profile id mismatch for {profile_id}: expected {expected}, got {actual}"
    )]
    RemoteIdMismatch {
        profile_id: String,
        expected: String,
        actual: String,
    },
    #[error(
        "unbound profile {profile_id} expected enrollment handle @{expected}, archive belongs to @{actual}"
    )]
    HandleMismatch {
        profile_id: String,
        expected: String,
        actual: String,
    },
    #[error("unbound profile {0} needs a configured handle or remote_id before archive enrollment")]
    MissingEnrollmentIntent(String),
}

#[derive(Debug)]
pub struct XArchiveImport {
    pub batch: AcquisitionBatch,
    pub account_remote_id: RemoteId,
    pub account_handle: String,
    pub post_count: usize,
    pub skipped_retweets: usize,
}

pub fn import_directory(
    root: &Path,
    profile: &TrackedProfile,
    observed_at: DateTime<Utc>,
) -> Result<XArchiveImport, XArchiveError> {
    validate_profile(profile)?;
    if !root.is_dir() {
        return Err(XArchiveError::NotDirectory(root.to_path_buf()));
    }

    let data_root = if root.join("data").is_dir() {
        root.join("data")
    } else {
        root.to_path_buf()
    };

    let account_path = find_account_file(&data_root)
        .ok_or_else(|| XArchiveError::MissingAccount(data_root.clone()))?;
    let tweet_paths = find_tweet_files(&data_root)?;
    if tweet_paths.is_empty() {
        return Err(XArchiveError::MissingTweets(data_root));
    }

    let account_bytes = fs::read(&account_path)?;
    let account_items: Vec<AccountEnvelope> = parse_wrapped_array(&account_path, &account_bytes)?;
    let account = account_items
        .into_iter()
        .next()
        .ok_or_else(|| XArchiveError::Account("account.js contained no account record".to_owned()))?
        .account;

    let account_remote_id = RemoteId::new(account.account_id.clone())
        .map_err(|error| XArchiveError::Account(error.to_string()))?;
    validate_enrollment(profile, &account_remote_id, &account.username)?;

    let mut source_files = vec![(account_path.clone(), account_bytes)];
    for path in &tweet_paths {
        source_files.push((path.clone(), fs::read(path)?));
    }
    source_files.sort_by(|left, right| left.0.cmp(&right.0));

    let acquisition_id = acquisition_id(&source_files);
    let observation = ObservationMeta {
        surface: SurfaceId::new(SURFACE_X).expect("static X surface id is valid"),
        observed_at,
        acquisition_id,
        schema_version: 1,
    };

    let mut records = vec![NormalizedRecord::ProfileSnapshot(ProfileSnapshot {
        profile_id: profile.id.clone(),
        remote_id: account_remote_id.clone(),
        handle: Some(account.username.clone()),
        display_name: nonblank(account.display_name),
        bio: None,
        avatar_url: None,
        metrics: BTreeMap::new(),
        observation: observation.clone(),
        extensions: BTreeMap::from([(
            "sociarium.acquisition_source".to_owned(),
            json!("x_account_archive"),
        )]),
    })];

    let mut post_count = 0;
    let mut skipped_retweets = 0;

    for (path, bytes) in source_files
        .iter()
        .filter(|(path, _)| path != &account_path)
    {
        let items: Vec<TweetEnvelope> = parse_wrapped_array(path, bytes)?;
        for item in items {
            let tweet = item.tweet;
            if is_retweet(&tweet) {
                skipped_retweets += 1;
                continue;
            }
            records.push(NormalizedRecord::Post(normalize_tweet(
                profile,
                &account.username,
                tweet,
                observation.clone(),
                path,
            )?));
            post_count += 1;
        }
    }

    let raw = source_files
        .into_iter()
        .map(|(path, bytes)| RawEvidence {
            media_type: "application/javascript".to_owned(),
            bytes,
            suggested_path: Some(format!(
                "x-archive/{}",
                path.file_name()
                    .map(|name| name.to_string_lossy())
                    .unwrap_or_default()
            )),
        })
        .collect();

    Ok(XArchiveImport {
        batch: AcquisitionBatch {
            records,
            raw,
            next_cursor: None,
        },
        account_remote_id,
        account_handle: account.username,
        post_count,
        skipped_retweets,
    })
}

fn validate_profile(profile: &TrackedProfile) -> Result<(), XArchiveError> {
    if profile.surface.as_str() != SURFACE_X
        || profile.ownership != ProfileOwnership::SelfOwned
        || !profile.enabled
    {
        return Err(XArchiveError::InvalidProfile(profile.id.to_string()));
    }
    Ok(())
}

fn validate_enrollment(
    profile: &TrackedProfile,
    archive_remote_id: &RemoteId,
    archive_handle: &str,
) -> Result<(), XArchiveError> {
    if let Some(expected) = &profile.remote_id {
        if expected != archive_remote_id {
            return Err(XArchiveError::RemoteIdMismatch {
                profile_id: profile.id.to_string(),
                expected: expected.to_string(),
                actual: archive_remote_id.to_string(),
            });
        }
        return Ok(());
    }

    if let Some(expected) = profile.handle.as_deref() {
        if expected.eq_ignore_ascii_case(archive_handle) {
            return Ok(());
        }
        return Err(XArchiveError::HandleMismatch {
            profile_id: profile.id.to_string(),
            expected: expected.to_owned(),
            actual: archive_handle.to_owned(),
        });
    }

    Err(XArchiveError::MissingEnrollmentIntent(
        profile.id.to_string(),
    ))
}

fn find_account_file(root: &Path) -> Option<PathBuf> {
    fs::read_dir(root)
        .ok()?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| {
            path.is_file()
                && path
                    .file_name()
                    .is_some_and(|name| name.to_string_lossy().eq_ignore_ascii_case("account.js"))
        })
}

fn find_tweet_files(root: &Path) -> Result<Vec<PathBuf>, XArchiveError> {
    let mut files = fs::read_dir(root)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            if !path.is_file() {
                return false;
            }
            let Some(name) = path.file_name() else {
                return false;
            };
            let lower = name.to_string_lossy().to_ascii_lowercase();
            lower == "tweets.js"
                || lower == "tweet.js"
                || (lower.starts_with("tweets-part") && lower.ends_with(".js"))
        })
        .collect::<Vec<_>>();
    files.sort();
    Ok(files)
}

fn parse_wrapped_array<T: for<'de> Deserialize<'de>>(
    path: &Path,
    bytes: &[u8],
) -> Result<Vec<T>, XArchiveError> {
    let text = std::str::from_utf8(bytes)
        .map_err(|_| XArchiveError::MissingJsonArray(path.to_path_buf()))?
        .trim_start_matches('\u{feff}')
        .trim_start();

    let array_start = text
        .find('[')
        .ok_or_else(|| XArchiveError::MissingJsonArray(path.to_path_buf()))?;
    serde_json::from_str(&text[array_start..]).map_err(|source| XArchiveError::Json {
        path: path.to_path_buf(),
        source,
    })
}

fn normalize_tweet(
    profile: &TrackedProfile,
    username: &str,
    tweet: ArchiveTweet,
    observation: ObservationMeta,
    source_path: &Path,
) -> Result<Post, XArchiveError> {
    let remote_id_value = tweet
        .id_str
        .or(tweet.id)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| XArchiveError::Tweet("tweet has no stable id".to_owned()))?;
    let remote_id = RemoteId::new(remote_id_value.clone())
        .map_err(|error| XArchiveError::Tweet(error.to_string()))?;
    let text = tweet
        .full_text
        .or(tweet.text)
        .ok_or_else(|| XArchiveError::Tweet(format!("tweet {remote_id_value} has no text")))?;
    let created_at = tweet
        .created_at
        .as_deref()
        .map(parse_twitter_timestamp)
        .transpose()?;

    Ok(Post {
        id: x_post_object_id(&remote_id_value)?,
        profile_id: profile.id.clone(),
        remote_id,
        created_at,
        text,
        reply_to: relationship_id(
            tweet
                .in_reply_to_status_id_str
                .or(tweet.in_reply_to_status_id),
        )?,
        quote_of: relationship_id(tweet.quoted_status_id_str.or(tweet.quoted_status_id))?,
        canonical_url: Some(format!("https://x.com/{username}/status/{remote_id_value}")),
        observation,
        extensions: BTreeMap::from([
            (
                "sociarium.acquisition_source".to_owned(),
                json!("x_account_archive"),
            ),
            (
                "sociarium.archive_source_file".to_owned(),
                json!(
                    source_path
                        .file_name()
                        .map(|name| name.to_string_lossy())
                        .unwrap_or_default()
                ),
            ),
        ]),
    })
}

fn parse_twitter_timestamp(value: &str) -> Result<DateTime<Utc>, XArchiveError> {
    DateTime::parse_from_str(value, "%a %b %d %H:%M:%S %z %Y")
        .map(|value| value.with_timezone(&Utc))
        .map_err(|error| XArchiveError::Tweet(format!("invalid created_at {value:?}: {error}")))
}

fn relationship_id(value: Option<String>) -> Result<Option<ObjectId>, XArchiveError> {
    value
        .filter(|value| !value.trim().is_empty())
        .map(|value| x_post_object_id(&value))
        .transpose()
}

fn x_post_object_id(remote_id: &str) -> Result<ObjectId, XArchiveError> {
    ObjectId::new(format!("x:post:{remote_id}"))
        .map_err(|error| XArchiveError::Tweet(error.to_string()))
}

fn is_retweet(tweet: &ArchiveTweet) -> bool {
    tweet.retweeted.unwrap_or(false)
        || tweet
            .full_text
            .as_deref()
            .or(tweet.text.as_deref())
            .is_some_and(|text| text.starts_with("RT @"))
}

fn nonblank(value: Option<String>) -> Option<String> {
    value.filter(|value| !value.trim().is_empty())
}

fn acquisition_id(files: &[(PathBuf, Vec<u8>)]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"sociarium:x-account-archive:v1\0");
    for (path, bytes) in files {
        hasher.update(
            path.file_name()
                .map(|name| name.to_string_lossy())
                .unwrap_or_default()
                .as_bytes(),
        );
        hasher.update([0]);
        hasher.update((bytes.len() as u64).to_le_bytes());
        hasher.update(bytes);
    }
    format!("x-archive-{:x}", hasher.finalize())
}

#[derive(Debug, Deserialize)]
struct AccountEnvelope {
    account: ArchiveAccount,
}

#[derive(Debug, Deserialize)]
struct ArchiveAccount {
    username: String,
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "accountDisplayName")]
    display_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TweetEnvelope {
    tweet: ArchiveTweet,
}

#[derive(Debug, Deserialize)]
struct ArchiveTweet {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    id_str: Option<String>,
    #[serde(default)]
    full_text: Option<String>,
    #[serde(default)]
    text: Option<String>,
    #[serde(default)]
    created_at: Option<String>,
    #[serde(default)]
    retweeted: Option<bool>,
    #[serde(default)]
    in_reply_to_status_id: Option<String>,
    #[serde(default)]
    in_reply_to_status_id_str: Option<String>,
    #[serde(default)]
    quoted_status_id: Option<String>,
    #[serde(default)]
    quoted_status_id_str: Option<String>,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    use chrono::{TimeZone, Utc};
    use sociarium_core::{ProfileId, ProfileOwnership};

    use super::*;

    static COUNTER: AtomicU64 = AtomicU64::new(0);

    struct TempArchive(PathBuf);

    impl TempArchive {
        fn new(name: &str) -> Self {
            let path = std::env::temp_dir().join(format!(
                "sociarium-x-archive-{name}-{}-{}",
                std::process::id(),
                COUNTER.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir_all(path.join("data")).unwrap();
            Self(path)
        }

        fn data(&self) -> PathBuf {
            self.0.join("data")
        }
    }

    impl Drop for TempArchive {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn profile(handle: Option<&str>, remote_id: Option<&str>) -> TrackedProfile {
        TrackedProfile {
            id: ProfileId::new("x-main").unwrap(),
            surface: SurfaceId::new("x").unwrap(),
            remote_id: remote_id.map(|value| RemoteId::new(value).unwrap()),
            handle: handle.map(str::to_owned),
            ownership: ProfileOwnership::SelfOwned,
            enabled: true,
        }
    }

    fn write_fixture(root: &TempArchive) {
        fs::write(
            root.data().join("account.js"),
            r#"window.YTD.account.part0 = [
  {
    "account": {
      "username": "sguzman",
      "accountId": "6679733",
      "accountDisplayName": "Salvador"
    }
  }
]"#,
        )
        .unwrap();

        fs::write(
            root.data().join("tweets.js"),
            r#"window.YTD.tweets.part0 = [
  {
    "tweet": {
      "id_str": "100",
      "created_at": "Thu Sep 17 12:00:00 +0000 2026",
      "full_text": "first archived post",
      "retweeted": false
    }
  },
  {
    "tweet": {
      "id_str": "200",
      "created_at": "Thu Sep 17 13:00:00 +0000 2026",
      "full_text": "reply and quote",
      "in_reply_to_status_id_str": "150",
      "quoted_status_id_str": "175",
      "retweeted": false
    }
  },
  {
    "tweet": {
      "id_str": "300",
      "created_at": "Thu Sep 17 14:00:00 +0000 2026",
      "full_text": "RT @someone this is a repost",
      "retweeted": false
    }
  }
]"#,
        )
        .unwrap();
    }

    #[test]
    fn imports_wrapped_account_and_authored_posts() {
        let archive = TempArchive::new("basic");
        write_fixture(&archive);
        let imported = import_directory(
            &archive.0,
            &profile(Some("SGUZMAN"), None),
            Utc.with_ymd_and_hms(2026, 9, 18, 1, 0, 0).unwrap(),
        )
        .unwrap();

        assert_eq!(imported.account_remote_id.as_str(), "6679733");
        assert_eq!(imported.account_handle, "sguzman");
        assert_eq!(imported.post_count, 2);
        assert_eq!(imported.skipped_retweets, 1);
        assert_eq!(imported.batch.records.len(), 3);
        assert_eq!(imported.batch.raw.len(), 2);
        assert!(imported.batch.next_cursor.is_none());

        let NormalizedRecord::Post(post) = &imported.batch.records[2] else {
            panic!("expected second authored post");
        };
        assert_eq!(post.remote_id.as_str(), "200");
        assert_eq!(
            post.reply_to.as_ref().map(ObjectId::as_str),
            Some("x:post:150")
        );
        assert_eq!(
            post.quote_of.as_ref().map(ObjectId::as_str),
            Some("x:post:175")
        );
    }

    #[test]
    fn same_archive_has_stable_acquisition_id() {
        let archive = TempArchive::new("stable-id");
        write_fixture(&archive);
        let profile = profile(Some("sguzman"), None);

        let first = import_directory(
            &archive.0,
            &profile,
            Utc.with_ymd_and_hms(2026, 9, 18, 1, 0, 0).unwrap(),
        )
        .unwrap();
        let second = import_directory(
            &archive.0,
            &profile,
            Utc.with_ymd_and_hms(2026, 9, 18, 2, 0, 0).unwrap(),
        )
        .unwrap();

        assert_eq!(
            first.batch.records[0].observation().acquisition_id,
            second.batch.records[0].observation().acquisition_id
        );
    }

    #[test]
    fn bound_profile_rejects_different_archive_account() {
        let archive = TempArchive::new("remote-mismatch");
        write_fixture(&archive);
        let error = import_directory(
            &archive.0,
            &profile(Some("sguzman"), Some("999999")),
            Utc.with_ymd_and_hms(2026, 9, 18, 1, 0, 0).unwrap(),
        )
        .unwrap_err();

        assert!(matches!(error, XArchiveError::RemoteIdMismatch { .. }));
    }

    #[test]
    fn first_enrollment_rejects_different_handle() {
        let archive = TempArchive::new("handle-mismatch");
        write_fixture(&archive);
        let error = import_directory(
            &archive.0,
            &profile(Some("someone-else"), None),
            Utc.with_ymd_and_hms(2026, 9, 18, 1, 0, 0).unwrap(),
        )
        .unwrap_err();

        assert!(matches!(error, XArchiveError::HandleMismatch { .. }));
    }

    #[test]
    fn malformed_wrapped_json_is_rejected() {
        let archive = TempArchive::new("malformed");
        write_fixture(&archive);
        fs::write(
            archive.data().join("tweets.js"),
            "window.YTD.tweets.part0 = [ definitely not json",
        )
        .unwrap();

        let error = import_directory(
            &archive.0,
            &profile(Some("sguzman"), None),
            Utc.with_ymd_and_hms(2026, 9, 18, 1, 0, 0).unwrap(),
        )
        .unwrap_err();

        assert!(matches!(error, XArchiveError::Json { .. }));
    }
}
