use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::transactions::data::RecordRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum MergePlanningDecisionKind {
    Admitted,
    Blocked,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergePlanningDecisionRecord {
    pub record: RecordRef,
    pub target_record: Option<RecordRef>,
    pub decision: MergePlanningDecisionKind,
    pub classification: crate::merge::data::MergeConflictClass,
    pub causal_disposition: crate::merge::data::MergeRecordCausalDisposition,
    pub policy_resolution: crate::merge::data::MergePolicyResolution,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergePlanningDecisionLog {
    pub decisions: Arc<[MergePlanningDecisionRecord]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergePlanningDecisionLogDigestBasis {
    pub canonical_decisions: Arc<[MergePlanningDecisionKind]>,
    pub canonical_records: Arc<[RecordRef]>,
    pub canonical_target_records: Arc<[Option<RecordRef>]>,
    pub canonical_classifications: Arc<[crate::merge::data::MergeConflictClass]>,
    pub canonical_causal_dispositions: Arc<[crate::merge::data::MergeRecordCausalDisposition]>,
    pub canonical_policy_resolutions: Arc<[crate::merge::data::MergePolicyResolution]>,
}
