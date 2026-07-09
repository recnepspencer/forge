use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceScope, WorthQueryEvidenceTag,
};
use crate::query_context::QueryContextFamily;

#[allow(dead_code)]
pub(super) fn scoped_query_context_compatibility_label(
    family: QueryContextFamily,
    declared_basis_label: &str,
) -> String {
    worth_query_evidence_identity(WorthQueryEvidenceScope::QueryContextCompatibilityBasisLabel)
        .field_shape(WorthQueryEvidenceTag::new("family"), family.as_str())
        .field_value(
            WorthQueryEvidenceTag::new("declared_basis_label"),
            declared_basis_label,
        )
        .seal()
        .terminal_projection_for_reporting()
        .to_string()
}
