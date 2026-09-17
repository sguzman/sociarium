use std::fs;
use std::path::{Path, PathBuf};

use sociarium_core::NormalizedRecord;
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct CorpusLayout {
    root: PathBuf,
}

impl CorpusLayout {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    pub fn raw_dir(&self) -> PathBuf {
        self.root.join("raw")
    }

    pub fn objects_dir(&self) -> PathBuf {
        self.root.join("objects")
    }

    pub fn observations_dir(&self) -> PathBuf {
        self.root.join("observations")
    }

    pub fn provenance_dir(&self) -> PathBuf {
        self.root.join("provenance")
    }

    pub fn ensure_dirs(&self) -> Result<(), StoreError> {
        for path in [
            self.raw_dir(),
            self.objects_dir(),
            self.observations_dir(),
            self.provenance_dir(),
        ] {
            fs::create_dir_all(path)?;
        }
        Ok(())
    }

    pub fn write_normalized_json(
        &self,
        relative_path: impl AsRef<Path>,
        record: &NormalizedRecord,
    ) -> Result<PathBuf, StoreError> {
        let path = self.objects_dir().join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_vec_pretty(record)?;
        fs::write(&path, json)?;
        Ok(path)
    }
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("filesystem error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Json(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layer_directories_are_distinct() {
        let layout = CorpusLayout::new("corpus");
        assert_ne!(layout.raw_dir(), layout.objects_dir());
        assert_ne!(layout.objects_dir(), layout.provenance_dir());
    }
}
