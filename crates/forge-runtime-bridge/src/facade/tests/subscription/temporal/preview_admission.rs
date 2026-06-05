use super::super::support::*;

#[test]
fn runtime_admits_preview_temporal_subscription_for_matching_preview_scope() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let admitted = admitted_detail_subscription_in_runtime(&runtime);
    let preview_basis = admitted_preview_basis_for_truth(
        &runtime,
        "preview-temporal-match",
        TruthBranchIdentity::new("analysis"),
        TruthSnapshotIdentity::new("snapshot-a"),
    );
    let temporal_basis = admitted_temporal_basis(BridgeTemporalTruthViewBasis::authoritative(
        TruthBranchIdentity::new("analysis"),
        TruthCommitIdentity::new("commit-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
    ));

    let temporal = runtime
        .admit_preview_temporal_subscription(
            &admitted,
            &preview_basis,
            temporal_basis,
            BridgeTemporalSubscriptionFamilyKind::WakeDriven,
        )
        .expect("preview temporal subscription should admit");
    let ready = runtime.prepare_preview_temporal_subscription_activation(&temporal);

    assert_eq!(
        temporal.family().kind(),
        BridgeTemporalSubscriptionFamilyKind::WakeDriven
    );
    assert_eq!(
        temporal.preview_basis().preview_basis_identity(),
        preview_basis.preview_basis_identity()
    );
    assert_eq!(
        temporal.counters().subscription_temporal_admission_count(),
        1
    );
    assert_eq!(
        ready
            .preview_temporal_admission()
            .preview_temporal_admission_identity(),
        temporal.preview_temporal_admission_identity()
    );
    assert_eq!(
        ready
            .counters()
            .subscription_temporal_activation_ready_count(),
        1
    );
}

#[test]
fn runtime_rejects_preview_temporal_subscription_when_preview_branch_drifts() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let admitted = admitted_detail_subscription_in_runtime(&runtime);
    let preview_basis = admitted_preview_basis_for_truth(
        &runtime,
        "preview-temporal-branch-drift",
        TruthBranchIdentity::new("preview-branch"),
        TruthSnapshotIdentity::new("snapshot-a"),
    );
    let temporal_basis = admitted_temporal_basis(BridgeTemporalTruthViewBasis::authoritative(
        TruthBranchIdentity::new("analysis"),
        TruthCommitIdentity::new("commit-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
    ));

    let rejection = runtime
        .admit_preview_temporal_subscription(
            &admitted,
            &preview_basis,
            temporal_basis,
            BridgeTemporalSubscriptionFamilyKind::WakeDriven,
        )
        .expect_err("preview branch drift should reject temporal admission");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgePreviewTemporalSubscriptionAdmissionRejectionKind::PreviewBranchIdentityMismatch
    );
    assert_eq!(
        rejection.counters().subscription_temporal_rejection_count(),
        1
    );
}

#[test]
fn runtime_rejects_preview_temporal_subscription_when_preview_snapshot_drifts() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let admitted = admitted_detail_subscription_in_runtime(&runtime);
    let preview_basis = admitted_preview_basis_for_truth(
        &runtime,
        "preview-temporal-snapshot-drift",
        TruthBranchIdentity::new("analysis"),
        TruthSnapshotIdentity::new("snapshot-preview"),
    );
    let temporal_basis = admitted_temporal_basis(BridgeTemporalTruthViewBasis::authoritative(
        TruthBranchIdentity::new("analysis"),
        TruthCommitIdentity::new("commit-a"),
        TruthSnapshotIdentity::new("snapshot-a"),
    ));

    let rejection = runtime
        .admit_preview_temporal_subscription(
            &admitted,
            &preview_basis,
            temporal_basis,
            BridgeTemporalSubscriptionFamilyKind::WakeDriven,
        )
        .expect_err("preview snapshot drift should reject temporal admission");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgePreviewTemporalSubscriptionAdmissionRejectionKind::PreviewSnapshotIdentityMismatch
    );
}

#[test]
fn runtime_rejects_preview_temporal_subscription_when_family_does_not_support_basis_kind() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let admitted = admitted_detail_subscription_in_runtime(&runtime);
    let preview_basis = admitted_preview_basis_for_truth(
        &runtime,
        "preview-temporal-family-mismatch",
        TruthBranchIdentity::new("analysis"),
        TruthSnapshotIdentity::new("snapshot-a"),
    );
    let temporal_basis = admitted_temporal_basis(BridgeTemporalTruthViewBasis::historical(
        TruthBranchIdentity::new("analysis"),
        TruthCommitIdentity::new("commit-historical"),
        TruthSnapshotIdentity::new("snapshot-a"),
    ));

    let rejection = runtime
        .admit_preview_temporal_subscription(
            &admitted,
            &preview_basis,
            temporal_basis,
            BridgeTemporalSubscriptionFamilyKind::WakeDriven,
        )
        .expect_err("wake-driven preview temporal admission should reject historical basis");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgePreviewTemporalSubscriptionAdmissionRejectionKind::TemporalFamilyDoesNotSupportBasisKind
    );
    assert_eq!(
        rejection.counters().subscription_temporal_rejection_count(),
        1
    );
}
