pub const CASES: [(&str, &str); 35] = [
    (
        "artifact_family_admission_outcome_is_not_constructible.rs",
        "field `case` of struct `ArtifactFamilyAdmissionOutcome` is private",
    ),
    (
        "physical_key_domain_admission_outcome_is_not_constructible.rs",
        "field `case` of struct `PhysicalKeyDomainAdmissionOutcome` is private",
    ),
    (
        "materialization_admission_outcome_is_not_constructible.rs",
        "field `case` of struct `CatalogRootMaterializationAdmissionOutcome` is private",
    ),
    (
        "layout_strategy_registry_snapshot_is_not_constructible.rs",
        "field `inner` of struct `LayoutStrategyRegistrySnapshot` is private",
    ),
    (
        "compatibility_receipt_is_not_constructible.rs",
        "associated function `new` is private",
    ),
    (
        "legacy_selected_operation_alias_is_removed.rs",
        "no `S8IndexedSelectedAccessPlan` in the root",
    ),
    (
        "physical_compaction_owner_case_id_is_not_constructible.rs",
        "enum has no tuple variants to construct",
    ),
    (
        "physical_compaction_owner_case_is_not_constructible.rs",
        "private",
    ),
    (
        "verifier_protocol_cannot_certify_exact.rs",
        "module `maintenance` is private",
    ),
    (
        "migration_outcome_has_no_generic_projection.rs",
        "no method named `into_transition_outcome`",
    ),
    (
        "generic_maintenance_transition_is_removed.rs",
        "no `S8IndexMaintenanceTransitionOutcome` in `maintenance`",
    ),
    (
        "generic_integrity_readmission_is_removed.rs",
        "no method named `readmit_with`",
    ),
    (
        "offline_requirement_cannot_enter_import_readmission.rs",
        "expected `ImportReadmissionRequirement`, found `OfflineReadmissionRequirement`",
    ),
    (
        "offline_requirement_is_not_constructible.rs",
        "fields `family` and `identity` of struct `OfflineReadmissionRequirement` are private",
    ),
    (
        "live_exact_maintenance_outcome_is_not_constructible.rs",
        "field `witness` of struct `LiveExactMaintenanceOutcome` is private",
    ),
    (
        "live_maintenance_posture_outcome_is_not_constructible.rs",
        "field `case` of struct `LiveMaintenancePostureOutcome` is private",
    ),
    (
        "layout_mutation_admission_outcome_is_not_constructible.rs",
        "field `case` of struct `LayoutMutationAdmissionOutcome` is private",
    ),
    (
        "copy_on_write_layout_mutation_request_fields_are_private.rs",
        "fields `strategy`, `plan`, `materialization` and `current_security` of struct `CopyOnWriteLayoutMutationRequest` are private",
    ),
    (
        "layout_binding_witness_is_not_constructible.rs",
        "cannot construct `LayoutBindingWitness` with struct literal syntax due to private fields",
    ),
    (
        "rebuild_admission_outcome_is_not_constructible.rs",
        "field `case` of struct `DerivedIndexRebuildAdmissionOutcome` is private",
    ),
    (
        "btree_lookup_readiness_outcome_is_not_constructible.rs",
        "field `case` of struct `BTreeLookupReadinessOutcome` is private",
    ),
    (
        "btree_lookup_execution_outcome_is_not_constructible.rs",
        "field `case` of struct `BTreeLookupExecutionOutcome` is private",
    ),
    (
        "migration_interruption_outcome_is_not_constructible.rs",
        "field `case` of struct `LayoutMigrationInterruptionOutcome` is private",
    ),
    (
        "derived_index_parity_basis_is_not_constructible.rs",
        "struct `LayoutCoverageWitness` is private",
    ),
    (
        "copy_on_write_mutation_receipt_is_not_constructible.rs",
        "fields `family`, `maintenance_mode` and `publication` of struct `CopyOnWriteLayoutMutationReceipt` are private",
    ),
    (
        "exact_btree_publication_evidence_is_not_constructible.rs",
        "fields `family`, `maintenance_mode`, `coverage` and `counters` of struct `ExactBTreePublicationEvidence` are private",
    ),
    (
        "owner_case_observation_is_not_constructible.rs",
        "field `case_id` of struct `OwnerCaseObservation` is private",
    ),
    (
        "declared_case_is_not_an_observed_case.rs",
        "expected struct `OwnerCaseObservation<BTreeLookupReadinessCaseId>`",
    ),
    (
        "caller_cannot_construct_admitted_family.rs",
        "AdmittedLayoutFamily",
    ),
    (
        "caller_cannot_construct_current_binding.rs",
        "CurrentMaterializationBinding",
    ),
    (
        "caller_cannot_construct_plan_identity.rs",
        "LayoutPlanIdentity",
    ),
    (
        "caller_cannot_construct_ready_indexed_access.rs",
        "ReadyIndexedAccess",
    ),
    (
        "caller_cannot_mint_strategy_evidence.rs",
        "StrategyInvariantEvidence",
    ),
    (
        "caller_cannot_pair_access_with_copied_counters.rs",
        "PhysicalAccessCounterReceipt",
    ),
    (
        "removed_access_lowering_facade_is_private.rs",
        "access_lowering",
    ),
];
