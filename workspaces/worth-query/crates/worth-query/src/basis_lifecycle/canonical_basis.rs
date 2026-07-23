use worth_foundational::facade::{
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact, CanonicalBasisValue,
    CanonicalizationRuleVersion,
};

use super::proofs::NormalizedBasisIntent;

pub(super) fn prepare_admitted_basis(
    normalized: &NormalizedBasisIntent,
) -> CanonicalBasisReadyArtifact {
    let version = CanonicalizationRuleVersion::new("worth-query-admitted-basis-v1")
        .expect("the fixed Query basis canonicalization version is valid");
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::BoundaryArtifact,
        canonical_entries(normalized),
    )
    .into_result()
    .expect("a normalized Query basis has a complete canonical representation")
}

fn canonical_entries(normalized: &NormalizedBasisIntent) -> Vec<CanonicalBasisEntry> {
    vec![
        text_entry("family", normalized.family().as_str()),
        text_entry("authority", normalized.authority().as_str()),
        text_entry("scope", normalized.scope().as_str()),
        text_entry("visibility", normalized.visibility().as_str()),
        text_entry("lifecycle", normalized.lifecycle().as_str()),
        text_entry("operation-lane", normalized.operation_lane()),
        optional_text_entry("policy", normalized.policy_digest()),
        optional_text_entry("tenant-schema", normalized.tenant_schema_digest()),
        optional_text_entry(
            "eligibility-denial",
            normalized
                .eligibility_denial_cause()
                .map(|cause| cause.as_str()),
        ),
        optional_text_entry(
            "lower-runtime-binding",
            normalized.lower_runtime_binding_digest(),
        ),
    ]
}

fn text_entry(locus: &str, value: &str) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::BoundaryArtifact,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::BoundaryArtifact,
        CanonicalBasisValue::ExactText(value.to_string().into()),
    )
}

fn optional_text_entry(locus: &str, value: Option<&str>) -> CanonicalBasisEntry {
    value.map_or_else(
        || {
            CanonicalBasisEntry::new(
                CanonicalBasisDomain::BoundaryArtifact,
                CanonicalBasisLocus::Named(locus.to_string().into()),
                CanonicalBasisEntryKind::BoundaryArtifact,
                CanonicalBasisValue::Null,
            )
        },
        |value| text_entry(locus, value),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absent_optional_basis_is_typed_null_not_reserved_text() {
        assert_eq!(
            optional_text_entry("policy", None).value(),
            &CanonicalBasisValue::Null
        );
        assert_eq!(
            optional_text_entry("policy", Some("<absent>")).value(),
            &CanonicalBasisValue::ExactText("<absent>".into())
        );
    }
}
