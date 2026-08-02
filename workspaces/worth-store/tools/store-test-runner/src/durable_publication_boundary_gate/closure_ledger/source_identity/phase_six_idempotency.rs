const SOURCE_FILES: &[&str] = &[
    "_docs/worth-store/physical-reconstruction-c7-authority-trace.csv",
    "_docs/worth-store/physical-reconstruction-c7-durable-publication-join.md",
    "_docs/worth-store/physical-reconstruction-c7-phase-6-checkpoint-plan.md",
    "_docs/worth-store/physical-reconstruction-c7-public-api.csv",
    "_docs/worth-store/physical-reconstruction-c7-removal-ledger.csv",
    "workspaces/worth-store/crates/worth-store-physical-format/src/lib.rs",
    "workspaces/worth-store/crates/worth-store-wal/src/artifact_store/mod.rs",
    "workspaces/worth-store/crates/worth-store-wal/src/lib.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/admission/platform_basis_join.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/admission/policy.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/checkpoint/publication.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/mod.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/observation/policy.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/construction.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/durability_bootstrap.rs",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/mod.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission/checkpoint_capture.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission/idempotency_reopen.rs",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/authority_trace.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger/phase_six.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger/source_identity.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger/source_identity/phase_six_idempotency.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/contract/idempotency_reopen.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/inventory/mod.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/inventory/removal_ledger.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/inventory/source_discovery.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/public_api/facade_reachability.rs",
    "workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/public_api/locked_surfaces.rs",
];

const SOURCE_TREES: &[&str] = &[
    "workspaces/worth-store/crates/worth-store-physical-format/src/checkpoint",
    "workspaces/worth-store/crates/worth-store-wal/src/artifact_store/segment_inventory",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/checkpoint/reopen",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/mutation/idempotency",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/durability/wal/inventory",
    "workspaces/worth-store/crates/worth-store/src/physical_runtime/instance/construction",
    "workspaces/worth-store/crates/worth-store/tests/physical_record_journeys/durability_admission/independent_wal_oracle",
];

pub(super) fn source_identity() -> Result<String, String> {
    super::source_identity("P6 idempotency", SOURCE_FILES, SOURCE_TREES, &[])
}
