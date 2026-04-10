use crate::facade::{
    BridgePreviewLifecycleStateKind, BridgePreviewResidueClass, BridgePreviewSessionDeclaration,
    BridgePreviewSessionDeclarationIdentity, BridgePreviewSessionIdentity, BridgeRequestKind,
    BridgeSignalBranchIdentity, BridgeSpeculativeBranchBinding,
    BridgeSpeculativeBranchBindingIdentity, TruthBranchIdentity,
};

use super::support::{build_runtime, registration};
use crate::harness::fixtures::{InMemoryRelationalBridgeSource, RecordingSignalBridgeSink};

fn preview_declaration() -> BridgePreviewSessionDeclaration {
    BridgePreviewSessionDeclaration::new(
        BridgePreviewSessionDeclarationIdentity::new("harness:preview-declaration"),
        BridgeRequestKind::Preview,
        BridgeSpeculativeBranchBinding::new(
            BridgeSpeculativeBranchBindingIdentity::new("harness:binding"),
            TruthBranchIdentity::new("main"),
            BridgeSignalBranchIdentity::new("signal:harness"),
        ),
        "truth-view:harness",
        "source-capability:harness",
        "request-shape:harness",
        "artifact-schema:harness",
    )
}

#[test]
fn bridge_harness_speculation_records_are_queryable_across_execution_promotion_and_replay() {
    let source = InMemoryRelationalBridgeSource::default();
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );
    let admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("harness:preview-session"),
            preview_declaration(),
        )
        .expect("preview declaration should admit");
    let (active, execution_record) = runtime.activate_preview_session(admitted, 3, 1, 2);
    let proof = active.promotion_admissibility_proof();
    let (_promoted, promotion_record) = runtime
        .promote_preview_session(
            active,
            &execution_record,
            &proof,
            "commit-boundary:harness",
            "authoritative-artifact:harness",
        )
        .expect("promotion should succeed");
    let replay_bundle = runtime
        .replay_preview_bundle("harness:preview-session")
        .expect("replay bundle should exist");

    let diagnostics = runtime.diagnostics();
    let handle = diagnostics.handle();

    assert_eq!(diagnostics.preview_execution_records().len(), 1);
    assert_eq!(diagnostics.preview_promotion_records().len(), 1);
    assert_eq!(
        diagnostics
            .preview_execution_record_for_session_identity("harness:preview-session")
            .expect("execution record should be queryable by session identity")
            .record_identity(),
        execution_record.record_identity()
    );
    assert_eq!(
        handle
            .preview_promotion_record_for_session_identity("harness:preview-session")
            .expect("promotion record should be queryable through diagnostics handle")
            .record_identity(),
        promotion_record.record_identity()
    );
    assert_eq!(
        diagnostics
            .explain_preview_replay_bundle(&replay_bundle)
            .lifecycle_outcome(),
        BridgePreviewLifecycleStateKind::Promoted
    );
}

#[test]
fn bridge_harness_speculation_discard_replay_remains_queryable() {
    let source = InMemoryRelationalBridgeSource::default();
    let runtime = build_runtime(
        source,
        RecordingSignalBridgeSink::default(),
        vec![registration()],
    );
    let admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new("harness:preview-discard"),
            preview_declaration(),
        )
        .expect("preview declaration should admit");
    let (active, execution_record) = runtime.activate_preview_session(admitted, 2, 1, 1);
    let (_discarded, discard_record) = runtime
        .discard_preview_session(
            active,
            &execution_record,
            vec![
                BridgePreviewResidueClass::PreviewExecutionRetained,
                BridgePreviewResidueClass::TemporaryRoutingResidue,
            ],
        )
        .expect("discard should succeed");
    let replay_bundle = runtime
        .replay_preview_bundle("harness:preview-discard")
        .expect("discard replay bundle should exist");

    let diagnostics = runtime.diagnostics();
    let discard_explanation = diagnostics
        .explain_last_preview_discard_record()
        .expect("discard explanation should exist");

    assert_eq!(diagnostics.preview_discard_records().len(), 1);
    assert_eq!(
        discard_explanation.preview_discard_record_identity(),
        discard_record.record_identity().as_str()
    );
    assert_eq!(
        diagnostics
            .explain_preview_replay_bundle(&replay_bundle)
            .lifecycle_outcome(),
        BridgePreviewLifecycleStateKind::Discarded
    );
}
