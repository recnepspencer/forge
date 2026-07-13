use super::compile_fail_support;

mod runtime_authority_additional_cases;

#[test]
fn parallel_runtime_and_compatibility_modules_are_removed() {
    for (fixture, expected) in [
        (
            "parallel_layout_runtime_is_removed.rs",
            "no `layout_runtime`",
        ),
        ("access_planning_module_is_private.rs", "private"),
        (
            "access_execution_module_is_private.rs",
            "could not find `access_execution`",
        ),
        (
            "physical_layout_access_proxy_is_removed.rs",
            "could not find `layout_access`",
        ),
        (
            "owner_ready_capability_is_not_constructible.rs",
            "are private",
        ),
        (
            "store_authority_and_copied_values_cannot_construct_family.rs",
            "fields `lifecycle`, `security_identity` and `authority_identity` of struct `AdmittedPhysicalArtifactFamily` are private",
        ),
        (
            "degraded_selection_cannot_enter_indexed_lowering.rs",
            "expected `BTreeLookupReady`, found `SelectedDegradedExactScan`",
        ),
        (
            "generic_indexed_plan_cannot_enter_btree_lookup_lowering.rs",
            "no `SelectedIndexedAccessPlan` in the root",
        ),
        (
            "btree_lookup_selection_is_not_constructible.rs",
            "is private",
        ),
        (
            "dead_btree_root_publication_selection_is_removed.rs",
            "no `SelectedBTreeRootPublication` in the root",
        ),
        (
            "lsm_lookup_cannot_enter_btree_lowering.rs",
            "expected `BTreeLookupReady`, found `SelectedLsmLookup`",
        ),
        (
            "btree_lookup_cannot_enter_lsm_admission.rs",
            "expected `SelectedLsmLookup`, found `SelectedBTreeLookup`",
        ),
        (
            "btree_witness_cannot_execute_lookup.rs",
            "no method named `execute_separator_directed_lookup`",
        ),
        (
            "btree_read_source_is_not_constructible.rs",
            "due to private fields",
        ),
        ("btree_replay_source_is_not_constructible.rs", "are private"),
        ("btree_replay_ready_is_not_constructible.rs", "are private"),
        (
            "btree_root_agreement_is_not_constructible.rs",
            "are private",
        ),
        (
            "btree_witness_cannot_execute_replay.rs",
            "no method named `execute_replay_recovery`",
        ),
        (
            "unbound_recovery_intent_cannot_execute_btree_replay.rs",
            "mismatched types",
        ),
        (
            "lsm_lookup_admission_is_not_constructible.rs",
            "are private",
        ),
        (
            "lsm_compaction_admission_is_not_constructible.rs",
            "is private",
        ),
        (
            "lsm_publication_admission_is_not_constructible.rs",
            "is private",
        ),
        (
            "lsm_replay_admission_is_not_constructible.rs",
            "private fields",
        ),
        (
            "lsm_lookup_cannot_enter_compaction_admission.rs",
            "expected `SelectedLsmCompaction`, found `SelectedLsmLookup`",
        ),
        (
            "lsm_compaction_membership_is_not_constructible.rs",
            "are private",
        ),
        (
            "lsm_compaction_demand_is_not_constructible.rs",
            "are private",
        ),
        (
            "lsm_membership_raw_encoders_are_private.rs",
            "no `lsm_membership_digest` in the root",
        ),
        (
            "lsm_membership_session_is_not_constructible.rs",
            "due to private fields",
        ),
        (
            "lsm_activation_declaration_is_not_constructible.rs",
            "are private",
        ),
        (
            "wal_artifact_store_is_not_constructible.rs",
            "field `identity` of struct `AdmittedWalArtifactStore` is private",
        ),
        (
            "lsm_replay_source_is_not_constructible.rs",
            "private fields",
        ),
        (
            "lsm_replay_source_cannot_be_substituted_at_execution.rs",
            "this method takes 1 argument but 2 arguments were supplied",
        ),
        (
            "lsm_witness_cannot_author_replay.rs",
            "no method named `execute_replay_wal_tail`",
        ),
        (
            "prepared_lsm_compaction_is_not_constructible.rs",
            "are private",
        ),
        (
            "prepared_lsm_compaction_cannot_authorize_lookup.rs",
            "no method named `admit_lookup_source`",
        ),
        (
            "prepared_lsm_compaction_cannot_publish_without_interlock.rs",
            "expected `InterlockedLsmCompaction`, found `PreparedLsmCompaction`",
        ),
        (
            "interlocked_lsm_compaction_is_not_constructible.rs",
            "are private",
        ),
        (
            "raw_epochs_cannot_construct_lsm_physical_intent.rs",
            "no function or associated item named `new`",
        ),
        (
            "scalar_lsm_physical_binding_is_removed.rs",
            "no `BaselineLsmPhysicalPublicationBinding` in the root",
        ),
        (
            "lsm_membership_replacement_is_not_constructible.rs",
            "are private",
        ),
        (
            "lsm_replacement_output_is_not_constructible.rs",
            "are private",
        ),
        (
            "generic_checkpoint_cannot_replace_lsm_membership.rs",
            "expected `&AdmittedLsmMembershipReplacement`",
        ),
        (
            "lsm_activation_cannot_form_without_physical_publication.rs",
            "this function takes 3 arguments but 2 arguments were supplied",
        ),
        (
            "lsm_output_cannot_bind_without_physical_intent.rs",
            "this function takes 3 arguments but 2 arguments were supplied",
        ),
        (
            "lsm_replacement_cannot_bind_without_manifest.rs",
            "this function takes 3 arguments but 2 arguments were supplied",
        ),
        (
            "lsm_witness_cannot_execute_unselected_lookup.rs",
            "no method named `execute_lookup_latest_visible_record`",
        ),
        ("lsm_lookup_source_is_not_constructible.rs", "is private"),
        (
            "raw_lsm_identity_cannot_admit_materialization.rs",
            "no method named `admit_lsm_replacement_materialization`",
        ),
        ("wal_lookup_request_is_not_constructible.rs", "are private"),
        ("page_lookup_request_is_not_constructible.rs", "are private"),
        (
            "indexed_selection_cannot_enter_degraded_lowering.rs",
            "no `SelectedIndexedAccessPlan` in the root",
        ),
        (
            "degraded_ready_capability_is_not_constructible.rs",
            "are private",
        ),
        (
            "degraded_generic_counter_admission_is_removed.rs",
            "no method named `admit_counters`",
        ),
        (
            "lsm_counter_observation_has_no_default.rs",
            "no function or associated item named `default`",
        ),
        (
            "lsm_counter_observation_has_no_raw_constructor.rs",
            "no function or associated item named `new`",
        ),
        (
            "btree_lookup_execution_is_not_constructible.rs",
            "private fields",
        ),
        (
            "btree_replay_execution_is_not_constructible.rs",
            "private fields",
        ),
        (
            "lsm_lookup_execution_is_not_constructible.rs",
            "private fields",
        ),
        (
            "lsm_publication_execution_is_not_constructible.rs",
            "private fields",
        ),
        (
            "lsm_replay_execution_is_not_constructible.rs",
            "private fields",
        ),
        (
            "lsm_compaction_execution_is_not_constructible.rs",
            "private fields",
        ),
        (
            "degraded_execution_is_not_constructible.rs",
            "private fields",
        ),
        (
            "executed_operations_cannot_cross_owner_variants.rs",
            "mismatched types",
        ),
        (
            "indexed_generic_execution_lane_is_removed.rs",
            "no `indexed_access_runtime` in the root",
        ),
        (
            "degraded_nonphysical_execute_is_removed.rs",
            "no method named `execute`",
        ),
        (
            "degraded_rebind_cannot_readmit_degraded_stale.rs",
            "no `DegradedScanRebindAdmission`",
        ),
        (
            "degraded_readmission_is_not_constructible.rs",
            "are private",
        ),
        (
            "caller_cannot_author_physical_request_identity.rs",
            "no `StorePhysicalRequestIdentity` in the root",
        ),
        (
            "request_identity_has_no_scalar_authority_projection.rs",
            "no method named `binding_words`",
        ),
        ("raw_request_arguments_cannot_select.rs", "private struct"),
        (
            "admitted_physical_access_request_is_not_constructible.rs",
            "private fields",
        ),
        (
            "admitted_physical_recovery_request_is_not_constructible.rs",
            "private fields",
        ),
        (
            "admitted_physical_mutation_request_is_not_constructible.rs",
            "private fields",
        ),
        (
            "admitted_artifact_family_is_not_constructible.rs",
            "due to private fields",
        ),
        ("admitted_key_domain_is_not_constructible.rs", "are private"),
        (
            "admitted_layout_materialization_is_not_constructible.rs",
            "are private",
        ),
        (
            "materialization_source_identity_is_not_constructible.rs",
            "private fields",
        ),
        (
            "btree_replay_physical_source_identity_is_not_constructible.rs",
            "private fields",
        ),
        (
            "lsm_published_membership_identity_is_not_constructible.rs",
            "private fields",
        ),
        (
            "lsm_replay_source_identity_is_not_constructible.rs",
            "private fields",
        ),
        (
            "admitted_coverage_basis_is_not_constructible.rs",
            "are private",
        ),
        (
            "current_layout_materialization_is_not_constructible.rs",
            "is private",
        ),
        (
            "current_materialization_frontier_is_not_constructible.rs",
            "is private",
        ),
        (
            "stale_layout_materialization_is_not_constructible.rs",
            "are private",
        ),
        (
            "btree_lookup_absence_is_not_constructible.rs",
            "are private",
        ),
        ("lsm_lookup_absence_is_not_constructible.rs", "are private"),
        (
            "lsm_lookup_readiness_outcome_is_not_constructible.rs",
            "field `case` of struct `BaselineLsmLookupAdmissionOutcome` is private",
        ),
        (
            "coverage_cannot_issue_lookup_absence.rs",
            "no method named `prove_exact_index_absence`",
        ),
        (
            "admitted_access_intent_is_not_constructible.rs",
            "are private",
        ),
        ("access_plan_cost_is_not_constructible.rs", "are private"),
        ("access_plan_identity_is_not_constructible.rs", "are private"),
        ("selection_outcome_is_not_constructible.rs", "is private"),
        (
            "budget_receipt_is_not_constructible.rs",
            "fields `request`, `scope` and `admitted_envelope` of struct `PreExecutionBudgetAdmissionReceipt` are private",
        ),
        (
            "raw_concrete_key_cannot_enter_request_admission.rs",
            "private struct",
        ),
        (
            "budget_cannot_issue_physical_operation_readiness.rs",
            "no `StoreRootPublicationReady` in the root",
        ),
        (
            "caller_cannot_construct_physical_operation_readiness.rs",
            "are private",
        ),
        (
            "caller_cannot_construct_selected_owner_operation.rs",
            "no `SelectedIndexedAccessPlan` in the root",
        ),
    ]
    .into_iter()
    .chain(runtime_authority_additional_cases::CASES)
    {
        compile_fail_support::assert_compile_fails_in_ui_dir(
            "runtime_authority",
            fixture,
            &[expected],
            &[
                "forge_store_authority",
                "forge_store_budgets",
                "forge_store_lsm_authority",
                "forge_store_physical_format",
                "forge_store_physical_isolation",
                "forge_store_recovery_physics",
                "forge_store_security",
                "forge_store_wal",
            ],
        );
    }
}
