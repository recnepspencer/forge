use serde::{Deserialize, Serialize};

use crate::publication::patch::data::CanonicalAspectSet;

use super::RecordRef;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct PatchVsTruthDeltaReport {
    pub records_checked: u64,
    pub exact_match: bool,
    pub mismatched_targets: Vec<RecordRef>,
    pub structural_mismatches: u64,
    pub aspect_mismatches: u64,
    pub opaque_aspect_mismatches: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct AspectTagAccuracyReport {
    pub records_checked: u64,
    pub correctly_tagged_records: u64,
    pub touched_aspects: CanonicalAspectSet,
    pub opaque_aspect_record_count: u64,
}
