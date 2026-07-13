pub const CASES: [(&str, &str); 9] = [
    (
        "milestone_selected_operation_names_are_removed.rs",
        "no `S8IndexedSelectedAccessPlan` in the root",
    ),
    (
        "physical_compaction_owner_case_id_is_not_constructible.rs",
        "private fields",
    ),
    (
        "physical_compaction_owner_case_is_not_constructible.rs",
        "private",
    ),
    (
        "verifier_protocol_cannot_certify_exact.rs",
        "expected `&ExactMaintenanceProtocol`, found `&VerifierMaintenanceProtocol`",
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
];
