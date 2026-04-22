use super::support::*;

#[test]
fn preview_subscription_promotion_emits_authoritative_boundary_record() {
    let (runtime, preview_active, promotion_record, promoted_ready) =
        preview_promotion_detail_subscription("subscription-promote");
    let preview_identity = preview_active
        .preview_active_subscription_identity()
        .clone();
    let preview_basis = preview_active.preview_basis_identity().clone();
    let promoted_identity = promoted_ready
        .admitted()
        .admitted_subscription_identity()
        .clone();
    let work_trace = preview_work_trace(&runtime, &preview_active, "subscription-promote");

    let record = runtime
        .promote_preview_subscription(
            preview_active,
            &work_trace,
            &promotion_record,
            &promoted_ready,
        )
        .expect("matching preview promotion should emit a subscription boundary record");

    assert_eq!(
        record.outcome_class(),
        crate::facade::BridgeSubscriptionPreviewPromotionOutcomeClass::PromotedAuthoritativeBoundary
    );
    assert_eq!(
        record.preview_active_subscription_identity(),
        &preview_identity
    );
    assert_eq!(record.preview_basis_identity(), &preview_basis);
    assert_eq!(
        record.promoted_admitted_subscription_identity(),
        &promoted_identity
    );
    assert_eq!(
        record.speculation_promotion_record_digest(),
        promotion_record.digest()
    );
    assert_eq!(
        record.preview_work_trace_identity(),
        work_trace.preview_work_trace_identity()
    );
    assert_eq!(record.preview_work_trace_digest(), work_trace.digest());
    assert_eq!(
        record.authoritative_commit_boundary_digest(),
        promotion_record.authoritative_commit_boundary_digest()
    );
    assert_eq!(
        record.authoritative_artifact_digest(),
        promotion_record.authoritative_artifact_digest()
    );
    assert_eq!(record.counters().subscription_preview_promotion_count(), 1);
    assert_eq!(
        record
            .counters()
            .subscription_rich_diagnostics_hot_path_materialization_count(),
        0
    );
    assert_ne!(
        record.preview_active_subscription_identity().as_str(),
        record.promoted_admitted_subscription_identity().as_str()
    );
}

#[test]
fn preview_subscription_promotion_rejects_mismatched_promotion_record() {
    let (runtime, preview_active, _promotion_record, promoted_ready) =
        preview_promotion_detail_subscription("subscription-promote-mismatch-a");
    let (_other_runtime, _other_preview_active, other_promotion_record, _other_promoted_ready) =
        preview_promotion_detail_subscription("subscription-promote-mismatch-b");
    let work_trace =
        preview_work_trace(&runtime, &preview_active, "subscription-promote-mismatch-a");

    let rejection = runtime
        .promote_preview_subscription(
            preview_active,
            &work_trace,
            &other_promotion_record,
            &promoted_ready,
        )
        .expect_err("promotion records from another preview session must reject");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionPreviewPromotionRejectionKind::PromotionSessionMismatch
    );
    assert_eq!(
        rejection
            .counters()
            .subscription_preview_promotion_rejection_count(),
        1
    );
}

#[test]
fn preview_subscription_promotion_rejects_promoted_subscription_drift() {
    let (runtime, preview_active, promotion_record, _promoted_ready) =
        preview_promotion_detail_subscription("subscription-promote-drift");
    let work_trace = preview_work_trace(&runtime, &preview_active, "subscription-promote-drift");
    let drift_declaration = runtime
        .declare_subscription(
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            vec![NormalizedSubscriptionSliceIntent::try_new(
                "entity-drift",
                "profile",
                "name",
                SubscriptionSliceKind::SignalField,
            )
            .expect("slice intent should validate")],
            BridgeSubscriptionDeliveryIntentClass::None,
        )
        .expect("drift declaration should admit");
    let drift_admitted = runtime
        .admit_subscription(
            &drift_declaration,
            BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new("snapshot-a")),
        )
        .expect("drift admission should succeed");
    let drift_ready = runtime.prepare_subscription_activation(&drift_admitted);

    let rejection = runtime
        .promote_preview_subscription(preview_active, &work_trace, &promotion_record, &drift_ready)
        .expect_err("promotion must bind the matching promoted authoritative subscription");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionPreviewPromotionRejectionKind::PromotedSubscriptionMismatch
    );
}

#[test]
fn preview_subscription_promotion_rejects_preview_work_trace_drift() {
    let (runtime, preview_active, promotion_record, promoted_ready) =
        preview_promotion_detail_subscription("subscription-promote-work-trace-a");
    let (other_runtime, other_preview_active, _other_promotion_record, _other_promoted_ready) =
        preview_promotion_detail_subscription("subscription-promote-work-trace-b");
    let other_work_trace = preview_work_trace(
        &other_runtime,
        &other_preview_active,
        "subscription-promote-work-trace-b",
    );

    let rejection = runtime
        .promote_preview_subscription(
            preview_active,
            &other_work_trace,
            &promotion_record,
            &promoted_ready,
        )
        .expect_err("promotion must reject preview work from another scope");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionPreviewPromotionRejectionKind::PreviewWorkTraceMismatch
    );
}
