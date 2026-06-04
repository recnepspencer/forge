use forge_runtime_bridge::facade::{
    BridgePreviewResidueClass, BridgePreviewRetainedArtifactSchema, BridgePreviewSessionBasis,
    BridgePreviewSessionDeclaration, BridgePreviewSessionDeclarationIdentity,
    BridgePreviewSessionIdentity, BridgeRequestKind, BridgeRouteRequest,
    BridgeSignalBranchIdentity, BridgeSourceCapability, BridgeSourceCapabilitySet,
    BridgeSpeculativeBranchBinding, BridgeSpeculativeBranchBindingIdentity,
    BridgeTruthViewEvaluationRequest, BridgeTruthViewSelector, ChangeStreamDeclaration,
    RuntimeBridge, SnapshotReadContract, SnapshotReadPacket, SnapshotReadRequest,
    StreamCheckpointFrontierKind, StreamCheckpointPublicationMode, StreamCoalescingFamily,
    StreamCoalescingIntent, StreamConsumerShape, StreamDeliveryIntent,
    StreamDiagnosticsPolicyClass, StreamReplayMode, StreamResumeMode, StructuralFingerprintFamily,
    StructuralTruthViewBasis, TruthBranchIdentity, TruthCommitIdentity, TruthSnapshotIdentity,
};

use super::super::super::super::*;
use super::super::materialization::{
    bridge_runtime, changed_reference_set, registered_source, registered_structural, request_for,
};
use super::writeback_support::retain_writeback_record_identities;
use lower_runtime_slot_references::bridge_request_with_lower_runtime_slot_references;

mod lower_runtime_slot_references;

pub(super) fn artifact_with_lower_runtime_slot_evidence(
    commit_identity: TruthCommitIdentity,
) -> QueryCausalInspectionArtifact {
    let runtime = bridge_runtime();
    let routed = runtime.route(commit_identity.clone()).unwrap();
    let retained_evidence = retain_lower_runtime_slot_evidence(&runtime, &commit_identity);
    let reference_set = changed_reference_set(routed.route_identity());
    let flow = admit_causal_inspection(request_for(
        reference_set,
        CausalInspectionRichness::ReferenceOnly,
    ));
    let CausalInspectionProofFlow::Admitted(admitted) = flow else {
        panic!("slot evidence fixture should admit");
    };
    let bridge_request = bridge_request_with_lower_runtime_slot_references(
        &admitted,
        &routed,
        &retained_evidence,
        &commit_identity,
    );
    let envelope = runtime
        .diagnostics()
        .assemble_causal_explanation_envelope(bridge_request)
        .expect("bridge envelope should assemble with lower runtime slots");

    materialize_admitted_causal_inspection(
        &admitted,
        &envelope,
        CausalInspectionRedactionPolicy::PreserveDetail,
        CausalInspectionMaterializationPolicy::OfflineInterpretableArtifact,
    )
    .expect("slot evidence materialization should consume bridge envelope")
}

struct RetainedLowerRuntimeSlotEvidence {
    historical_evaluation_record_identity: String,
    preview_execution_record_identity: String,
    preview_discard_record_identity: String,
    source_materialization_record_identity: String,
    structural_remap_record_identity: String,
    stream_replay_record_identity: String,
    writeback_admission_record_identity: String,
    writeback_mapper_envelope_identity: String,
    writeback_mapped_family_input_identity: String,
    writeback_mapper_record_identity: String,
    writeback_execution_record_identity: String,
    writeback_replay_record_identity: String,
}

fn retain_lower_runtime_slot_evidence(
    runtime: &RuntimeBridge,
    commit_identity: &TruthCommitIdentity,
) -> RetainedLowerRuntimeSlotEvidence {
    let (preview_execution_record_identity, preview_discard_record_identity) =
        retain_preview_record_identities(runtime, commit_identity.as_str());
    let writeback = retain_writeback_record_identities(runtime, commit_identity.as_str());
    RetainedLowerRuntimeSlotEvidence {
        historical_evaluation_record_identity: retain_historical_evaluation_record_identity(
            runtime,
        ),
        preview_execution_record_identity,
        preview_discard_record_identity,
        source_materialization_record_identity: retain_source_materialization_record_identity(
            runtime,
        ),
        structural_remap_record_identity: retain_structural_remap_record_identity(runtime),
        stream_replay_record_identity: retain_stream_replay_record_identity(
            runtime,
            commit_identity,
        ),
        writeback_admission_record_identity: writeback.admission_record_identity,
        writeback_mapper_envelope_identity: writeback.mapper_envelope_identity,
        writeback_mapped_family_input_identity: writeback.mapped_family_input_identity,
        writeback_mapper_record_identity: writeback.mapper_record_identity,
        writeback_execution_record_identity: writeback.execution_record_identity,
        writeback_replay_record_identity: writeback.replay_record_identity,
    }
}

fn retain_historical_evaluation_record_identity(runtime: &RuntimeBridge) -> String {
    runtime
        .evaluate(BridgeTruthViewEvaluationRequest::for_branch_head(
            TruthBranchIdentity::new("analysis"),
        ))
        .expect("historical evaluation should retain evidence")
        .record()
        .record_identity()
        .as_str()
        .to_string()
}

fn retain_preview_record_identities(runtime: &RuntimeBridge, suffix: &str) -> (String, String) {
    let preview_admitted = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new(format!("preview-session:{suffix}")),
            preview_declaration(suffix),
        )
        .expect("preview declaration should admit");
    let (preview_active, preview_execution) =
        runtime.activate_preview_session(preview_admitted, 3, 1, 2);
    let (_, preview_discard) = runtime
        .discard_preview_session(
            preview_active,
            &preview_execution,
            vec![
                BridgePreviewResidueClass::PreviewExecutionRetained,
                BridgePreviewResidueClass::ReplayRetainedNonAuthoritative,
                BridgePreviewResidueClass::TemporaryDiagnosticsResidue,
            ],
        )
        .expect("preview discard should retain evidence");
    (
        preview_execution.record_identity().as_str().to_string(),
        preview_discard.record_identity().as_str().to_string(),
    )
}

fn retain_source_materialization_record_identity(runtime: &RuntimeBridge) -> String {
    let source_contract = runtime
        .admit_source(registered_source(
            "source:causal-materialization-history",
            BridgeTruthViewSelector::historical_commit(
                TruthBranchIdentity::new("analysis"),
                TruthCommitIdentity::new("commit-a"),
            ),
            vec![
                BridgeSourceCapability::SnapshotRead,
                BridgeSourceCapability::HistoricalRead,
                BridgeSourceCapability::BranchRead,
                BridgeSourceCapability::ReplayContinuityRead,
            ],
        ))
        .expect("source declaration should admit");
    let source_observation = runtime
        .materialize_source_packet(&source_contract, SnapshotReadPacket::new(vec![]))
        .expect("source packet should materialize");
    runtime
        .canonicalize_source_materialization_record(&source_contract, &source_observation)
        .expect("source materialization should retain evidence")
        .record_identity()
        .as_str()
        .to_string()
}

fn retain_structural_remap_record_identity(runtime: &RuntimeBridge) -> String {
    let structural_contract = runtime
        .admit_structural_comparison(registered_structural(
            "structural:causal-materialization-snapshot",
            StructuralFingerprintFamily::TopologyFingerprint,
            StructuralTruthViewBasis::explicit_snapshot(BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("analysis"),
                TruthSnapshotIdentity::new("snapshot-causal-materialization"),
            )),
        ))
        .expect("structural declaration should admit");
    let structural_read = SnapshotReadPacket::new(vec![SnapshotReadRequest::for_coarse(
        "entity-1",
        SnapshotReadContract::scalar(
            forge_foundational::facade::AspectKey::new("profile")
                .expect("valid snapshot aspect key"),
            forge_foundational::facade::ScalarAspectType::String,
        ),
    )]);
    let structural_planned = runtime
        .plan_structural_match_packet_set_from_read_packets(
            &structural_contract,
            structural_read.clone(),
            vec![structural_read],
        )
        .expect("structural packets should plan");
    let structural_reduced = runtime
        .reduce_structural_match_set(&structural_planned)
        .expect("structural set should reduce");
    let structural_artifact = runtime
        .publish_structural_remap_artifact(&structural_reduced)
        .expect("structural artifact should publish");
    runtime
        .canonicalize_structural_remap_record(
            &structural_contract,
            &structural_planned,
            &structural_reduced,
            &structural_artifact,
        )
        .record_identity()
        .as_str()
        .to_string()
}

fn retain_stream_replay_record_identity(
    runtime: &RuntimeBridge,
    commit_identity: &TruthCommitIdentity,
) -> String {
    let stream_protocol = runtime
        .validate_change_stream_declaration(ChangeStreamDeclaration::new(
            StreamConsumerShape::RoutingConsumer,
            StreamResumeMode::FromCheckpointOnly,
            StreamCheckpointPublicationMode::PublishEveryWindow,
            StreamCoalescingIntent::Prefer(StreamCoalescingFamily::RoutingWindowCoalescing),
            StreamReplayMode::Enabled,
            StreamDeliveryIntent::RouteInvalidations,
            StreamDiagnosticsPolicyClass::Standard,
        ))
        .expect("stream declaration should validate");
    let stream_contract = runtime
        .resolve_change_stream_consumer_contract(&stream_protocol)
        .expect("stream contract should resolve");
    let stream_envelope = runtime
        .ingest_committed_patch(BridgeRouteRequest::for_commit(TruthCommitIdentity::new(
            format!("commit-stream-{}", commit_identity.as_str()),
        )))
        .expect("stream commit should ingest");
    let stream_window = runtime
        .plan_change_stream_window(&stream_contract, vec![stream_envelope])
        .expect("stream window should plan");
    let stream_checkpoint = runtime.publish_consumer_checkpoint(
        &stream_contract,
        &stream_window,
        StreamCheckpointFrontierKind::ContiguousFrontier,
    );
    runtime
        .canonicalize_stream_replay_record(&stream_contract, &stream_window, &stream_checkpoint)
        .expect("stream replay should retain evidence")
        .replay_record_identity()
        .as_str()
        .to_string()
}

fn preview_declaration(suffix: &str) -> BridgePreviewSessionDeclaration {
    BridgePreviewSessionDeclaration::new(
        BridgePreviewSessionDeclarationIdentity::new(format!("preview:slot:{suffix}")),
        BridgeRequestKind::Preview,
        BridgeSpeculativeBranchBinding::new(
            BridgeSpeculativeBranchBindingIdentity::new(format!("binding:slot:{suffix}")),
            TruthBranchIdentity::new("truth:analysis"),
            BridgeSignalBranchIdentity::new("signal:analysis"),
        ),
        BridgePreviewSessionBasis::new(
            BridgeTruthViewSelector::branch_snapshot(
                TruthBranchIdentity::new("truth:analysis"),
                TruthSnapshotIdentity::new("snapshot-causal-materialization"),
            ),
            BridgeSourceCapabilitySet::new(vec![BridgeSourceCapability::SnapshotRead]),
            BridgePreviewRetainedArtifactSchema::PreviewLifecycleArtifactsV1,
        ),
    )
}
