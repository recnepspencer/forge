use serde::{Deserialize, Serialize};

use crate::publication::patch::data::CanonicalAspectSet;

use super::RecordRef;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatchVsTruthDeltaReport {
    pub records_checked: usize,
    pub exact_match: bool,
    pub mismatched_targets: Vec<RecordRef>,
    pub structural_mismatches: usize,
    pub aspect_mismatches: usize,
    pub degraded_precision_mismatches: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectTagAccuracyReport {
    pub records_checked: usize,
    pub correctly_tagged_records: usize,
    pub touched_aspects: CanonicalAspectSet,
    pub degraded_precision_record_count: usize,
}
