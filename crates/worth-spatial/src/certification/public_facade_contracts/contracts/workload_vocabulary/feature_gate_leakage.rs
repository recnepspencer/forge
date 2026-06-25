const FACADE_MOD_RS: &str = include_str!("../../../../facade/mod.rs");
const WORKLOAD_VOCABULARY_FACADE_RS: &str =
    include_str!("../../../../facade/workload_vocabulary/mod.rs");
const QUERY_ADOPTION_FACADE_RS: &str = include_str!("../../../../facade/query_adoption.rs");
const SPATIAL_CARGO_TOML: &str = include_str!("../../../../../Cargo.toml");

#[test]
fn public_facades_do_not_leak_certification_or_test_support_constructors() {
    for (label, source) in [
        ("facade/mod.rs", FACADE_MOD_RS),
        (
            "facade/workload_vocabulary/mod.rs",
            WORKLOAD_VOCABULARY_FACADE_RS,
        ),
        ("facade/query_adoption.rs", QUERY_ADOPTION_FACADE_RS),
    ] {
        assert!(
            !source.contains("#[cfg(test)]"),
            "{label} must not expose test-only facade exports"
        );
        assert!(
            !source.contains("#[doc(hidden)]"),
            "{label} must not hide production-usable constructor exports"
        );
        for forbidden in [
            "SpatialGeometryEvidenceTouchAuthorityParts",
            "BooleanEvidenceReceiptSealed",
            "certification_only_admitted_stage_row",
            "complete_ledger_with_additional_rows",
            "evaluate_current_spatial_support_pins",
            "from_parts",
        ] {
            assert!(
                !source.contains(forbidden),
                "{label} leaks certification/test constructor surface {forbidden}"
            );
        }
    }
}

#[test]
fn test_support_feature_is_not_a_public_constructor_gate() {
    assert!(
        SPATIAL_CARGO_TOML.contains("[features]"),
        "worth-spatial feature table must remain auditable"
    );
    assert!(
        SPATIAL_CARGO_TOML.contains("test-support-lowering = []"),
        "existing test support feature must stay explicit and dependency-free"
    );
    assert!(
        !WORKLOAD_VOCABULARY_FACADE_RS.contains("test-support-lowering"),
        "feature gate must not appear in ordinary workload_vocabulary facade"
    );
    assert!(
        !QUERY_ADOPTION_FACADE_RS.contains("test-support-lowering"),
        "feature gate must not appear in ordinary query_adoption facade"
    );
}
