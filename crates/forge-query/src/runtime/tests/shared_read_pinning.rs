mod hostile_matrix;
mod hot_path_lock_posture;
mod inventory;
mod pin_retire_residue;
mod published_artifact_retention;
mod sabotage;
mod send_sync_portability;
mod shared_read_context_boundary;
mod stale_basis_denial;

use super::shared_read_support::*;
use crate::application::{
    scan_shared_read_pin_hot_path_forbidden_patterns,
    scan_shared_read_pin_required_pattern_failures, scan_shared_read_pin_retire_forbidden_patterns,
    shared_read_pinning_operation_inventory, ForgeQuerySharedReadPinningCertification,
    ForgeQuerySharedReadPinningCounterEvidence, ForgeQuerySharedReadPinningHostileMatrixEvidence,
    ForgeQuerySharedReadPinningInventoryEvidence, ForgeQuerySharedReadPinningOperationKind,
    ForgeQuerySharedReadPortabilityEvidence, ForgeQuerySharedReadStaleBasisDenialEvidence,
};

fn shared_read_pinning_workspace(
    name: &str,
) -> (
    crate::runtime::ForgeQueryWorkspace,
    crate::runtime::ForgeQueryDerivedViewHandle<crate::runtime::ForgeQueryNativeRow>,
) {
    let mut workspace = shared_read_workspace(name);
    let derived = declare_shared_read_derived(
        &mut workspace,
        name,
        SharedReadPublishingMaintainer {
            invocations: std::sync::Arc::new(std::sync::atomic::AtomicUsize::new(0)),
            mode: SharedReadPublicationMode::SequencedRefresh(&["Task One", "Task Two"]),
        },
    );
    insert_task(&mut workspace, "task-1", "Task One");
    (workspace, derived)
}

fn workspace_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("workspace root should resolve")
}

fn shared_read_pinning_inventory_evidence() -> ForgeQuerySharedReadPinningInventoryEvidence {
    let workspace_root = workspace_root();
    let hot_path_failures = scan_shared_read_pin_hot_path_forbidden_patterns(&workspace_root);
    let retire_failures = scan_shared_read_pin_retire_forbidden_patterns(&workspace_root);
    let missing_required_patterns = scan_shared_read_pin_required_pattern_failures(&workspace_root);
    let operation_missing_count = shared_read_pinning_missing_operation_count();
    let scan_failure_count =
        hot_path_failures.len() + retire_failures.len() + missing_required_patterns.len();
    let inventory_digest = crate::evidence_identity::forge_query_evidence_identity(
        crate::evidence_identity::ForgeQueryEvidenceScope::ApplicationSupportReport,
    )
    .field_usize(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("scan_failure_count"),
        scan_failure_count,
    )
    .field_usize(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("missing_operation_count"),
        operation_missing_count,
    )
    .field_value_sequence(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("operation_path"),
        shared_read_pinning_operation_inventory()
            .iter()
            .map(|row| row.path()),
    )
    .field_value_sequence(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("operation_function"),
        shared_read_pinning_operation_inventory()
            .iter()
            .map(|row| row.function()),
    )
    .seal()
    .as_str()
    .to_string();
    ForgeQuerySharedReadPinningInventoryEvidence::new(
        scan_failure_count,
        operation_missing_count,
        inventory_digest,
    )
}

fn shared_read_pinning_missing_operation_count() -> usize {
    let operations = shared_read_pinning_operation_inventory()
        .iter()
        .map(|row| row.kind())
        .collect::<std::collections::BTreeSet<_>>();
    required_shared_read_pinning_operations()
        .iter()
        .filter(|operation| !operations.contains(operation))
        .count()
}

fn required_shared_read_pinning_operations() -> &'static [ForgeQuerySharedReadPinningOperationKind]
{
    &[
        ForgeQuerySharedReadPinningOperationKind::PinCurrentGeneration,
        ForgeQuerySharedReadPinningOperationKind::ReleaseGeneration,
        ForgeQuerySharedReadPinningOperationKind::DrainRetiredGeneration,
        ForgeQuerySharedReadPinningOperationKind::CaptureCommittedGeneration,
        ForgeQuerySharedReadPinningOperationKind::RetainPublishedArtifactGenerations,
        ForgeQuerySharedReadPinningOperationKind::ResolvePublishedArtifactGeneration,
        ForgeQuerySharedReadPinningOperationKind::MeasureCommittedReadHotPath,
        ForgeQuerySharedReadPinningOperationKind::MintSharedReadContext,
        ForgeQuerySharedReadPinningOperationKind::InspectSharedReadBasis,
        ForgeQuerySharedReadPinningOperationKind::ConsumePublishedArtifact,
        ForgeQuerySharedReadPinningOperationKind::ClassifyPinningBoundaryClosure,
    ]
}

fn shared_read_counter_evidence(
    runtime: &crate::runtime::ForgeQueryRuntime,
) -> ForgeQuerySharedReadPinningCounterEvidence {
    let counters = runtime.shared_read_counters();
    ForgeQuerySharedReadPinningCounterEvidence::new(
        counters.committed_read_hot_path_lock_count(),
        counters.orphaned_generation_count(),
        counters.unretired_pin_count(),
        counters.shared_read_mint_row_clone_count(),
        counters.reader_derived_evaluation_count(),
    )
}

fn pinning_phase_twelve_counters_are_closed(runtime: &crate::runtime::ForgeQueryRuntime) -> bool {
    let pinning = runtime.shared_read_pinning_diagnostics();
    let published = runtime.published_artifact_diagnostics();
    let counters = pinning
        .counters()
        .with_published_artifacts(published.counters());
    counters.committed_read_hot_path_lock_count() == 0
        && counters.orphaned_generation_count() == 0
        && counters.unretired_pin_count() == 0
        && pinning.retired_pinned_generation_count() == 0
        && published.retained_generation_count() <= pinning.retained_generation_count()
}

fn evidence_digest(
    label: &str,
    fields: impl IntoIterator<Item = (&'static str, String)>,
) -> String {
    let mut builder = crate::evidence_identity::forge_query_evidence_identity(
        crate::evidence_identity::ForgeQueryEvidenceScope::RuntimeHostileCertificationArtifact,
    )
    .field_shape(
        crate::evidence_identity::ForgeQueryEvidenceTag::new("evidence_label"),
        label,
    );
    for (tag, value) in fields {
        builder = builder.field_shape(
            crate::evidence_identity::ForgeQueryEvidenceTag::new(tag),
            value,
        );
    }
    builder.seal().as_str().to_string()
}

fn generation_ordinal(
    runtime: &crate::runtime::ForgeQueryRuntime,
    snapshot: &crate::memory_workspace::ForgeQuerySnapshotIdentity,
) -> u64 {
    runtime
        .shared_read_pinning_diagnostics()
        .generations()
        .iter()
        .find(|generation| generation.snapshot_identity() == snapshot)
        .expect("snapshot generation should be retained")
        .ordinal()
}
