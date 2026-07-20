use crate::evidence_identity::{
    WorthQueryEvidenceIdentity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::memory_workspace::WorthQuerySnapshotIdentity;

pub(in crate::runtime) fn unavailable_snapshot_identity() -> WorthQuerySnapshotIdentity {
    WorthQuerySnapshotIdentity::preview(
        WorthQueryEvidenceIdentity::compose(WorthQueryEvidenceScope::RuntimeStateSnapshot)
            .field_shape(
                WorthQueryEvidenceTag::new("snapshot_authority"),
                "unavailable",
            )
            .field_shape(
                WorthQueryEvidenceTag::new("snapshot_contract"),
                "backend-must-override-for-authoritative-truth",
            )
            .seal(),
    )
}
