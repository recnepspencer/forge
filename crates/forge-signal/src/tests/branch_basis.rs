use forge_proof::{AuthorityWitness, TransitionOutcome};

use crate::facade::*;
use crate::logic::transaction::SignalBranchBasisReadmissionAuthority;

#[test]
fn branch_basis_digest_survives_snapshot_restore_on_same_branch() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();

    let snapshot = runtime.capture_snapshot();
    let expected = runtime.current_branch_basis_artifact();
    let explicit_branch = runtime.current_branch();
    let explicit = match runtime.branch_basis_artifact(explicit_branch.clone()) {
        TransitionOutcome::Success(basis) => basis,
        other => panic!("expected explicit branch basis, got {other:?}"),
    };
    let snapshot_basis = match runtime.snapshot_restore_branch_basis_artifact(
        &snapshot,
        SnapshotRestoreIntent::restore_runtime_truth(),
    ) {
        TransitionOutcome::Success(basis) => basis,
        other => panic!("expected tracked snapshot basis, got {other:?}"),
    };

    runtime
        .restore_snapshot(&snapshot)
        .expect("restoring captured snapshot should succeed");
    let replayed = runtime.current_branch_basis_artifact();

    assert_eq!(
        expected.payload().basis_digest(),
        replayed.payload().basis_digest()
    );
    assert_eq!(
        expected.payload().branch_component_digest(),
        replayed.payload().branch_component_digest()
    );
    assert_eq!(
        expected.payload().snapshot_component_digest(),
        replayed.payload().snapshot_component_digest()
    );
    assert_eq!(
        expected.payload().head_component_digest(),
        replayed.payload().head_component_digest()
    );
    assert_eq!(
        expected.payload().restore_component_digest(),
        replayed.payload().restore_component_digest()
    );
    assert_eq!(
        expected.strong_basis().value(),
        replayed.strong_basis().value()
    );
    assert_eq!(
        expected.payload().basis_digest(),
        explicit.payload().basis_digest()
    );
    assert_eq!(
        expected.strong_basis().value(),
        explicit.strong_basis().value()
    );
    assert_eq!(
        snapshot_basis.payload().branch_component_digest(),
        expected.payload().branch_component_digest()
    );
    assert_eq!(
        snapshot_basis.payload().snapshot_component_digest(),
        expected.payload().snapshot_component_digest()
    );
    assert_eq!(
        snapshot_basis.payload().restore_posture(),
        &SignalBranchRestorePosture::SnapshotRestore {
            snapshot_id: snapshot.snapshot_id(),
            intent: SnapshotRestoreIntent::restore_runtime_truth(),
        }
    );
}

#[test]
fn branch_basis_validation_distinguishes_cross_branch_and_stale_posture_without_side_effects() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();

    let original_basis = runtime.current_branch_basis_artifact();
    let original_branch = runtime.current_branch();
    let original_head = runtime.branch_head_snapshot_id(original_branch.id);
    let feature = runtime
        .create_branch("feature")
        .expect("branch creation should succeed");
    let branch_count_after_feature = runtime.observe().known_branches().len();

    match runtime.validate_branch_basis_artifact(original_basis.clone(), feature.clone()) {
        TransitionOutcome::Denied(SignalBranchBasisDenial::CrossBranchMismatch {
            basis_branch_id,
            expected_branch_id,
        }) => {
            assert_eq!(basis_branch_id, original_branch.id);
            assert_eq!(expected_branch_id, feature.id);
        }
        other => panic!("expected cross-branch denial, got {other:?}"),
    }

    assert_eq!(runtime.current_branch().id, original_branch.id);
    assert_eq!(
        runtime.branch_head_snapshot_id(original_branch.id),
        original_head
    );
    assert_eq!(
        runtime.observe().known_branches().len(),
        branch_count_after_feature
    );

    let _snapshot = runtime.capture_snapshot();
    let refreshed_head = runtime.branch_head_snapshot_id(original_branch.id);
    match runtime.validate_branch_basis_artifact(original_basis.clone(), original_branch.clone()) {
        TransitionOutcome::Stale(stale) => {
            assert_eq!(
                stale.payload().basis_digest(),
                original_basis.payload().basis_digest()
            );
            assert_eq!(
                stale.payload().branch_component_digest(),
                original_basis.payload().branch_component_digest()
            );
        }
        other => panic!("expected stale branch-basis posture, got {other:?}"),
    }

    assert_eq!(runtime.current_branch().id, original_branch.id);
    assert_eq!(
        runtime.branch_head_snapshot_id(original_branch.id),
        refreshed_head
    );
    assert_eq!(
        runtime.observe().known_branches().len(),
        branch_count_after_feature
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .branch_basis_validation_count,
        2
    );
    assert_eq!(runtime.telemetry().transaction.branch_basis_denial_count, 1);
    assert_eq!(runtime.telemetry().transaction.branch_basis_stale_count, 1);
}

#[test]
fn branch_basis_trust_boundary_bridge_and_readmission_preserve_identity_digest() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();

    let snapshot = runtime.capture_snapshot();
    let basis = match runtime.snapshot_restore_branch_basis_artifact(
        &snapshot,
        SnapshotRestoreIntent::restore_runtime_truth(),
    ) {
        TransitionOutcome::Success(basis) => basis,
        other => panic!("expected tracked snapshot basis, got {other:?}"),
    };
    let bridged = bridge_signal_branch_basis_trust_boundary(basis.clone());
    let authority =
        AuthorityWitness::from_authority_marker(SignalBranchBasisReadmissionAuthority::new());

    let readmitted =
        bridged.readmit_with_authority(basis.strong_basis().value().clone(), authority);

    assert_eq!(
        readmitted.payload().basis_digest(),
        basis.payload().basis_digest()
    );
    assert_eq!(
        readmitted.strong_basis().value(),
        basis.strong_basis().value()
    );
}

#[test]
fn branch_basis_snapshot_lane_rejects_untracked_snapshot_without_side_effects() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();

    let mut snapshot = runtime.capture_snapshot();
    snapshot.meta.snapshot_id = SignalSnapshotId(snapshot.snapshot_id().0 + 41);
    let original_branch = runtime.current_branch();
    let original_head = runtime.branch_head_snapshot_id(original_branch.id);
    let original_branch_count = runtime.observe().known_branches().len();

    match runtime.snapshot_restore_branch_basis_artifact(
        &snapshot,
        SnapshotRestoreIntent::restore_runtime_truth(),
    ) {
        TransitionOutcome::Denied(SignalBranchBasisDenial::UntrackedSnapshot {
            branch_id,
            snapshot_id,
        }) => {
            assert_eq!(branch_id, original_branch.id);
            assert_eq!(snapshot_id, snapshot.snapshot_id());
        }
        other => panic!("expected untracked snapshot denial, got {other:?}"),
    }

    assert_eq!(runtime.current_branch().id, original_branch.id);
    assert_eq!(
        runtime.branch_head_snapshot_id(original_branch.id),
        original_head
    );
    assert_eq!(
        runtime.observe().known_branches().len(),
        original_branch_count
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .branch_basis_production_count,
        1
    );
    assert_eq!(runtime.telemetry().transaction.branch_basis_denial_count, 1);
}
