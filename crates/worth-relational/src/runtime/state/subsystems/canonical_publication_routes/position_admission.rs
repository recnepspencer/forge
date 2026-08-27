use std::sync::Arc;

use crate::history::data::{CanonicalCommitEnvelope, PositionedCanonicalCommit};
use crate::publication::patch::data::PatchStreamPosition;

/// Owner-issued admission for binding one canonical envelope to one runtime
/// patch-stream position. Its fields and constructors stay inside the
/// canonical publication owner; history data can only consume the seal.
pub(crate) struct CanonicalPositionAdmission {
    position: PatchStreamPosition,
    canonical: Arc<CanonicalCommitEnvelope>,
}

impl CanonicalPositionAdmission {
    pub(super) fn performed(
        position: PatchStreamPosition,
        canonical: Arc<CanonicalCommitEnvelope>,
    ) -> Self {
        Self {
            position,
            canonical,
        }
    }

    pub(crate) fn into_parts(self) -> (PatchStreamPosition, Arc<CanonicalCommitEnvelope>) {
        (self.position, self.canonical)
    }
}

pub(crate) fn readmit_positioned_canonical_commit(
    position: PatchStreamPosition,
    canonical: Arc<CanonicalCommitEnvelope>,
) -> Result<PositionedCanonicalCommit, String> {
    if position.0 == 0 {
        return Err("canonical patch-stream position zero is not admissible".into());
    }
    if canonical.branch_context != canonical.commit.branch_id {
        return Err("canonical envelope branch context does not match commit provenance".into());
    }
    crate::history::RelationalCommitCatalog::default()
        .validate_envelope(&canonical)
        .map_err(|denial| format!("canonical envelope admission failed: {denial:?}"))?;
    Ok(PositionedCanonicalCommit::admit(
        CanonicalPositionAdmission {
            position,
            canonical,
        },
    ))
}
