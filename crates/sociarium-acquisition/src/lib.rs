use serde::{Deserialize, Serialize};
use sociarium_core::NormalizedRecord;

/// Source-neutral raw evidence acquired from a social surface.
///
/// This may be a successful HTTP response body, an original account-archive file,
/// or another explicitly preserved source artifact. Authority-bearing credentials
/// are never raw social evidence.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RawEvidence {
    pub media_type: String,
    pub bytes: Vec<u8>,
    pub suggested_path: Option<String>,
}

/// One atomic unit of acquired evidence plus its normalized interpretation.
///
/// The optional continuation field is source-owned state. Live adapters may use
/// it for pagination or incremental checkpoints. Offline importers normally leave
/// it empty.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AcquisitionBatch {
    pub records: Vec<NormalizedRecord>,
    pub raw: Vec<RawEvidence>,
    pub next_cursor: Option<String>,
}
