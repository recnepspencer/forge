#[derive(Clone, Copy)]
pub(super) struct MutantExpectation {
    pub(super) predicate: &'static str,
    pub(super) source: &'static str,
    pub(super) scenario: &'static str,
}

pub(super) fn for_id(id: u8) -> MutantExpectation {
    MutantExpectation {
        predicate: predicate(id),
        source: source(id),
        scenario: scenario(id),
    }
}

fn predicate(id: u8) -> &'static str {
    match id {
        15 => "settlement",
        16 => "scheduler-admission",
        17 => "backend-receipt",
        18 => "derived-completion",
        19 => "post-dispatch-cancellation",
        20 => "stale-generation",
        21 => "health-revocation",
        22 => "physical-effect-no-retry",
        23 => "duplicate-work-registry",
        24..=26 => "store-local-async-registry",
        27 => "lifecycle-duplication",
        28 => "serialized-signal-reopen",
        29 => "internal-json-carrier",
        30 => "legacy-resource-node",
        31 => "raw-signal-slot-authority",
        32 => "foundational-mask-substitution",
        33 => "aspect-partition-broadening",
        34 => "global-mutation-lock",
        35 => "branch-label-disjointness",
        36 => "signal-evaluation-effect",
        37 => "generic-signal-settlement",
        38 => "scheduler-counter-settlement",
        39 => "skipped-backend-write",
        40 => "raw-backend-dispatch",
        41 => "duplicate-signal-dependency-authority",
        42 => "candidate-clean-without-exact-receipt",
        43 => "local-physical-work-scheduler",
        44 => "writeback-clean-without-exact-receipt",
        _ => unreachable!("campaign shape validates the mutant range"),
    }
}

fn source(id: u8) -> &'static str {
    match id {
        15 => "crates/worth-store/src/physical_runtime/work/execution/settlement.rs",
        16 => "crates/worth-store-io-scheduler/src/queue_execution/admission/request.rs",
        17 => "crates/worth-store/src/physical_runtime/work/progression/settlement/dispatched.rs",
        18 => "crates/worth-store/src/physical_runtime/instance/signal_owner/lifecycle_join.rs",
        19 | 24 | 25 | 27 => "crates/worth-store/src/physical_runtime/instance/work_lifecycle.rs",
        20 | 34 => "crates/worth-store/src/physical_runtime/instance/work_runtime.rs",
        21 => "crates/worth-store/src/physical_runtime/record_serving/lifecycle/serving_health.rs",
        22 => "crates/worth-store/src/physical_runtime/instance/signal_owner/reconciliation.rs",
        23 => "crates/worth-store/src/physical_runtime/work/submission/capacity_lease.rs",
        28 | 29 => "crates/worth-store/src/physical_runtime/record_serving/admission/open.rs",
        26 | 30 => "crates/worth-store/src/physical_runtime/instance/signal_owner/mod.rs",
        31 => "crates/worth-store/src/physical_runtime/work/profile/aspect_bindings.rs",
        32 => "crates/worth-store/src/physical_runtime/work/profile/declaration.rs",
        33 | 35 => "crates/worth-store/src/physical_runtime/work/concurrency_scope.rs",
        36 => "crates/worth-store/src/physical_runtime/instance/signal_owner/graph.rs",
        37 => "crates/worth-store/src/physical_runtime/instance/signal_owner/lifecycle_join.rs",
        38 => "crates/worth-store/src/physical_runtime/work/execution/settlement/classification.rs",
        39 => "crates/worth-store/src/physical_runtime/instance/executor/range_write.rs",
        40 => "crates/worth-store/src/physical_runtime/record_serving/residency/artifact_tree.rs",
        41 => "crates/worth-store/src/physical_runtime/work/signal_declaration.rs",
        42 => "crates/worth-store/src/physical_runtime/record_serving/residency/candidate_frame_residency/write_evidence.rs",
        43 => "crates/worth-store/src/physical_runtime/record_serving/residency/dirty/writeback/admission.rs",
        44 => "crates/worth-store/src/physical_runtime/work/execution/outcome/residency_writeback.rs",
        _ => unreachable!("campaign shape validates the mutant range"),
    }
}

fn scenario(id: u8) -> &'static str {
    match id {
        15 => "physical_work::authority_mutants::physical_settlement_requires_backend_and_scheduler_evidence",
        16 => "queue_execution::tests::admission_lowering::grouping_mismatch_is_a_typed_admission_denial",
        17 => "physical_work::authority_mutants::backend_receipts_cannot_settle_foreign_dispatched_work",
        18 => "physical_work::authority_mutants::derived_completion_must_join_the_real_signal_request",
        19 => "physical_work::post_dispatch_cancellation::cancellation_after_backend_dispatch_retains_terminal_settlement_obligation",
        20 => "physical_work::execution_capability::stale_execution_capability_cannot_cross_the_real_effect_boundary",
        21 => "physical_work::failure::partial_write_retains_exact_prefix_and_revokes_serving_health",
        22 => "physical_work::failure::derived_reconciliation::later_batch_panic_retains_earlier_settlement_without_repeating_media",
        23 => "physical_work::authority_sealing::duplicate_runtime::a_second_pending_work_registry_is_forbidden",
        24 => "physical_work::executor::signal_timeout_uses_deterministic_clock_and_proves_no_dispatch",
        25 => "physical_work::failure::pre_effect_backend_denial_is_the_only_retryable_physical_failure",
        26 => "physical_work::capacity::dropped_ready_work_releases_signal_and_command_capacity_one_before_close",
        27 => "physical_work::authority_sealing::duplicate_runtime::a_second_physical_lifecycle_is_forbidden",
        28 => "physical_work::authority_sealing::reopen_boundary::reopen_cannot_consume_serialized_signal_state",
        29 => "physical_work::authority_sealing::reopen_boundary::ordinary_physical_work_cannot_add_an_internal_json_carrier",
        30 => "physical_work::authority_sealing::semantic_boundary::legacy_signal_resource_construction_is_forbidden",
        31 => "physical_work::authority_sealing::semantic_boundary::raw_signal_slots_cannot_become_semantic_authority",
        32 => "physical_work::authority_sealing::semantic_boundary::foundational_masks_cannot_substitute_for_native_bindings",
        33 => "physical_work::authority_sealing::semantic_boundary::callers_cannot_broaden_aspect_or_partition_scope",
        34 | 35 => "physical_work::concurrency::independent_mutation_capabilities_execute_without_a_global_runtime_borrow",
        36 => "physical_work::authority_mutants::signal_evaluation_is_filesystem_effect_free",
        37 => "physical_work::authority_mutants::generic_signal_completion_cannot_upgrade_proven_no_effect",
        38 => "physical_work::authority_mutants::scheduler_counters_cannot_settle_cross_bound_backend_receipts",
        39 | 40 => "physical_work::authority_mutants::one_canonical_write_requires_one_backend_effect",
        41 => "c5_1_sealing_gate::ordinary_work_sources_keep_aspect_and_semantic_authority_at_typed_boundaries",
        42 => "physical_runtime::record_serving::residency::candidate_frame_residency::tests::exact_receipt::foreign_real_receipt_cannot_settle_candidate_residency",
        43 => "physical_work::residency_writeback_retry::no_effect_writeback_retains_dirty_ownership_through_signal_retry",
        44 => "physical_runtime::work::execution::outcome::residency_writeback::tests::writeback_settlement_rejects_wrong_bytes_and_accepts_exact_physical_receipt",
        _ => unreachable!("campaign shape validates the mutant range"),
    }
}
