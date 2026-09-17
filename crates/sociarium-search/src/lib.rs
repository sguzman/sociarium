use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use chrono::{DateTime, Utc};
use rusqlite::{Connection, OpenFlags, params};
use sociarium_core::{NormalizedRecord, Post};
use sociarium_store::{AcquisitionManifest, CorpusLayout, STORE_SCHEMA_VERSION};
use thiserror::Error;

pub const SEARCH_INDEX_SCHEMA_VERSION: i64 = 1;
const MAX_QUERY_RESULTS: usize = 1_000;
static REBUILD_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Debug)]
pub struct SearchIndex {
    corpus_root: PathBuf,
    path: PathBuf,
}

impl SearchIndex {
    pub fn for_corpus(corpus_root: impl Into<PathBuf>) -> Self {
        let corpus_root = corpus_root.into();
        let path = CorpusLayout::new(&corpus_root)
            .indexes_dir()
            .join("search.sqlite");
        Self { corpus_root, path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn rebuild(&self) -> Result<RebuildStats, SearchError> {
        let scan = scan_corpus(&self.corpus_root)?;
        let parent = self
            .path
            .parent()
            .ok_or_else(|| SearchError::InvalidIndexPath(self.path.clone()))?;
        fs::create_dir_all(parent)?;

        let pending = parent.join(format!(
            ".search.sqlite.pending-{}-{}",
            std::process::id(),
            REBUILD_COUNTER.fetch_add(1, Ordering::Relaxed)
        ));
        if pending.exists() {
            fs::remove_file(&pending)?;
        }
        if let Err(error) = build_index(&pending, &scan.posts) {
            let _ = fs::remove_file(&pending);
            return Err(error);
        }
        if self.path.exists() {
            fs::remove_file(&self.path)?;
        }
        fs::rename(&pending, &self.path)?;

        Ok(RebuildStats {
            acquisitions_scanned: scan.acquisitions_scanned,
            records_scanned: scan.records_scanned,
            posts_indexed: scan.posts.len(),
        })
    }

    pub fn search_posts(
        &self,
        query: &str,
        profile_id: Option<&str>,
        limit: usize,
    ) -> Result<Vec<PostHit>, SearchError> {
        let connection = self.open_read_only()?;
        let mut statement = connection.prepare(
            "SELECT p.object_id, p.profile_id, p.surface, p.remote_id, p.created_at, \
                    p.observed_at, p.text, p.canonical_url, bm25(posts_fts) \
             FROM posts_fts \
             JOIN posts p ON p.object_id = posts_fts.object_id \
             WHERE posts_fts MATCH ?1 AND (?2 IS NULL OR p.profile_id = ?2) \
             ORDER BY bm25(posts_fts), COALESCE(p.created_at, p.observed_at) DESC \
             LIMIT ?3",
        )?;
        let rows = statement.query_map(
            params![literal_fts_query(query)?, profile_id, bounded_limit(limit)],
            row_to_hit,
        )?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(SearchError::from)
    }

    pub fn list_posts(
        &self,
        profile_id: Option<&str>,
        limit: usize,
    ) -> Result<Vec<PostHit>, SearchError> {
        let connection = self.open_read_only()?;
        let mut statement = connection.prepare(
            "SELECT object_id, profile_id, surface, remote_id, created_at, observed_at, \
                    text, canonical_url, NULL \
             FROM posts \
             WHERE (?1 IS NULL OR profile_id = ?1) \
             ORDER BY COALESCE(created_at, observed_at) DESC, object_id DESC \
             LIMIT ?2",
        )?;
        let rows = statement.query_map(params![profile_id, bounded_limit(limit)], row_to_hit)?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(SearchError::from)
    }

    fn open_read_only(&self) -> Result<Connection, SearchError> {
        if !self.path.is_file() {
            return Err(SearchError::MissingIndex(self.path.clone()));
        }
        let connection = Connection::open_with_flags(&self.path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
        let version: i64 = connection.query_row("PRAGMA user_version", [], |row| row.get(0))?;
        if version != SEARCH_INDEX_SCHEMA_VERSION {
            return Err(SearchError::UnsupportedIndexSchema {
                found: version,
                expected: SEARCH_INDEX_SCHEMA_VERSION,
            });
        }
        Ok(connection)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RebuildStats {
    pub acquisitions_scanned: usize,
    pub records_scanned: usize,
    pub posts_indexed: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PostHit {
    pub object_id: String,
    pub profile_id: String,
    pub surface: String,
    pub remote_id: String,
    pub created_at: Option<String>,
    pub observed_at: String,
    pub text: String,
    pub canonical_url: Option<String>,
    pub rank: Option<f64>,
}

fn row_to_hit(row: &rusqlite::Row<'_>) -> rusqlite::Result<PostHit> {
    Ok(PostHit {
        object_id: row.get(0)?,
        profile_id: row.get(1)?,
        surface: row.get(2)?,
        remote_id: row.get(3)?,
        created_at: row.get(4)?,
        observed_at: row.get(5)?,
        text: row.get(6)?,
        canonical_url: row.get(7)?,
        rank: row.get(8)?,
    })
}

fn bounded_limit(limit: usize) -> i64 {
    limit.min(MAX_QUERY_RESULTS) as i64
}

fn literal_fts_query(query: &str) -> Result<String, SearchError> {
    let terms = query
        .split_whitespace()
        .filter(|term| !term.is_empty())
        .map(|term| format!("\"{}\"", term.replace('"', "\"\"")))
        .collect::<Vec<_>>();
    if terms.is_empty() {
        return Err(SearchError::EmptyQuery);
    }
    Ok(terms.join(" "))
}

fn build_index(path: &Path, posts: &[Post]) -> Result<(), SearchError> {
    let mut connection = Connection::open(path)?;
    connection.execute_batch(
        "PRAGMA journal_mode = OFF;
         PRAGMA synchronous = FULL;
         CREATE TABLE posts (
             object_id TEXT PRIMARY KEY,
             profile_id TEXT NOT NULL,
             surface TEXT NOT NULL,
             remote_id TEXT NOT NULL,
             created_at TEXT,
             observed_at TEXT NOT NULL,
             text TEXT NOT NULL,
             canonical_url TEXT
         );
         CREATE INDEX posts_profile_created ON posts(profile_id, created_at DESC);
         CREATE VIRTUAL TABLE posts_fts USING fts5(
             object_id UNINDEXED,
             profile_id UNINDEXED,
             surface UNINDEXED,
             text,
             tokenize = 'unicode61'
         );",
    )?;

    let transaction = connection.transaction()?;
    {
        let mut insert_post = transaction.prepare(
            "INSERT INTO posts (
                 object_id, profile_id, surface, remote_id, created_at, observed_at, text, canonical_url
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        )?;
        let mut insert_fts = transaction.prepare(
            "INSERT INTO posts_fts (object_id, profile_id, surface, text) VALUES (?1, ?2, ?3, ?4)",
        )?;
        for post in posts {
            insert_post.execute(params![
                post.id.as_str(),
                post.profile_id.as_str(),
                post.observation.surface.as_str(),
                post.remote_id.as_str(),
                post.created_at.as_ref().map(DateTime::to_rfc3339),
                post.observation.observed_at.to_rfc3339(),
                post.text.as_str(),
                post.canonical_url.as_deref(),
            ])?;
            insert_fts.execute(params![
                post.id.as_str(),
                post.profile_id.as_str(),
                post.observation.surface.as_str(),
                post.text.as_str(),
            ])?;
        }
    }
    transaction.commit()?;
    connection.execute_batch("PRAGMA user_version = 1; PRAGMA optimize;")?;
    Ok(())
}

#[derive(Debug)]
struct CorpusScan {
    posts: Vec<Post>,
    acquisitions_scanned: usize,
    records_scanned: usize,
}

fn scan_corpus(corpus_root: &Path) -> Result<CorpusScan, SearchError> {
    let acquisitions_root = CorpusLayout::new(corpus_root).acquisitions_dir();
    let mut latest_posts: BTreeMap<String, (DateTime<Utc>, String, Post)> = BTreeMap::new();
    let mut acquisitions_scanned = 0;
    let mut records_scanned = 0;

    for surface_dir in child_directories(&acquisitions_root)? {
        for profile_dir in child_directories(&surface_dir)? {
            for acquisition_dir in child_directories(&profile_dir)? {
                let name = acquisition_dir
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default();
                if name.starts_with(".pending-") {
                    continue;
                }
                let manifest_path = acquisition_dir.join("manifest.json");
                if !manifest_path.is_file() {
                    return Err(SearchError::MissingManifest(acquisition_dir));
                }
                let manifest: AcquisitionManifest =
                    serde_json::from_slice(&fs::read(&manifest_path)?)?;
                if manifest.schema_version != STORE_SCHEMA_VERSION {
                    return Err(SearchError::UnsupportedStoreSchema {
                        path: manifest_path,
                        found: manifest.schema_version,
                        expected: STORE_SCHEMA_VERSION,
                    });
                }

                acquisitions_scanned += 1;
                let records_path = acquisition_dir.join("normalized").join("records.jsonl");
                let reader = BufReader::new(File::open(&records_path)?);
                let mut acquisition_records = 0;
                for line in reader.lines() {
                    let line = line?;
                    if line.trim().is_empty() {
                        continue;
                    }
                    let record: NormalizedRecord = serde_json::from_str(&line)?;
                    acquisition_records += 1;
                    records_scanned += 1;
                    validate_record(&record, &manifest, &records_path)?;

                    if let NormalizedRecord::Post(post) = record {
                        let key = post.id.to_string();
                        let observed_at = post.observation.observed_at;
                        let acquisition_id = post.observation.acquisition_id.clone();
                        let replace = latest_posts.get(&key).is_none_or(|current| {
                            (observed_at, acquisition_id.as_str()) > (current.0, current.1.as_str())
                        });
                        if replace {
                            latest_posts.insert(key, (observed_at, acquisition_id, post));
                        }
                    }
                }
                if acquisition_records != manifest.record_count {
                    return Err(SearchError::RecordCountMismatch {
                        path: records_path,
                        expected: manifest.record_count,
                        actual: acquisition_records,
                    });
                }
            }
        }
    }

    Ok(CorpusScan {
        posts: latest_posts
            .into_values()
            .map(|(_, _, post)| post)
            .collect(),
        acquisitions_scanned,
        records_scanned,
    })
}

fn validate_record(
    record: &NormalizedRecord,
    manifest: &AcquisitionManifest,
    path: &Path,
) -> Result<(), SearchError> {
    if record.profile_id() != &manifest.profile_id
        || record.observation().surface != manifest.surface
        || record.observation().acquisition_id != manifest.acquisition_id
        || record.observation().observed_at != manifest.observed_at
    {
        return Err(SearchError::RecordManifestMismatch(path.to_path_buf()));
    }
    Ok(())
}

fn child_directories(path: &Path) -> Result<Vec<PathBuf>, SearchError> {
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut directories = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            directories.push(entry.path());
        }
    }
    directories.sort();
    Ok(directories)
}

#[derive(Debug, Error)]
pub enum SearchError {
    #[error("filesystem error: {0}")]
    Io(#[from] std::io::Error),
    #[error("corpus JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("SQLite search index error: {0}")]
    Sql(#[from] rusqlite::Error),
    #[error("search query cannot be empty")]
    EmptyQuery,
    #[error("invalid search index path: {0:?}")]
    InvalidIndexPath(PathBuf),
    #[error("search index does not exist: {0:?}; run `sociarium index rebuild`")]
    MissingIndex(PathBuf),
    #[error("unsupported search index schema {found}; expected {expected}; rebuild the index")]
    UnsupportedIndexSchema { found: i64, expected: i64 },
    #[error("acquisition directory is missing manifest.json: {0:?}")]
    MissingManifest(PathBuf),
    #[error("unsupported corpus store schema {found} in {path:?}; expected {expected}")]
    UnsupportedStoreSchema {
        path: PathBuf,
        found: u32,
        expected: u32,
    },
    #[error("normalized record does not match acquisition manifest: {0:?}")]
    RecordManifestMismatch(PathBuf),
    #[error("normalized record count mismatch in {path:?}: expected {expected}, got {actual}")]
    RecordCountMismatch {
        path: PathBuf,
        expected: usize,
        actual: usize,
    },
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use chrono::{TimeZone, Utc};
    use sociarium_core::{
        NormalizedRecord, ObjectId, ObservationMeta, Post, ProfileId, RemoteId, SurfaceId,
    };
    use sociarium_store::RawFileManifest;

    use super::*;

    struct TempCorpus(PathBuf);

    impl TempCorpus {
        fn new(name: &str) -> Self {
            let root = std::env::temp_dir().join(format!(
                "sociarium-search-{name}-{}-{}",
                std::process::id(),
                REBUILD_COUNTER.fetch_add(1, Ordering::Relaxed)
            ));
            let _ = fs::remove_dir_all(&root);
            Self(root)
        }
    }

    impl Drop for TempCorpus {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn write_post_acquisition(root: &Path, acquisition_id: &str, hour: u32, text: &str) {
        let observed_at = Utc.with_ymd_and_hms(2026, 9, 17, hour, 0, 0).unwrap();
        let observation = ObservationMeta {
            surface: SurfaceId::new("x").unwrap(),
            observed_at,
            acquisition_id: acquisition_id.to_owned(),
            schema_version: 1,
        };
        let post = Post {
            id: ObjectId::new("x:post:100").unwrap(),
            profile_id: ProfileId::new("x-main").unwrap(),
            remote_id: RemoteId::new("100").unwrap(),
            created_at: Some(Utc.with_ymd_and_hms(2026, 9, 17, 12, 0, 0).unwrap()),
            text: text.to_owned(),
            reply_to: None,
            quote_of: None,
            canonical_url: Some("https://x.com/sguzman/status/100".to_owned()),
            observation,
            extensions: BTreeMap::new(),
        };
        let record = NormalizedRecord::Post(post);
        let acquisition_dir = root
            .join("acquisitions")
            .join("x")
            .join("x-main")
            .join(acquisition_id);
        fs::create_dir_all(acquisition_dir.join("normalized")).unwrap();
        fs::write(
            acquisition_dir.join("normalized").join("records.jsonl"),
            format!("{}\n", serde_json::to_string(&record).unwrap()),
        )
        .unwrap();
        let manifest = AcquisitionManifest {
            schema_version: STORE_SCHEMA_VERSION,
            acquisition_id: acquisition_id.to_owned(),
            profile_id: ProfileId::new("x-main").unwrap(),
            surface: SurfaceId::new("x").unwrap(),
            observed_at,
            record_count: 1,
            raw_files: Vec::<RawFileManifest>::new(),
            next_cursor: None,
        };
        fs::write(
            acquisition_dir.join("manifest.json"),
            serde_json::to_vec_pretty(&manifest).unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn rebuild_uses_latest_observation_and_is_recoverable_after_deletion() {
        let corpus = TempCorpus::new("rebuild");
        write_post_acquisition(&corpus.0, "acq-old", 13, "older wording");
        write_post_acquisition(&corpus.0, "acq-new", 14, "newer wording about sociarium");

        let index = SearchIndex::for_corpus(&corpus.0);
        let stats = index.rebuild().unwrap();
        assert_eq!(stats.acquisitions_scanned, 2);
        assert_eq!(stats.records_scanned, 2);
        assert_eq!(stats.posts_indexed, 1);
        assert_eq!(index.search_posts("sociarium", None, 20).unwrap().len(), 1);
        assert!(index.search_posts("older", None, 20).unwrap().is_empty());

        fs::remove_file(index.path()).unwrap();
        assert!(matches!(
            index.search_posts("sociarium", None, 20),
            Err(SearchError::MissingIndex(_))
        ));
        index.rebuild().unwrap();
        let posts = index.list_posts(Some("x-main"), 20).unwrap();
        assert_eq!(posts.len(), 1);
        assert_eq!(posts[0].text, "newer wording about sociarium");
    }

    #[test]
    fn literal_queries_treat_punctuation_as_text() {
        assert_eq!(
            literal_fts_query("fallen/village").unwrap(),
            "\"fallen/village\""
        );
        assert!(matches!(
            literal_fts_query("   "),
            Err(SearchError::EmptyQuery)
        ));
    }
}
