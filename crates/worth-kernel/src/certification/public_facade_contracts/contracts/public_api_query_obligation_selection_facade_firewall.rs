#[test]
fn public_facade_does_not_export_query_selection_migration_internals() {
    let workload_composition_exports = include_str!("../../../workload_composition/mod.rs");
    let kernel_root_exports = include_str!("../../../lib.rs");
    let public_facade_sources = [
        include_str!("../../../query_obligation_selection/public_facade/mod.rs"),
        include_str!("../../../query_obligation_selection/public_facade/kinds.rs"),
        include_str!(
            "../../../query_obligation_selection/public_facade/milestone_five_closeout.rs"
        ),
        include_str!("../../../query_obligation_selection/public_facade/request.rs"),
        include_str!("../../../query_obligation_selection/public_facade/request_conversion.rs"),
        include_str!("../../../query_obligation_selection/public_facade/selected_status.rs"),
        include_str!("../../../query_obligation_selection/public_facade/selected_closeout.rs"),
        include_str!("../../../query_obligation_selection/public_facade/selected_precision.rs"),
    ]
    .join("\n");

    for forbidden in [
        "selection_substrate::",
        "QueryObligationSelectionSubstrate",
        "QueryObligationSelectionInput",
        "QueryObligationSelectionAuthorityKind",
        "QueryObligationSelectionErrorKind",
        "QuerySelectedGraphObligations",
        "QuerySelectedGraphObligationCloseout",
        "local_ceremony_closeout",
        "primitive_construction_contract",
        "selector_matrix",
        "from_query_proof",
    ] {
        assert!(
            !contains_export_token(workload_composition_exports, forbidden)
                && !contains_public_signature_token(&public_facade_sources, forbidden),
            "Worth-facing workload exports must not expose migration/internal selector surface `{forbidden}`"
        );
    }

    for forbidden in [
        "ForgeQueryAdmittedGraphReadAccessPlan",
        "ForgeQueryGraphReadAccessAdmission",
        "ForgeQueryGraphReadAccessPlanConsumption",
        "ForgeQueryReadReceipt",
    ] {
        assert!(
            !workload_composition_exports.contains(forbidden)
                && !kernel_root_exports.contains(forbidden),
            "Phase 7 selected-obligation DX must not claim graph-read access planning surface `{forbidden}`"
        );
    }
}

fn contains_public_signature_token(source: &str, forbidden: &str) -> bool {
    source
        .lines()
        .filter(|line| line.trim_start().starts_with("pub "))
        .any(|line| contains_export_token(line, forbidden))
}

fn contains_export_token(exports: &str, forbidden: &str) -> bool {
    if forbidden.ends_with("::") {
        return exports.contains(forbidden);
    }

    exports
        .split(|character: char| !(character == '_' || character.is_ascii_alphanumeric()))
        .any(|token| token == forbidden)
}
