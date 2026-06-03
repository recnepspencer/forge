use super::{
    registered_source, runtime, BridgeMappingId, BridgeMappingRegistration, BridgeRuntimePolicy,
    BridgeTruthViewSelector, CoarseRoutingMode, MappingSelector, RuntimeBridge,
    SignalInvalidationScope, StaticSink, StaticSource, StaticSourceAdapter, TruthBranchIdentity,
    TruthPatchScope, TruthSnapshotIdentity,
};
use crate::facade::BridgeSpeculativeSessionRequest;
use crate::speculation::{
    BridgePreviewResidueClass, BridgePreviewRetainedArtifactSchema, BridgePreviewSessionBasis,
    BridgePreviewSessionDeclaration, BridgePreviewSessionDeclarationIdentity,
    BridgePreviewSessionIdentity, BridgeRequestKind, BridgeSignalBranchIdentity,
    BridgeSpeculativeBranchBinding, BridgeSpeculativeBranchBindingIdentity,
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
        BridgePreviewSessionBasis::new(
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("truth:analysis"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            crate::facade::BridgeSourceCapabilitySet::new(vec![
                crate::facade::BridgeSourceCapability::SnapshotRead,
                crate::facade::BridgeSourceCapability::BranchRead,
            ]),
            BridgePreviewRetainedArtifactSchema::PreviewLifecycleArtifactsV1,
        ),
    )
}

#[test]
fn standard_builder_aliases_build_runtime() {
    let runtime = RuntimeBridge::builder()
        .with_policy(BridgeRuntimePolicy::default())
        .with_truth_source(StaticSource)
        .with_source_adapter(StaticSourceAdapter)
        .with_truth_branch_head_source(StaticSource)
        .with_compute_sink(StaticSink)
        .register_source(registered_source(
            "source:analysis-snapshot",
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("analysis"),
                TruthSnapshotIdentity::new("snapshot-a"),
            ),
            vec![
                crate::facade::BridgeSourceCapability::SnapshotRead,
                crate::facade::BridgeSourceCapability::BranchRead,
            ],
        ))
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("mapping"),
            TruthPatchScope::for_entity_field(
                MappingSelector::exact("entity-1"),
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid native aspect key"),
                forge_foundational::facade::FieldKey::new("name".to_owned())
                    .expect("valid native field key"),
            ),
            crate::snapshot::SnapshotReadContract::scalar(
                forge_foundational::facade::AspectKey::new("profile")
                    .expect("valid native aspect key"),
                forge_foundational::facade::ScalarAspectType::String,
            ),
            SignalInvalidationScope::new("signal:profile"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("builder aliases should produce a valid runtime");

    assert_eq!(runtime.policy(), &BridgeRuntimePolicy::default());
}

#[test]
fn standard_route_flows_from_commit_string_to_evaluation_target() {
    let runtime = runtime(BridgeRuntimePolicy::default());

    let routed = runtime
        .route(crate::facade::TruthCommitIdentity::new("commit-std"))
        .expect("standard route should succeed");
    let evaluation = runtime
        .evaluate_current(routed.target())
        .expect("evaluation target should prepare current evaluation");

    assert_eq!(
        routed.result().result_summary().source_commit().as_str(),
        "commit-std"
    );
    assert_eq!(
        routed.result().receipt().snapshot_identity().as_str(),
        "snapshot-a"
    );
    assert_eq!(
        evaluation.snapshot().snapshot_identity().as_str(),
        "snapshot-a"
    );
    assert_eq!(runtime.diagnostics().route_records().len(), 1);
    assert!(matches!(
        runtime.diagnostics().explain_last(),
        Some(crate::facade::BridgeStandardDiagnosticsExplanation::Route(
            _
        ))
    ));
}

#[test]
fn standard_truth_view_evaluation_flows_from_branch_head_request() {
    let runtime = runtime(BridgeRuntimePolicy::default());

    let evaluation = runtime
        .evaluate(
            crate::facade::BridgeTruthViewEvaluationRequest::for_branch_head(
                TruthBranchIdentity::new("analysis"),
            ),
        )
        .expect("branch-head evaluation should succeed");

    assert_eq!(evaluation.snapshot_identity().as_str(), "snapshot-a");
    assert_eq!(
        evaluation
            .record()
            .decision_log()
            .snapshot_identity()
            .as_str(),
        "snapshot-a"
    );
    assert_eq!(
        runtime
            .diagnostics()
            .last_historical_evaluation_record()
            .expect("historical evaluation should be retained")
            .record_identity(),
        evaluation.record().record_identity()
    );
    assert_eq!(
        runtime
            .diagnostics()
            .explain_last_evaluation()
            .expect("standard diagnostics should explain the last evaluation")
            .record_identity(),
        evaluation.record().record_identity()
    );
    assert_eq!(
        runtime
            .diagnostics()
            .explain_evaluation(evaluation.record().record_identity())
            .expect("standard diagnostics should explain a named evaluation")
            .record_identity(),
        evaluation.record().record_identity()
    );
    assert!(matches!(
        runtime.diagnostics().explain_last(),
        Some(crate::facade::BridgeStandardDiagnosticsExplanation::Evaluation(_))
    ));
}

#[test]
fn standard_speculation_flow_activates_discards_and_promotes() {
    let runtime = runtime(BridgeRuntimePolicy::default());

    let comparison = runtime
        .speculate(BridgeSpeculativeSessionRequest::new(
            BridgePreviewSessionIdentity::new("preview-session:std-compare"),
            preview_declaration(),
            3,
            1,
            2,
        ))
        .expect("standard speculation should activate")
        .compare_to_main();

    assert_eq!(
        comparison
            .main_evaluation_request(TruthBranchIdentity::new("main"))
            .selector(),
        &crate::facade::BridgeTruthViewSelector::branch_head(TruthBranchIdentity::new("main"))
    );
    assert_eq!(
        comparison.speculative_evaluation_request().selector(),
        &crate::facade::BridgeTruthViewSelector::branch_head(TruthBranchIdentity::new(
            "truth:analysis"
        ))
    );

    let discarded = runtime
        .speculate(BridgeSpeculativeSessionRequest::new(
            BridgePreviewSessionIdentity::new("preview-session:std-discard"),
            preview_declaration(),
            3,
            1,
            2,
        ))
        .expect("standard speculation should activate")
        .discard(vec![
            BridgePreviewResidueClass::PreviewExecutionRetained,
            BridgePreviewResidueClass::ReplayRetainedNonAuthoritative,
            BridgePreviewResidueClass::TemporaryDiagnosticsResidue,
        ])
        .expect("standard speculation discard should succeed");

    assert_eq!(
        discarded.session().lifecycle_state_kind(),
        crate::facade::BridgePreviewLifecycleStateKind::Discarded
    );

    let promoted = runtime
        .speculate(BridgeSpeculativeSessionRequest::new(
            BridgePreviewSessionIdentity::new("preview-session:std-promote"),
            preview_declaration(),
            3,
            1,
            2,
        ))
        .expect("standard speculation should activate")
        .promote()
        .expect("standard promotion should succeed");

    assert_eq!(
        promoted.session().lifecycle_state_kind(),
        crate::facade::BridgePreviewLifecycleStateKind::Promoted
    );
    assert_eq!(runtime.diagnostics().preview_execution_records().len(), 3);
    assert_eq!(runtime.diagnostics().preview_discard_records().len(), 1);
    assert_eq!(runtime.diagnostics().preview_promotion_records().len(), 1);
    let promoted_session_identity =
        BridgePreviewSessionIdentity::new("preview-session:std-promote");
    assert!(matches!(
        runtime
            .diagnostics()
            .explain_session(&promoted_session_identity),
        Some(crate::facade::BridgeStandardSessionExplanation::PreviewPromotion(_))
    ));
    assert!(matches!(
        runtime.diagnostics().explain_last(),
        Some(crate::facade::BridgeStandardDiagnosticsExplanation::PreviewPromotion(_))
    ));
}
