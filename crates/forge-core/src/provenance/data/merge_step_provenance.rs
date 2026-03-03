//! Merge step provenance payload and selector origin.

use serde::{Deserialize, Serialize};

use super::snapshot_handle_ref::SnapshotHandleRef;

/// Origin of a merge-step selector decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectorOrigin {
    AutoDerived,
    UserSelector,
    PolicyResolved,
}

/// Serializable provenance payload for an executed merge step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeStepProvenance {
    pub step_index: u32,
    pub edge_snapshot: SnapshotHandleRef,
    pub survive_face_snapshot: SnapshotHandleRef,
    pub kill_face_snapshot: SnapshotHandleRef,
    pub selector_origin: SelectorOrigin,
}
