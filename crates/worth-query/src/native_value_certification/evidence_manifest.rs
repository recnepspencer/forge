#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryMilestoneNineThirteenNativeValueEvidenceRow {
    phase: u8,
    path: &'static str,
    probe: &'static str,
}

impl WorthQueryMilestoneNineThirteenNativeValueEvidenceRow {
    const fn new(phase: u8, path: &'static str, probe: &'static str) -> Self {
        Self { phase, path, probe }
    }

    pub const fn phase(&self) -> u8 {
        self.phase
    }

    pub const fn path(&self) -> &'static str {
        self.path
    }

    pub const fn probe(&self) -> &'static str {
        self.probe
    }
}

const EVIDENCE: &[WorthQueryMilestoneNineThirteenNativeValueEvidenceRow] = &[
    row(
        21,
        "crates/worth-query/src/consumer_kit/native_value_authority_inventory/tests.rs",
        "fn phase_twenty_one_native_value_grammar_is_complete_and_single_authority()",
    ),
    row(
        21,
        "crates/worth-query/src/consumer_kit/native_value_authority_inventory/tests.rs",
        "fn phase_twenty_one_current_query_source_inventory_is_closed()",
    ),
    row(
        21,
        "crates/worth-query/src/consumer_kit/native_value_authority_inventory/tests.rs",
        "fn source_inventory_rejects_seeded_coarse_scalar_authority()",
    ),
    row(
        21,
        "crates/worth-query/src/consumer_kit/native_value_authority_inventory/tests.rs",
        "fn source_inventory_rejects_seeded_scalar_only_struct_bridge()",
    ),
    row(
        21,
        "crates/worth-query/src/consumer_kit/native_value_authority_inventory/tests.rs",
        "fn source_inventory_rejects_seeded_misleading_row_and_bypass_wrapper()",
    ),
    row(
        22,
        "crates/worth-foundational/tests/certification/aspects/portable/patch_boundary.rs",
        "fn serialized_whole_struct_patch_earns_fresh_authority_from_the_current_contract()",
    ),
    row(
        22,
        "crates/worth-foundational/tests/certification/aspects/portable/state_boundary.rs",
        "fn a_late_invalid_entry_denies_the_complete_snapshot()",
    ),
    row(
        22,
        "crates/worth-foundational/tests/certification/aspects/portable/patch_boundary.rs",
        "fn field_set_and_clear_survive_without_becoming_a_field_map_authority()",
    ),
    row(
        22,
        "crates/worth-foundational/tests/certification/aspects/portable/readmission_denials.rs",
        "fn malformed_value_and_illegal_field_mask_deny_at_foundational_admission()",
    ),
    row(
        22,
        "crates/worth-foundational/tests/compile_time_boundaries.rs",
        "fn portable_aspect_candidates_cannot_cross_authoritative_boundaries()",
    ),
    row(
        23,
        "crates/worth-relational/src/tests/transactions/core/native_aspect_mutations.rs",
        "fn native_entity_patch_supports_optional_whole_clear()",
    ),
    row(
        23,
        "crates/worth-relational/src/tests/transactions/core/native_aspect_mutations.rs",
        "fn native_relation_creation_synthesizes_endpoint_aspects_and_publishes_updates()",
    ),
    row(
        24,
        "crates/worth-relational/src/tests/transactions/core/native_aspect_conflicts.rs",
        "fn native_same_record_updates_conflict_before_truth_or_publication_changes()",
    ),
    row(
        24,
        "crates/worth-relational/src/tests/transactions/core/native_aspect_conflicts.rs",
        "fn native_create_merge_is_stable_across_batch_permutations()",
    ),
    row(
        24,
        "crates/worth-relational/src/tests/transactions/core/native_aspect_conflicts.rs",
        "fn compatibility_and_native_scalar_authoring_publish_identical_patch_meaning()",
    ),
    row(
        24,
        "crates/worth-relational/src/tests/transactions/core/native_aspect_conflicts.rs",
        "fn compatibility_and_native_updates_on_one_target_have_one_conflict_law()",
    ),
    row(
        24,
        "crates/worth-relational/src/tests/transactions/core/native_aspect_conflicts.rs",
        "fn mixed_native_entity_and_relation_updates_share_one_atomic_commit()",
    ),
    row(
        25,
        "crates/worth-relational/src/tests/transactions/core/native_value_matrix.rs",
        "fn ordinary_native_mutation_roundtrips_every_foundational_scalar_family()",
    ),
    row(
        25,
        "crates/worth-relational/src/tests/transactions/core/native_aspect_mutations.rs",
        "fn native_patch_state_survives_checkpoint_readmission()",
    ),
    row(
        25,
        "crates/worth-relational/src/tests/transactions/core/native_durability.rs",
        "fn native_struct_reference_and_clear_state_survive_checkpoint_readmission()",
    ),
    row(
        25,
        "crates/worth-relational/src/aspect_wire/aspect_value_canonical_codec_tests.rs",
        "fn native_aspect_value_decode_returns_error_for_malformed_canonical_body()",
    ),
    row(
        26,
        "crates/worth-query/tests/native_aspect_mutation_public_dx.rs",
        "fn ordinary_mutation_roundtrips_every_foundational_scalar_family()",
    ),
    row(
        26,
        "crates/worth-query/tests/native_aspect_mutation_public_dx.rs",
        "fn ordinary_mutation_preserves_struct_set_clear_null_and_denial_boundaries()",
    ),
    row(
        26,
        "crates/worth-query/tests/native_aspect_mutation_public_dx.rs",
        "fn invalid_native_values_deny_before_lower_runtime_execution_without_residue()",
    ),
    row(
        26,
        "crates/worth-query/tests/aspect_native_query_compile_fail.rs",
        "fn aspect_native_query_boundaries_are_compile_time_enforced()",
    ),
    row(
        27,
        "crates/worth-query/tests/native_predicate_contract_matrix.rs",
        "fn equality_and_membership_preserve_every_exact_native_operand()",
    ),
    row(
        27,
        "crates/worth-query/tests/native_predicate_contract_matrix.rs",
        "fn incompatible_native_operators_deny_during_schema_validation()",
    ),
    row(
        27,
        "crates/worth-query/tests/native_predicate_contract_matrix.rs",
        "fn typed_schema_contract_mapping_preserves_every_native_family()",
    ),
    row(
        28,
        "crates/worth-query/src/projection_consumption/tests/phase_four/bridge_native_shapes.rs",
        "fn bridge_row_set_preserves_complete_struct_values_through_consumption()",
    ),
    row(
        28,
        "crates/worth-query/src/projection_consumption/tests/phase_four/extraction.rs",
        "fn relational_row_set_preserves_struct_facts_and_typed_refinement_denials()",
    ),
    row(
        28,
        "crates/worth-query/src/projection_consumption/tests/retained_live/native_shapes.rs",
        "fn retained_derived_consumption_preserves_complete_struct_values()",
    ),
    row(
        28,
        "crates/worth-query/src/projection_consumption/tests/retained_live/native_shapes.rs",
        "fn live_artifact_consumption_preserves_complete_struct_values()",
    ),
    row(
        28,
        "crates/worth-query/src/projection_consumption/consumed/native_refinement.rs",
        "fn borrowed_refinement_preserves_every_foundational_scalar_family()",
    ),
    row(
        29,
        "crates/worth-query/src/runtime/tests/native_value_identity_basis.rs",
        "fn every_native_scalar_uses_one_foundational_basis_across_query_identity_contexts()",
    ),
    row(
        29,
        "crates/worth-query/src/consumer_kit/native_value_authority_inventory/tests.rs",
        "fn source_inventory_rejects_seeded_debug_identity_encoder()",
    ),
    row(
        29,
        "crates/worth-query/src/runtime/tests/native_value_identity_basis.rs",
        "fn struct_identity_basis_is_preserved_without_flattening_and_remains_domain_separated()",
    ),
    row(
        29,
        "crates/worth-query/src/runtime/tests/native_value_identity_basis.rs",
        "fn width_reference_and_interning_distinctions_survive_every_query_identity_context()",
    ),
    row(
        30,
        "crates/hadwiger-research/src/query_entry/ordinary_query_tests.rs",
        "fn hadwiger_candidate_search_uses_the_installed_domain_read_journey()",
    ),
    row(
        30,
        "crates/hadwiger-research/src/query_entry/ordinary_query_tests.rs",
        "fn hadwiger_contribution_lowers_through_the_installed_handle()",
    ),
    row(
        30,
        "workspaces/worth-ui/crates/worth-ui-query-binding/src/installed_measurements_tests.rs",
        "fn worth_ui_executes_the_installed_domain_read_projection_and_inspection_journey()",
    ),
    row(
        30,
        "workspaces/worth-ui/crates/worth-ui-query-binding/src/installed_measurements_tests.rs",
        "fn worth_ui_executes_installed_workflow_contribution_and_invariant_journeys()",
    ),
    row(
        30,
        "workspaces/worth-ui/crates/worth-ui-query-binding/src/installed_measurements_tests.rs",
        "fn worth_ui_installed_live_handle_owns_activation_and_disposal()",
    ),
    row(
        30,
        "workspaces/worth-ui/crates/worth-ui-query-binding/src/prerequisites/query_measurement_fact_receipt_tests.rs",
        "fn measurement_fact_receipts_follow_real_projection_consumption_and_preserve_identity()",
    ),
    row(
        30,
        "crates/worth-query/docs/capabilities/native-aspect-values.md",
        "# Native Aspect Values",
    ),
];

const fn row(
    phase: u8,
    path: &'static str,
    probe: &'static str,
) -> WorthQueryMilestoneNineThirteenNativeValueEvidenceRow {
    WorthQueryMilestoneNineThirteenNativeValueEvidenceRow::new(phase, path, probe)
}

pub fn worth_query_milestone_nine_thirteen_native_value_evidence_rows(
) -> &'static [WorthQueryMilestoneNineThirteenNativeValueEvidenceRow] {
    EVIDENCE
}
