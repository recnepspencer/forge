use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQueryCommitIdentity;
use crate::runtime::ForgeQueryAspectTouch;

pub(super) fn effect_trigger_commit_evidence_identity(
    commit_identity: &ForgeQueryCommitIdentity,
) -> ForgeQueryEvidenceIdentity {
    let commit_evidence_identity = commit_identity.evidence_identity();
    forge_query_evidence_identity(ForgeQueryEvidenceScope::EffectTriggerCommitIdentity)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("trigger_commit_identity"),
            &commit_evidence_identity,
        )
        .seal()
}

pub(super) fn terminal_touch_digest_projection_sequence(
    touches: &[ForgeQueryAspectTouch],
) -> String {
    touches
        .iter()
        .map(ForgeQueryAspectTouch::admitted_touch_digest_part)
        .collect::<Vec<_>>()
        .join(",")
}
