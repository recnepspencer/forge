use super::{ControlledMutation, MutationTarget};

mod authority_accounting;
mod closeout_cost;
mod evidence_integrity;
mod ledger_accounting;
mod process_accounting;
mod timing_guard;
mod wal_lifecycle_evidence;
mod wal_reopen_cleanup;
mod wal_successor_cleanup;

pub(super) const AUTHORITY_ACCOUNTING_MUTATIONS: &[ControlledMutation] =
    authority_accounting::MUTATIONS;
pub(super) const CLOSEOUT_COST_MUTATIONS: &[ControlledMutation] = closeout_cost::MUTATIONS;
pub(super) const EVIDENCE_INTEGRITY_MUTATIONS: &[ControlledMutation] =
    evidence_integrity::MUTATIONS;
pub(super) const LEDGER_ACCOUNTING_MUTATIONS: &[ControlledMutation] = ledger_accounting::MUTATIONS;
pub(super) const PROCESS_ACCOUNTING_MUTATIONS: &[ControlledMutation] =
    process_accounting::MUTATIONS;
pub(super) const TIMING_GUARD_MUTATIONS: &[ControlledMutation] = timing_guard::MUTATIONS;
pub(super) const WAL_LIFECYCLE_EVIDENCE_MUTATIONS: &[ControlledMutation] =
    wal_lifecycle_evidence::MUTATIONS;
pub(super) const WAL_REOPEN_CLEANUP_MUTATIONS: &[ControlledMutation] =
    wal_reopen_cleanup::MUTATIONS;
pub(super) const WAL_SUCCESSOR_CLEANUP_MUTATIONS: &[ControlledMutation] =
    wal_successor_cleanup::MUTATIONS;

pub(super) const MUTATIONS: &[ControlledMutation] = &[
    ControlledMutation {
        id: 79,
        predicate: "c7-source-schedule-domain-drifted",
        source: "crates/worth-store-physical-certification/src/schedule/seed.rs",
        needle: "digest.update(b\"store.physical-reconstruction.c7.schedule.v1\");",
        replacement: "digest.update(b\"worth.store.physical.schedule-perturbation.v1\");",
        package: "store-test-runner",
        target: MutationTarget::LibraryWithFeatures {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::schedule::tests::source_closure_seed_domain_is_c7_specific_and_stable",
    },
    ControlledMutation {
        id: 80,
        predicate: "c7-crash-seam-rotation-collapsed",
        source: "crates/worth-store-physical-certification/src/schedule/crash_seam.rs",
        needle: "Some(Self::ALL[lane % Self::ALL.len()])",
        replacement: "Some(Self::ALL[0])",
        package: "store-test-runner",
        target: MutationTarget::LibraryWithFeatures {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::schedule::tests::sixteen_ci_lanes_cover_every_explicit_c7_crash_seam_twice",
    },
    ControlledMutation {
        id: 81,
        predicate: "abort-recovery-handoff-minted",
        source: "crates/worth-store/src/physical_runtime/instance/lifecycle.rs",
        needle: "if matches!(self.stop, PhysicalWorkStopKind::Close) {",
        replacement: "if true {",
        package: "worth-store",
        target: MutationTarget::Integration("physical_record_journeys"),
        selector: "durability_admission::closeout_handoff::abort_never_mints_a_recovery_handoff",
    },
    ControlledMutation {
        id: 82,
        predicate: "completed-unobserved-identity-dropped",
        source: "crates/worth-store/src/physical_runtime/instance/lifecycle.rs",
        needle: "completed_unobserved: Some(completed_unobserved),",
        replacement: "completed_unobserved: Some(Vec::new().into_boxed_slice()),",
        package: "worth-store",
        target: MutationTarget::Integration("physical_record_journeys"),
        selector: "durability_admission::closeout_handoff::completed_but_unobserved_identity_survives_into_operation_fates",
    },
    ControlledMutation {
        id: 83,
        predicate: "reopened-previous-root-dropped",
        source: "crates/worth-store/src/physical_runtime/record_serving/admission/open.rs",
        needle: "let previous_root = if generation == 1 {\n        None\n    } else {\n        let previous = CurrentRootAdmission {\n            generation: generation - 1,\n            ..admission\n        };\n        Some(load_root_manifest(&previous)?)\n    };",
        replacement: "let previous_root = None;",
        package: "worth-store",
        target: MutationTarget::Integration("physical_record_journeys"),
        selector: "durability_admission::closeout_handoff::published_and_reopened_closeout_retains_current_and_immediate_previous_roots",
    },
    ControlledMutation {
        id: 84,
        predicate: "performance-counter-mismatch-accepted",
        source: "crates/worth-store/src/physical_runtime/durability/evidence_projection/performance/receipt.rs",
        needle: "if expected_rows != observed_rows {",
        replacement: "if false && expected_rows != observed_rows {",
        package: "worth-store",
        target: MutationTarget::Integration("physical_record_journeys"),
        selector: "durability_admission::closeout_handoff::every_governed_performance_claim_requires_exact_closeout_counters",
    },
    ControlledMutation {
        id: 85,
        predicate: "compiled-source-identity-omits-source-tree",
        source: "crates/worth-store/build.rs",
        needle: "    collect_files(&manifest.join(\"src\"), &mut inputs);",
        replacement: "    let _omitted_source_tree = manifest.join(\"src\");",
        package: "worth-store",
        target: MutationTarget::Integration("physical_record_journeys"),
        selector: "durability_admission::closeout_source_identity::compiled_closeout_identity_covers_the_complete_store_source_tree",
    },
    ControlledMutation {
        id: 86,
        predicate: "c7-selected-schedule-not-propagated",
        source: "tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege/c7_crash_campaign.rs",
        needle: "        checkpoint_order.encoded().into(),",
        replacement: "        DurabilityCheckpointOrder::CheckpointBeforeTarget.encoded().into(),",
        package: "store-test-runner",
        target: MutationTarget::LibraryWithFeatures {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::c7_crash_campaign::tests::selected_schedule_and_seam_reach_the_c7_child_exactly",
    },
    ControlledMutation {
        id: 87,
        predicate: "c7-case-timing-omitted",
        source: "tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege/timing/mod.rs",
        needle: "            case_count: u64::try_from(case_count).ok(),",
        replacement: "            case_count: None,",
        package: "store-test-runner",
        target: MutationTarget::LibraryWithFeatures {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::timing::tests::timing_fixtures_are_valid_before_hostile_deltas",
    },
    ControlledMutation {
        id: 88,
        predicate: "c7-physical-work-signal-family-stale",
        source: "crates/worth-store/src/bin/physical_store_work_courtroom/bounded_residency/work_reconciliation/causal_record.rs",
        needle: "    if operation.required_signal_family() != family {",
        replacement: "    let _declared_family = operation.required_signal_family();\n    if worth_store::physical_runtime::PhysicalWorkSignalFamily::ExactWriteback != family {",
        package: "store-test-runner",
        target: MutationTarget::NestedExecutableLibrary {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::mutation_proofs::physical_work_topology_falsifies_unsettled_metadata_read",
    },
    ControlledMutation {
        id: 89,
        predicate: "c7-zero-checkpoint-accepted",
        source: "tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege/oracle/performance.rs",
        needle: "    if checkpoint_started != 1 {",
        replacement: "    if checkpoint_started > 1 {",
        package: "store-test-runner",
        target: MutationTarget::LibraryWithFeatures {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::oracle::performance::tests::zero_started_checkpoints_are_rejected",
    },
    ControlledMutation {
        id: 90,
        predicate: "c7-positioned-write-accounting-stale",
        source: "crates/worth-store/src/bin/physical_store_work_courtroom/bounded_residency/writeback_pressure/append_pressure.rs",
        needle: "        || positioned_write_delta\n            != super::CANDIDATE_WRITEBACK_POSITIONED_WRITE_ORDINAL.saturating_add(1)",
        replacement: "        || positioned_write_delta != 3",
        package: "store-test-runner",
        target: MutationTarget::NestedExecutableLibrary {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::mutation_proofs::physical_work_topology_falsifies_unsettled_metadata_read",
    },
    ControlledMutation {
        id: 91,
        predicate: "c7-performance-evidence-omitted",
        source: "crates/worth-store/src/bin/physical_store_work_courtroom/bounded_residency/protocol.rs",
        needle: "    performance::emit(&evidence.close)?;",
        replacement: "    let _performance_evidence_omitted: fn(\n        &ServingShutdownOutcome<ClosedRuntime>,\n    ) -> Result<(), String> = performance::emit;",
        package: "store-test-runner",
        target: MutationTarget::NestedExecutableLibrary {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::mutation_proofs::physical_work_topology_falsifies_unsettled_metadata_read",
    },
    ControlledMutation {
        id: 92,
        predicate: "phase-ten-public-surface-family-omitted",
        source: "tools/store-test-runner/src/durable_publication_boundary_gate/public_api/mod.rs",
        needle: "        .chain(PHASE_TEN_SURFACES)",
        replacement: "        .chain([])",
        package: "store-test-runner",
        target: MutationTarget::Library,
        selector: "durable_publication_boundary_gate::public_api::every_locked_public_surface_resolves_and_has_one_final_disposition",
    },
    ControlledMutation {
        id: 93,
        predicate: "c8-handoff-authority-lane-omitted",
        source: "tools/store-test-runner/src/durable_publication_boundary_gate/authority_trace.rs",
        needle: "    \"successor-extraction\",",
        replacement: "    \"diagnostic-evidence-projection\",",
        package: "store-test-runner",
        target: MutationTarget::Library,
        selector: "durable_publication_boundary_gate::authority_trace::every_current_authority_lane_resolves_to_ordered_production_sources",
    },
    ControlledMutation {
        id: 94,
        predicate: "phase-ten-removal-inventory-unclassified-source-accepted",
        source: "tools/store-test-runner/src/durable_publication_boundary_gate/inventory/mod.rs",
        needle: "        .filter(|path| !deleted_paths.contains(*path))",
        replacement: "        .filter(|_| false)",
        package: "store-test-runner",
        target: MutationTarget::Library,
        selector: "durable_publication_boundary_gate::inventory::reconciliation_rejects_omission_stale_row_and_family_drift",
    },
    ControlledMutation {
        id: 95,
        predicate: "phase-ten-ledger-reopened-history-accepted",
        source: "tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger/phase_ten.rs",
        needle: "    if let Some(row) = rows.iter().find(|row| row.status != LedgerStatus::Proved) {",
        replacement: "    if let Some(row) = rows.iter().find(|_| false) {",
        package: "store-test-runner",
        target: MutationTarget::Library,
        selector: "durable_publication_boundary_gate::closure_ledger::phase_ten::phase_ten_validator_rejects_omission_wrong_phase_stale_identity_and_reopened_history",
    },
    ControlledMutation {
        id: 96,
        predicate: "phase-ten-ledger-guarantee-set-truncated",
        source: "tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger/phase_ten.rs",
        needle: "    \"C7-LEDGER-02\",\n];",
        replacement: "    \"C7-COURTROOM-01\",\n];",
        package: "store-test-runner",
        target: MutationTarget::Library,
        selector: "durable_publication_boundary_gate::closure_ledger::phase_ten::phase_ten_guarantee_set_is_exact",
    },
    ControlledMutation {
        id: 97,
        predicate: "canonical-wal-origin-sealed",
        source: "crates/worth-store/src/physical_runtime/durability/wal/inventory/reopen.rs",
        needle: "    let requires_inspection =\n        cutoff.lsn().is_none() && !segment_inventory.retains_canonical_wal_origin();",
        replacement: "    let requires_inspection = cutoff.lsn().is_none();",
        package: "worth-store",
        target: MutationTarget::Integration("physical_record_journeys"),
        selector: "durability_admission::wal_rotation::whole_groups_rotate_twice_and_reopen_reconstructs_the_exact_bounded_inventory",
    },
    ControlledMutation {
        id: 98,
        predicate: "c7-bounded-checkpoint-memory-unadmitted",
        source: "crates/worth-store/src/bin/physical_store_work_courtroom/bounded_residency/configuration.rs",
        needle: "std::num::NonZeroU64::new(self.checkpoint_memory_bytes)",
        replacement: "std::num::NonZeroU64::new(16 * 1024 * 1024)",
        package: "store-test-runner",
        target: MutationTarget::NestedExecutableLibrary {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::c7_crash_campaign::tests::bounded_checkpoint_memory_reaches_the_selected_termination_point",
    },
    ControlledMutation {
        id: 99,
        predicate: "c7-wal-write-role-mismatch",
        source: "crates/worth-store/src/bin/physical_store_work_courtroom/c7_crash.rs",
        needle: "            Self::BeforeWalAppend => (\n                MediaOperationRole::PositionedWrite,\n                first,\n                MediaFaultDirective::PauseBefore(gate),\n            ),",
        replacement: "            Self::BeforeWalAppend => (\n                MediaOperationRole::Append,\n                first,\n                MediaFaultDirective::PauseBefore(gate),\n            ),",
        package: "store-test-runner",
        target: MutationTarget::NestedExecutableLibrary {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::c7_crash_campaign::tests::bounded_checkpoint_memory_reaches_the_selected_termination_point",
    },
    ControlledMutation {
        id: 100,
        predicate: "c7-wal-barrier-role-mismatch",
        source: "crates/worth-store/src/bin/physical_store_work_courtroom/c7_crash.rs",
        needle: "                MediaOperationRole::SynchronizeFileState,",
        replacement: "                MediaOperationRole::SynchronizeFileData,",
        package: "store-test-runner",
        target: MutationTarget::NestedExecutableLibrary {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::c7_crash_campaign::tests::wal_barrier_boundary_reaches_the_selected_operation",
    },
    ControlledMutation {
        id: 101,
        predicate: "c7-data-write-relative-selection-mismatch",
        source: "crates/worth-store/src/bin/physical_store_work_courtroom/c7_crash.rs",
        needle: "        let second = NonZeroU64::new(2).expect(\"two is nonzero\");",
        replacement: "        let second = NonZeroU64::MIN;",
        package: "store-test-runner",
        target: MutationTarget::NestedExecutableLibrary {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::c7_crash_campaign::tests::data_write_boundary_reaches_the_second_positioned_write",
    },
    ControlledMutation {
        id: 102,
        predicate: "c7-marker-media-role-unchecked",
        source: "tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege/c7_crash_campaign/checkpoint.rs",
        needle: "        || detail[0] != expected.role",
        replacement: "        || false",
        package: "store-test-runner",
        target: MutationTarget::LibraryWithFeatures {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::c7_crash_campaign::tests::checkpoint_verification_rejects_the_wrong_media_role",
    },
    ControlledMutation {
        id: 103,
        predicate: "c7-marker-relative-match-unchecked",
        source: "tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege/c7_crash_campaign/checkpoint.rs",
        needle: "        || *selected_match != expected.selected_match",
        replacement: "        || false",
        package: "store-test-runner",
        target: MutationTarget::LibraryWithFeatures {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::c7_crash_campaign::tests::checkpoint_verification_rejects_the_wrong_relative_match",
    },
    ControlledMutation {
        id: 104,
        predicate: "c7-offline-bounded-schema-v3-rejected",
        source: "crates/worth-store-offline-verifier/src/bin/physical_store_offline_observer/bounded_residency_verification/configuration.rs",
        needle: "const SCHEMA: &str = \"worth.store.physical-work-courtroom.bounded-residency.configuration.v3\";",
        replacement: "const SCHEMA: &str = \"worth.store.physical-work-courtroom.bounded-residency.configuration.v2\";",
        package: "worth-store-offline-verifier",
        target: MutationTarget::Binary("physical_store_offline_observer"),
        selector: "bounded_residency_verification::configuration::tests::current_hostile_profile_is_accepted_independently",
    },
    ControlledMutation {
        id: 105,
        predicate: "c7-reopen-bounded-policy-discarded",
        source: "crates/worth-store/src/bin/physical_store_work_courtroom/configuration.rs",
        needle: "                    .map(Self::BoundedResidency)",
        replacement: "                    .map(|_| Self::Standard)",
        package: "worth-store",
        target: MutationTarget::Binary("physical_store_work_courtroom"),
        selector: "bounded_residency::configuration::tests::reopen_retains_the_bounded_checkpoint_memory_policy",
    },
    ControlledMutation {
        id: 106,
        predicate: "c7-offline-empty-prefix-ambiguous",
        source: "crates/worth-store-offline-verifier/src/bin/physical_store_offline_observer/hostile_physical_truth.rs",
        needle: "        \"-\".to_owned()",
        replacement: "        super::hex(prefix)",
        package: "worth-store-offline-verifier",
        target: MutationTarget::Binary("physical_store_offline_observer"),
        selector: "hostile_physical_truth::tests::empty_prefix_uses_explicit_protocol_token",
    },
    ControlledMutation {
        id: 107,
        predicate: "c7-reopen-exact-wal-overlap-rejected",
        source: "crates/worth-store/src/physical_runtime/durability/mutation/idempotency/bootstrap.rs",
        needle: "        Ok(()) | Err(PhysicalMutationWalBindingDenial::AlreadyWalBound) => Ok(()),",
        replacement: "        Ok(()) => Ok(()),",
        package: "worth-store",
        target: MutationTarget::Library,
        selector: "physical_runtime::durability::mutation::idempotency::bootstrap::tests::reopen_accepts_only_new_or_exactly_repeated_wal_bindings",
    },
    ControlledMutation {
        id: 108,
        predicate: "c7-serving-checkpoint-omitted",
        source: "crates/worth-store/src/bin/physical_store_work_courtroom/bounded_residency/serving.rs",
        needle: "    super::checkpoint::complete_reliability_seed(&serving)?;",
        replacement: "    let _serving_checkpoint_omitted =\n        super::checkpoint::complete_reliability_seed;",
        package: "store-test-runner",
        target: MutationTarget::NestedExecutableLibrary {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::mutation_proofs::physical_work_topology_falsifies_unsettled_metadata_read",
    },
    ControlledMutation {
        id: 109,
        predicate: "c7-post-boundary-recovery-residue-rejected",
        source: "tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege/oracle/c7_campaign.rs",
        needle: "    if seam.interrupts_unsettled_media_effect() {",
        replacement: "    if false {",
        package: "store-test-runner",
        target: MutationTarget::LibraryWithFeatures {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::oracle::c7_campaign::tests::unsettled_media_points_require_one_recovery_obligation",
    },
    ControlledMutation {
        id: 110,
        predicate: "c7-settled-boundary-residue-required",
        source: "tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege/oracle/c7_campaign.rs",
        needle: "    } else {\n        0\n    }",
        replacement: "    } else {\n        1\n    }",
        package: "store-test-runner",
        target: MutationTarget::LibraryWithFeatures {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::oracle::c7_campaign::tests::settled_between_effect_points_require_zero_recovery_obligations",
    },
    ControlledMutation {
        id: 111,
        predicate: "c7-case-process-role-collapsed",
        source: "tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege/c7_crash_campaign.rs",
        needle: "        format!(\"c7:{}:{}\", seam.label(), self.label())",
        replacement: "        format!(\"c7:{}\", self.label())",
        package: "store-test-runner",
        target: MutationTarget::LibraryWithFeatures {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::c7_crash_campaign::tests::every_case_process_role_is_globally_unique",
    },
];
