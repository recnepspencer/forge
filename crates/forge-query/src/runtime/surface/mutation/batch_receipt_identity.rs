use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::ForgeQueryAspectTouch;

pub(super) fn evidence_value_identities(
    role: &'static str,
    values: &[String],
) -> Vec<ForgeQueryEvidenceIdentity> {
    values
        .iter()
        .map(|value| {
            forge_query_evidence_identity(ForgeQueryEvidenceScope::BatchWriteReceipt)
                .field_shape(ForgeQueryEvidenceTag::new("role"), role)
                .field_value(ForgeQueryEvidenceTag::new("value"), value)
                .seal()
        })
        .collect()
}

pub(super) fn terminal_touch_projection_identities(
    role: &'static str,
    touches: &[ForgeQueryAspectTouch],
) -> Vec<ForgeQueryEvidenceIdentity> {
    touches
        .iter()
        .map(|touch| {
            forge_query_evidence_identity(ForgeQueryEvidenceScope::BatchWriteReceipt)
                .field_shape(ForgeQueryEvidenceTag::new("role"), role)
                .field_value(
                    ForgeQueryEvidenceTag::new("value"),
                    touch.admitted_touch_digest_part(),
                )
                .seal()
        })
        .collect()
}
