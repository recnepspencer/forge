use super::*;

#[cfg(test)]
pub(in crate::runtime) fn runtime_state_snapshot_test_subject_identity(
    label: &str,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::LowerRuntimeBoundaryEvidence)
        .field_shape(WorthQueryEvidenceTag::new("test_subject"), label)
        .seal()
}
