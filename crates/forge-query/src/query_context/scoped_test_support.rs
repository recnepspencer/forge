use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::query_context::QueryContextFamily;

pub(super) fn scoped_query_context_compatibility_label(
    family: QueryContextFamily,
    declared_basis_label: &str,
) -> String {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::QueryContextCompatibilityBasisLabel)
        .field_shape(ForgeQueryEvidenceTag::new("family"), family.as_str())
        .field_value(
            ForgeQueryEvidenceTag::new("declared_basis_label"),
            declared_basis_label,
        )
        .seal()
        .terminal_projection_for_reporting()
        .to_string()
}
