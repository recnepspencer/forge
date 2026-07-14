use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

/// Typed evidence identity for worth-ui query-binding correlation digests.
pub fn worth_ui_query_binding_evidence_identity(
    surface: &'static str,
    basis_values: &[String],
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::ApplicationSupportSectionPosture)
        .field_shape(
            WorthQueryEvidenceTag::new("section"),
            "worth-ui-query-binding-evidence",
        )
        .field_shape(WorthQueryEvidenceTag::new("surface"), surface)
        .field_value_sequence(WorthQueryEvidenceTag::new("basis"), basis_values)
        .seal()
}
