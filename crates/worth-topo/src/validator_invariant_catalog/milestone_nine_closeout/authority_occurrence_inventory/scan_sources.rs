pub(in crate::validator_invariant_catalog) const FORBIDDEN_AUTHORITY_PATTERNS: &[&str] = &[
    "milestone_one_invariant_registrations",
    "DERIVED_TOPOLOGY_RULE_SPECS",
    "milestone_three_validator_expectations",
    "CertificationValidatorExpectation",
    "validator_expectations",
    "validator_family_count",
    "validator_name_count",
    "derived_validation_row_count",
    "query_invariant_validator",
    "spatial_validator",
    "required_phase_1_validator_rows",
    "required_phase_2_validator_rows",
    "operator_local_invariant_hook",
    "local_validator_array",
    "selection_only_validator",
    "hardcoded_validator_support",
];

pub(in crate::validator_invariant_catalog::milestone_nine_closeout::authority_occurrence_inventory) fn current_source_pairs(
) -> [(&'static str, &'static str); 8] {
    [
        (
            "certification/topology_operator_closeout/acceptance_rows/validator_family_coverage.rs",
            include_str!("../../../certification/topology_operator_closeout/acceptance_rows/validator_family_coverage.rs"),
        ),
        (
            "certification/topology_operator_closeout/validation_breadth_row.rs",
            include_str!("../../../certification/topology_operator_closeout/validation_breadth_row.rs"),
        ),
        ("runtime_support.rs", include_str!("../../../runtime_support.rs")),
        (
            "topology_operators/application/declaration_entry/execution_finalize.rs",
            include_str!("../../../topology_operators/application/declaration_entry/execution_finalize.rs"),
        ),
        (
            "topology_operators/declaration_entry/mod.rs",
            include_str!("../../../topology_operators/declaration_entry/mod.rs"),
        ),
        (
            "topology_operators/edge_split_blueprint/required_phase_1_validator_lanes.rs",
            include_str!("../../../topology_operators/edge_split_blueprint/required_phase_1_validator_lanes.rs"),
        ),
        (
            "topology_operators/loop_reconstruction_blueprint/phase_2_inventory/validator_rows.rs",
            include_str!("../../../topology_operators/loop_reconstruction_blueprint/phase_2_inventory/validator_rows.rs"),
        ),
        (
            "validation/rule_registry.rs",
            include_str!("../../../validation/rule_registry.rs"),
        ),
    ]
}
