use super::runtime;
use crate::facade::{
    BridgePreviewLifecycleStateKind, BridgePreviewResidueClass, BridgePreviewSessionDeclaration,
    BridgePreviewSessionDeclarationIdentity, BridgePreviewSessionIdentity, BridgeRequestKind,
    BridgeRuntimePolicy, BridgeSignalBranchIdentity, BridgeSpeculativeBranchBinding,
    BridgeSpeculativeBranchBindingIdentity, TruthBranchIdentity,
};

fn preview_declaration() -> BridgePreviewSessionDeclaration {
    BridgePreviewSessionDeclaration::new(
        BridgePreviewSessionDeclarationIdentity::new("preview:analysis"),
        BridgeRequestKind::Preview,
        BridgeSpeculativeBranchBinding::new(
            BridgeSpeculativeBranchBindingIdentity::new("binding:analysis"),
            TruthBranchIdentity::new("truth:analysis"),
            BridgeSignalBranchIdentity::new("signal:analysis"),
        ),
        "truth-view:analysis",
        "source-capability:analysis",
        "request-shape:analysis",
        "artifact-schema:analysis",
    )
}

#[test]
fn runtime_activates_and_discards_preview_session_with_zero_authoritative_residue() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:analysis"),
            preview_declaration(),
        )
        .expect("preview declaration should admit");

    let (active, execution_record) = runtime.activate_preview_session(admitted, 3, 1, 2);
    assert_eq!(execution_record.counters().preview_artifact_count(), 3);

    let (discarded, discard_record) = runtime
        .discard_preview_session(
            active,
            &execution_record,
            vec![
                BridgePreviewResidueClass::PreviewExecutionRetained,
                BridgePreviewResidueClass::ReplayRetainedNonAuthoritative,
                BridgePreviewResidueClass::TemporaryDiagnosticsResidue,
            ],
        )
        .expect("preview discard should succeed");

    assert_eq!(
        discarded.lifecycle_state_kind(),
        BridgePreviewLifecycleStateKind::Discarded
    );
    assert_eq!(
        discard_record
            .residue_report()
            .authoritative_residue_count(),
        0
    );
    assert_eq!(discard_record.counters().destroyed_artifact_count(), 1);
    assert_eq!(runtime.diagnostics().preview_execution_records().len(), 1);
    assert_eq!(runtime.diagnostics().preview_discard_records().len(), 1);
    assert_eq!(
        runtime
            .diagnostics()
            .preview_discard_record_for_session_identity(discarded.session_identity().as_str())
            .expect("discard diagnostics should be retained")
            .record_identity(),
        discard_record.record_identity()
    );
}

#[test]
fn runtime_rejects_preview_discard_when_authoritative_residue_remains() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:authority"),
            preview_declaration(),
        )
        .expect("preview declaration should admit");
    let (active, execution_record) = runtime.activate_preview_session(admitted, 2, 1, 1);

    let error = runtime
        .discard_preview_session(
            active,
            &execution_record,
            vec![BridgePreviewResidueClass::AuthoritativeRoutingResidue],
        )
        .expect_err("authoritative residue must block discard");

    assert_eq!(
        error.kind(),
        crate::error::BridgeSpeculationErrorKind::PreviewResidueClassificationMismatch
    );
}

#[test]
fn runtime_admits_and_activates_reused_preview_session_only_for_exact_equivalence() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let source_admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:reuse-source"),
            preview_declaration(),
        )
        .expect("source preview declaration should admit");
    let (source_active, source_execution_record) =
        runtime.activate_preview_session(source_admitted, 4, 2, 2);

    let target_admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:reuse-target"),
            preview_declaration(),
        )
        .expect("target preview declaration should admit");
    let reuse_equivalence = runtime
        .admit_preview_reuse(&source_active, &source_execution_record, &target_admitted)
        .expect("exactly equivalent preview declarations should admit reuse");

    let (target_active, target_execution_record) = runtime
        .activate_preview_session_with_reuse(
            target_admitted,
            &source_active,
            &source_execution_record,
            &reuse_equivalence,
        )
        .expect("reused preview activation should succeed");

    assert_eq!(
        target_active.lifecycle_state_kind(),
        BridgePreviewLifecycleStateKind::Active
    );
    assert_eq!(
        target_execution_record.counters().preview_artifact_count(),
        source_execution_record.counters().preview_artifact_count()
    );
    assert_eq!(runtime.diagnostics().preview_execution_records().len(), 2);
}

#[test]
fn runtime_rejects_preview_reuse_when_target_basis_drifts() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let source_admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:reuse-source-drift"),
            preview_declaration(),
        )
        .expect("source preview declaration should admit");
    let (source_active, source_execution_record) =
        runtime.activate_preview_session(source_admitted, 2, 1, 1);

    let drifted_target = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:reuse-target-drift"),
            preview_declaration().with_structural_basis_digest("structural:drift"),
        )
        .expect("drifted preview declaration should still admit");
    let error = runtime
        .admit_preview_reuse(&source_active, &source_execution_record, &drifted_target)
        .expect_err("drifted declarations must fail reuse admission");

    assert_eq!(
        error.kind(),
        crate::error::BridgeSpeculationErrorKind::PreviewReuseEquivalenceMismatch
    );
}

#[test]
fn runtime_promotes_preview_session_and_replays_promoted_bundle() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:promotion"),
            preview_declaration(),
        )
        .expect("preview declaration should admit");
    let (active, execution_record) = runtime.activate_preview_session(admitted, 3, 1, 2);
    let proof = active.promotion_admissibility_proof();

    let (promoted, promotion_record) = runtime
        .promote_preview_session(
            active,
            &execution_record,
            &proof,
            "commit-boundary:promotion",
            "authoritative-artifact:promotion",
        )
        .expect("promotion should succeed");
    let replay_bundle = runtime
        .replay_preview_bundle(promoted.session_identity().as_str())
        .expect("retained promotion records should replay");

    assert_eq!(
        promoted.lifecycle_state_kind(),
        BridgePreviewLifecycleStateKind::Promoted
    );
    assert_eq!(runtime.diagnostics().preview_promotion_records().len(), 1);
    assert_eq!(
        replay_bundle.lifecycle_outcome(),
        BridgePreviewLifecycleStateKind::Promoted
    );
    assert_eq!(
        replay_bundle
            .preview_promotion_record()
            .expect("promotion replay should contain terminal record")
            .record_identity(),
        promotion_record.record_identity()
    );
}

#[test]
fn runtime_rejects_stale_duplicate_and_post_discard_promotion() {
    let runtime = runtime(BridgeRuntimePolicy::default());

    let admitted_a = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:promotion-a"),
            preview_declaration(),
        )
        .expect("first preview declaration should admit");
    let (active_a, execution_record_a) = runtime.activate_preview_session(admitted_a, 2, 1, 1);
    let stale_proof = active_a.promotion_admissibility_proof();

    let admitted_b = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:promotion-b"),
            preview_declaration(),
        )
        .expect("second preview declaration should admit");
    let (active_b_stale, execution_record_b_stale) =
        runtime.activate_preview_session(admitted_b, 2, 1, 1);

    let stale_error = runtime
        .promote_preview_session(
            active_b_stale,
            &execution_record_b_stale,
            &stale_proof,
            "commit-boundary:stale",
            "authoritative-artifact:stale",
        )
        .expect_err("stale proof must fail promotion");
    assert_eq!(
        stale_error.kind(),
        crate::error::BridgeSpeculationErrorKind::PromotionAdmissibilityMismatch
    );

    let duplicate_admission_error = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:promotion-b"),
            preview_declaration(),
        )
        .expect_err("duplicate preview session identity must fail at admission");
    assert_eq!(
        duplicate_admission_error.kind(),
        crate::error::BridgeSpeculationErrorKind::PreviewSessionIdentityConflict
    );

    let (_discarded, _discard_record) = runtime
        .discard_preview_session(
            active_a,
            &execution_record_a,
            vec![
                BridgePreviewResidueClass::PreviewExecutionRetained,
                BridgePreviewResidueClass::TemporaryRoutingResidue,
            ],
        )
        .expect("discard should succeed");
    let reactivated_admission_error = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:promotion-a"),
            preview_declaration(),
        )
        .expect_err("discarded preview session identity must remain reserved");
    assert_eq!(
        reactivated_admission_error.kind(),
        crate::error::BridgeSpeculationErrorKind::PreviewSessionIdentityConflict
    );
}

#[test]
fn runtime_replays_discarded_preview_bundle_from_retained_records() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:replay-discard"),
            preview_declaration(),
        )
        .expect("preview declaration should admit");
    let (active, execution_record) = runtime.activate_preview_session(admitted, 3, 2, 1);
    let (discarded, discard_record) = runtime
        .discard_preview_session(
            active,
            &execution_record,
            vec![
                BridgePreviewResidueClass::PreviewDiagnosticsRetained,
                BridgePreviewResidueClass::TemporaryDiagnosticsResidue,
            ],
        )
        .expect("discard should succeed");

    let replay_bundle = runtime
        .replay_preview_bundle(discarded.session_identity().as_str())
        .expect("discarded preview should replay from retained records");

    assert_eq!(
        replay_bundle.lifecycle_outcome(),
        BridgePreviewLifecycleStateKind::Discarded
    );
    assert_eq!(
        replay_bundle
            .preview_discard_record()
            .expect("discard replay should contain discard record")
            .record_identity(),
        discard_record.record_identity()
    );
    assert_eq!(replay_bundle.counters().replay_bundle_width(), 2);
}

#[test]
fn runtime_rejects_post_discard_reentry_and_preserves_canonical_discard_bundle() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:discard-terminal"),
            preview_declaration(),
        )
        .expect("preview declaration should admit");
    let (active, execution_record) = runtime.activate_preview_session(admitted, 4, 2, 2);
    let (discarded, discard_record) = runtime
        .discard_preview_session(
            active,
            &execution_record,
            vec![
                BridgePreviewResidueClass::PreviewExecutionRetained,
                BridgePreviewResidueClass::ReplayRetainedNonAuthoritative,
                BridgePreviewResidueClass::TemporaryDiagnosticsResidue,
            ],
        )
        .expect("discard should succeed");

    let initial_replay = runtime
        .replay_preview_bundle(discarded.session_identity().as_str())
        .expect("discarded preview should replay");
    assert_eq!(
        initial_replay
            .preview_discard_record()
            .expect("discard replay should retain discard record")
            .record_identity(),
        discard_record.record_identity()
    );

    let hostile_reentry_error = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:discard-terminal"),
            preview_declaration().with_structural_basis_digest("structural:hostile-reentry"),
        )
        .expect_err("discarded preview session identity must reject hostile re-entry");
    assert_eq!(
        hostile_reentry_error.kind(),
        crate::error::BridgeSpeculationErrorKind::PreviewSessionIdentityConflict
    );

    let replay_after_reentry_attempt = runtime
        .replay_preview_bundle(discarded.session_identity().as_str())
        .expect("discard replay should remain canonical after rejected re-entry");

    assert_eq!(
        replay_after_reentry_attempt.lifecycle_outcome(),
        BridgePreviewLifecycleStateKind::Discarded
    );
    assert_eq!(
        replay_after_reentry_attempt
            .preview_discard_record()
            .expect("discard replay should still contain the original terminal record")
            .record_identity(),
        discard_record.record_identity()
    );
    assert!(
        replay_after_reentry_attempt
            .preview_promotion_record()
            .is_none(),
        "discarded sessions must not acquire promotion residue through hostile re-entry"
    );
    assert_eq!(runtime.diagnostics().preview_execution_records().len(), 1);
    assert_eq!(runtime.diagnostics().preview_discard_records().len(), 1);
    assert_eq!(runtime.diagnostics().preview_promotion_records().len(), 0);
}

#[test]
fn runtime_explains_preview_promotion_and_replay_from_retained_records() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:explain-promotion"),
            preview_declaration(),
        )
        .expect("preview declaration should admit");
    let (active, execution_record) = runtime.activate_preview_session(admitted, 5, 1, 2);
    let proof = active.promotion_admissibility_proof();
    let (_promoted, promotion_record) = runtime
        .promote_preview_session(
            active,
            &execution_record,
            &proof,
            "commit-boundary:explain",
            "authoritative-artifact:explain",
        )
        .expect("promotion should succeed");

    let execution_explanation = runtime
        .diagnostics()
        .explain_last_preview_execution_record()
        .expect("execution explanation should exist");
    let promotion_explanation = runtime
        .diagnostics()
        .explain_last_preview_promotion_record()
        .expect("promotion explanation should exist");
    let replay_bundle = runtime
        .replay_preview_bundle("preview-session:explain-promotion")
        .expect("promotion replay bundle should exist");
    let replay_explanation = runtime
        .diagnostics()
        .explain_preview_replay_bundle(&replay_bundle);

    assert_eq!(
        execution_explanation.preview_execution_record_identity(),
        execution_record.record_identity().as_str()
    );
    assert_eq!(
        promotion_explanation.preview_promotion_record_identity(),
        promotion_record.record_identity().as_str()
    );
    assert_eq!(
        replay_explanation.lifecycle_outcome(),
        BridgePreviewLifecycleStateKind::Promoted
    );
    assert!(replay_explanation.has_promotion_record());
}

#[test]
fn runtime_explains_preview_discard_from_retained_records() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("preview-session:explain-discard"),
            preview_declaration(),
        )
        .expect("preview declaration should admit");
    let (active, execution_record) = runtime.activate_preview_session(admitted, 2, 1, 1);
    let (_discarded, discard_record) = runtime
        .discard_preview_session(
            active,
            &execution_record,
            vec![
                BridgePreviewResidueClass::PreviewDiagnosticsRetained,
                BridgePreviewResidueClass::TemporaryDiagnosticsResidue,
            ],
        )
        .expect("discard should succeed");

    let discard_explanation = runtime
        .diagnostics()
        .explain_last_preview_discard_record()
        .expect("discard explanation should exist");

    assert_eq!(
        discard_explanation.preview_discard_record_identity(),
        discard_record.record_identity().as_str()
    );
    assert_eq!(discard_explanation.authoritative_residue_count(), 0);
    assert_eq!(discard_explanation.destroyed_artifact_count(), 1);
}
