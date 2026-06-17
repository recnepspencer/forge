use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

/// Typed evidence identity for worth-ui query-binding correlation digests.
pub fn worth_ui_query_binding_evidence_identity(
    surface: &'static str,
    basis_values: &[String],
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::ApplicationSupportSectionPosture)
        .field_shape(
            ForgeQueryEvidenceTag::new("section"),
            "worth-ui-query-binding-evidence",
        )
        .field_shape(ForgeQueryEvidenceTag::new("surface"), surface)
        .field_value_sequence(ForgeQueryEvidenceTag::new("basis"), basis_values)
        .seal()
}
