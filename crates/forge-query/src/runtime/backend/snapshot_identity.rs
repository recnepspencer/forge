use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQuerySnapshotIdentity;

pub(in crate::runtime) fn unavailable_snapshot_identity() -> ForgeQuerySnapshotIdentity {
    ForgeQuerySnapshotIdentity::preview(
        ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::RuntimeStateSnapshot)
            .field_shape(
                ForgeQueryEvidenceTag::new("snapshot_authority"),
                "unavailable",
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("snapshot_contract"),
                "backend-must-override-for-authoritative-truth",
            )
            .seal(),
    )
}
