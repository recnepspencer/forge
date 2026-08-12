use std::path::{Path, PathBuf};

use super::super::repository_root;

mod phase_six_checkpoint;
mod phase_six_idempotency;
mod phase_six_reclamation;

const PHASE_ONE_SOURCE_FILES: &[&str] = &[
    "_docs/worth-store/physical-foundation-reconstruction-roadmap.md",
    "_docs/worth-store/physical-reality-audit.csv",
    "_docs/worth-store/physical-reconstruction-c7-authority-trace.csv",
    "_docs/worth-store/physical-reconstruction-c7-cargo-graph.csv",
    "_docs/worth-store/physical-reconstruction-c7-durable-publication-join.md",
    "_docs/worth-store/physical-reconstruction-c7-public-api.csv",
    "_docs/worth-store/physical-reconstruction-c7-removal-ledger.csv",
    "workspaces/worth-store/Cargo.lock",
    "workspaces/worth-store/tools/store-test-runner/src/lib.rs",
    "workspaces/worth-store/tools/store-test-runner/src/physical_residency_boundary_gate/constructor_syntax.rs",
    "workspaces/worth-store/tools/store-test-runner/src/physical_residency_boundary_gate/mod.rs",
    "workspaces/worth-store/tools/store-test-runner/src/physical_residency_boundary_gate/production_source_graph.rs",
    "workspaces/worth-store/tools/store-test-runner/src/physical_residency_boundary_gate/production_source_graph/macro_provenance.rs",
    "workspaces/worth-store/tools/store-test-runner/src/physical_residency_boundary_gate/production_source_graph/tests.rs",
    "workspaces/worth-store/tools/store-test-runner/src/physical_residency_boundary_gate/production_source_graph/tests/causal_compile.rs",
    "workspaces/worth-store/tools/store-test-runner/src/physical_residency_boundary_gate/production_source_graph/target_roots.rs",
    "workspaces/worth-store/tools/store-test-runner/src/physical_residency_boundary_gate/runtime_ownership.rs",
    "workspaces/worth-store/tools/store-test-runner/src/physical_writer_gate/candidate_publication.rs",
];
const PHASE_ONE_SOURCE_TREES: &[&str] =
    &["workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate"];

const PHASE_TWO_AUTHORITY_SOURCE_FILES: &[&str] = &[
    "workspaces/worth-store/crates/worth-store-physical-backend/src/durability_profile/mod.rs",
    "workspaces/worth-store/crates/worth-store-physical-backend/src/durability_profile/physical_admission_basis.rs",
    "workspaces/worth-store/crates/worth-store-physical-backend/src/filesystem_media/durability_admission.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/construction.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/lifecycle.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/parts.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/admission/request.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/admission/transition.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/lifecycle/serving_runtime.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority_ui.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/contract/durability_owner_lifecycle.rs",
];
const PHASE_TWO_AUTHORITY_SOURCE_TREES: &[&str] = &[
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority/support",
];
const PHASE_TWO_AUTHORITY_COMPILE_ATTACKS: &[&str] = &[
    "admitted_durability_policy_is_sealed",
    "durability_policy_cannot_be_omitted_from_open",
    "incomplete_durability_policy_cannot_admit",
    "physical_durability_basis_cannot_be_duplicated",
    "raw_backend_profile_cannot_admit_durability",
];

const PHASE_TWO_MUTATION_SOURCE_FILES: &[&str] = &[
    "workspaces/worth-store/crates/worth-store-aspect-native/src/canonical_basis.rs",
    "workspaces/worth-store/crates/worth-store-aspect-native/src/canonical_basis/canonical_basis_construction.rs",
    "workspaces/worth-store/crates/worth-store-aspect-native/src/canonical_basis/canonical_basis_domains.rs",
    "workspaces/worth-store/crates/worth-store-aspect-native/src/canonical_basis/canonical_basis_sources.rs",
    "workspaces/worth-store/crates/worth-store-aspect-native/src/canonical_basis/physical_mutation_request.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/construction.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/durable_preparation.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/submission.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/durable_preparation.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/work_semantics/durability/policy_binding_basis.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/work/profile/capability.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/work/profile/identity.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/work/submission/mutation_identity.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/physical_work/durability_signal_binding.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority_ui.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/contract/mutation_preparation.rs",
];
const PHASE_TWO_MUTATION_SOURCE_TREES: &[&str] = &[
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/mutation",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/durable_preparation",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/contract/mutation_preparation",
];
const PHASE_TWO_MUTATION_COMPILE_ATTACKS: &[&str] =
    &["physical_mutation_preparation_authority_is_sealed"];

const PHASE_THREE_WAL_SOURCE_FILES: &[&str] = &[
    "_docs/worth-store/bounded-physical-record-access.md",
    "_docs/worth-store/physical-reality-audit.csv",
    "_docs/worth-store/physical-reconstruction-c7-durable-publication-join.md",
    "_docs/worth-store/physical-reconstruction-c7-phase-3-implementation-plan.md",
    "_docs/worth-store/physical-reconstruction-c7-public-api.csv",
    "_docs/worth-store/physical-reconstruction-c7-removal-ledger.csv",
    "_docs/worth-store/physical-wal-append.md",
    "workspaces/worth-store/crates/worth-store-certification/Cargo.toml",
    "workspaces/worth-store/crates/worth-store-certification/src/courtroom/protocol_models/durability_recovery/scenario.rs",
    "workspaces/worth-store/crates/worth-store-certification/src/courtroom/protocol_models/durability_recovery/tests.rs",
    "workspaces/worth-store/crates/worth-store-formal-models/Cargo.toml",
    "workspaces/worth-store/crates/worth-store-formal-models/src/protocols/durability_recovery/owner_mapping.rs",
    "workspaces/worth-store/crates/worth-store-io-scheduler/src/background_pacing/capacity.rs",
    "workspaces/worth-store/crates/worth-store-io-scheduler/src/background_pacing/streaming_pressure_link.rs",
    "workspaces/worth-store/crates/worth-store-io-scheduler/src/foreground_reservation/fairness.rs",
    "workspaces/worth-store/crates/worth-store-io-scheduler/src/foreground_reservation/lane.rs",
    "workspaces/worth-store/crates/worth-store-io-scheduler/src/foreground_reservation/resource_contract.rs",
    "workspaces/worth-store/crates/worth-store-io-scheduler/src/foreground_reservation/streaming_read_lane_link.rs",
    "workspaces/worth-store/crates/worth-store-io-scheduler/src/queue_execution/observation/replay.rs",
    "workspaces/worth-store/crates/worth-store-io-scheduler/src/queue_execution/policy/physical_work/declaration.rs",
    "workspaces/worth-store/crates/worth-store-physical-backend/src/durability_profile/barrier_receipt.rs",
    "workspaces/worth-store/crates/worth-store-physical-backend/src/durability_profile/physical_admission_basis.rs",
    "workspaces/worth-store/crates/worth-store-physical-backend/src/facade.rs",
    "workspaces/worth-store/crates/worth-store-physical-backend/src/filesystem_media/artifact_tree/artifact_append.rs",
    "workspaces/worth-store/crates/worth-store-physical-backend/src/filesystem_media/artifact_tree_effects.rs",
    "workspaces/worth-store/crates/worth-store-recovery-physics/Cargo.toml",
    "workspaces/worth-store/crates/worth-store-recovery-physics/src/lib.rs",
    "workspaces/worth-store/crates/worth-store-wal/Cargo.toml",
    "workspaces/worth-store/crates/worth-store-wal/README.md",
    "workspaces/worth-store/crates/worth-store-wal/src/lib.rs",
    "workspaces/worth-store/crates/worth-store/Cargo.toml",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/construction.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/mutation/idempotency/attempt_binding.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/batch.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/submission.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/durable_preparation/prepared.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/work_semantics/mod.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority/physical_wal_append_examples.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority_ui.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger/source_identity.rs",
];
const PHASE_THREE_WAL_SOURCE_TREES: &[&str] = &[
    "workspaces/worth-store/crates/worth-store-recovery-physics/src/wal_recovery_basis",
    "workspaces/worth-store/crates/worth-store-wal/src/append",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/mutation/admission",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/mutation/progression",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/wal",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/executor",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/media_ownership",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/work_semantics/durability",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/work",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/inventory",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/public_api",
];

const PHASE_FOUR_DATA_SOURCE_FILES: &[&str] = &[
    "_docs/worth-store/physical-reconstruction-c7-authority-trace.csv",
    "_docs/worth-store/physical-reconstruction-c7-durable-publication-join.md",
    "_docs/worth-store/physical-reconstruction-c7-public-api.csv",
    "_docs/worth-store/physical-reconstruction-c7-removal-ledger.csv",
    "workspaces/worth-store/crates/worth-store-physical-format/src/binary_format/record_page_lsn.rs",
    "workspaces/worth-store/crates/worth-store-physical-format/src/record_framing/durable_frame.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/executor/wal_barrier.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/scheduler_admission.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/durable_data.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/submission.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/wal_data_planning.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/durable_data_plan.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/residency/candidate_frame_residency/write_evidence.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/residency/candidate_frame_residency/writeback_progression.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/residency/dirty/outcome.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/residency/dirty/writeback/execution.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/work_semantics/durability/wal_barrier_basis.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/work/declaration/wal_barrier_scope.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission/wal_barrier.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority_ui.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/authority_trace.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/contract/destination_ownership.rs",
];
const PHASE_FOUR_DATA_SOURCE_TREES: &[&str] = &[
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/grouping/wal_barrier",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/data",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/mutation/progression",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission/data_durability",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/contract/wal_before_data",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/public_api",
];
const PHASE_FOUR_DATA_COMPILE_ATTACKS: &[&str] = &[
    "physical_data_progression_is_sealed",
    "wal_durable_authority_requires_completed_barrier",
];

const PHASE_FIVE_GROUP_SOURCE_FILES: &[&str] = &[
    "_docs/worth-store/physical-reconstruction-c7-authority-trace.csv",
    "_docs/worth-store/physical-reconstruction-c7-durable-publication-join.md",
    "_docs/worth-store/physical-reconstruction-c7-public-api.csv",
    "_docs/worth-store/physical-reconstruction-c7-removal-ledger.csv",
    "_docs/worth-store/physical-wal-append.md",
    "crates/worth-proof/src/collections/non_empty.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/wal/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/wal/port.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/wal/runtime_owner.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/construction.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/durable_preparation.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/group_wal_planning.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/pre_seal_cancellation.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/director/submission.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/durable_preparation.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority/physical_wal_append_examples.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority_ui.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/authority_trace.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger/phase_five.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger/source_identity.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/contract/group_commit.rs",
];
const PHASE_FIVE_GROUP_SOURCE_TREES: &[&str] = &[
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/grouping",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/mutation/idempotency",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/wal/port",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/record_serving/publication/durable_preparation",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/inventory",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/public_api",
];

const PHASE_SIX_WAL_SOURCE_FILES: &[&str] = &[
    "_docs/worth-store/physical-reconstruction-c7-authority-trace.csv",
    "_docs/worth-store/physical-reconstruction-c7-durable-publication-join.md",
    "_docs/worth-store/physical-reconstruction-c7-public-api.csv",
    "_docs/worth-store/physical-reconstruction-c7-removal-ledger.csv",
    "_docs/worth-store/physical-wal-append.md",
    "workspaces/worth-store/crates/worth-store-wal/src/lib.rs",
    "workspaces/worth-store/crates/worth-store-wal/src/artifact_store/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/admission/wal_policy.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/wal/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/wal/append_declaration.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/wal/append_settlement.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/wal/port.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/wal/port/group.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/wal/runtime_owner.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/work/declaration/wal_append_scope.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/work/execution/command/types.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/work/execution/command/wal.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/work/execution/settlement/classification/wal.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/executor/wal_segment_create.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission/wal_group_continuation.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission/wal_reopen.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission/wal_rotation.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission/independent_wal_oracle.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/authority_trace.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/contract/wal_segment_lifecycle.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/contract/wal_source_syntax.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/inventory/mod.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/inventory/removal_ledger.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/inventory/source_discovery.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/public_api/mod.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/public_api/locked_surfaces.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/public_api/facade_reachability.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger/source_identity.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger/phase_six.rs",
];
const PHASE_SIX_WAL_SOURCE_TREES: &[&str] = &[
    "workspaces/worth-store/crates/worth-store-wal/src/artifact_store/segment_inventory",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/wal/group_reservation",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/wal/inventory",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission/independent_wal_oracle",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/contract/wal_segment_lifecycle",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/contract/wal_source_syntax",
];

pub(super) fn phase_one_source_identity() -> Result<String, String> {
    source_identity(
        "P1 closure",
        PHASE_ONE_SOURCE_FILES,
        PHASE_ONE_SOURCE_TREES,
        &[],
    )
}

pub(super) fn phase_two_authority_source_identity() -> Result<String, String> {
    source_identity(
        "P2 authority",
        PHASE_TWO_AUTHORITY_SOURCE_FILES,
        PHASE_TWO_AUTHORITY_SOURCE_TREES,
        PHASE_TWO_AUTHORITY_COMPILE_ATTACKS,
    )
}

pub(super) fn phase_two_mutation_source_identity() -> Result<String, String> {
    source_identity(
        "P2 mutation",
        PHASE_TWO_MUTATION_SOURCE_FILES,
        PHASE_TWO_MUTATION_SOURCE_TREES,
        PHASE_TWO_MUTATION_COMPILE_ATTACKS,
    )
}

pub(super) fn phase_three_wal_source_identity() -> Result<String, String> {
    source_identity(
        "P3 WAL",
        PHASE_THREE_WAL_SOURCE_FILES,
        PHASE_THREE_WAL_SOURCE_TREES,
        &[],
    )
}

pub(super) fn phase_four_data_source_identity() -> Result<String, String> {
    source_identity(
        "P4 data",
        PHASE_FOUR_DATA_SOURCE_FILES,
        PHASE_FOUR_DATA_SOURCE_TREES,
        PHASE_FOUR_DATA_COMPILE_ATTACKS,
    )
}

pub(super) fn phase_five_group_source_identity() -> Result<String, String> {
    source_identity(
        "P5 group",
        PHASE_FIVE_GROUP_SOURCE_FILES,
        PHASE_FIVE_GROUP_SOURCE_TREES,
        &[],
    )
}

pub(super) fn phase_six_wal_source_identity() -> Result<String, String> {
    source_identity(
        "P6 WAL",
        PHASE_SIX_WAL_SOURCE_FILES,
        PHASE_SIX_WAL_SOURCE_TREES,
        &[],
    )
}

pub(super) fn phase_six_checkpoint_source_identity() -> Result<String, String> {
    phase_six_checkpoint::source_identity()
}

pub(super) fn phase_six_idempotency_source_identity() -> Result<String, String> {
    phase_six_idempotency::source_identity()
}

pub(super) fn phase_six_reclamation_source_identity() -> Result<String, String> {
    phase_six_reclamation::source_identity()
}

pub(super) fn source_identity(
    label: &str,
    files: &[&str],
    trees: &[&str],
    compile_attacks: &[&str],
) -> Result<String, String> {
    let repository = repository_root();
    let mut sources = files
        .iter()
        .map(|path| repository.join(path))
        .collect::<Vec<_>>();
    for tree in trees {
        collect_source_tree(&repository.join(tree), &mut sources)?;
    }
    for attack in compile_attacks {
        let fixture = repository
            .join("workspaces/worth-store/crates/worth-store/tests/physical_runtime_authority")
            .join(attack);
        sources.push(fixture.with_extension("rs"));
        sources.push(fixture.with_extension("stderr"));
    }
    sources.sort();
    sources.dedup();
    let digest = crate::local_source_fingerprint::hash_sources(&repository, &sources)?;
    let short = digest[..6]
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    Ok(format!("{label} {short}"))
}

fn collect_source_tree(directory: &Path, sources: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = std::fs::read_dir(directory)
        .map_err(|error| format!("cannot inspect {}: {error}", directory.display()))?;
    for entry in entries {
        let path = entry
            .map_err(|error| format!("cannot inspect {}: {error}", directory.display()))?
            .path();
        if path.is_dir() {
            collect_source_tree(&path, sources)?;
        } else if path.is_file() {
            sources.push(path);
        }
    }
    Ok(())
}
