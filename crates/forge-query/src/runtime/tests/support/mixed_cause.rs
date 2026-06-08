use forge_foundational::facade::{
    AspectKey, AspectLocator, CanonicalFieldPath, FieldKey, LocatorAuthority,
};
use forge_proof::TransitionOutcome;
use forge_runtime_bridge::facade::{
    AdmittedBridgeAsyncCompletion, AdmittedBridgeSubscription, AdmittedBridgeTemporalBasis,
    BridgeAsyncCompletionAdmissionReport, BridgeAsyncRequestAdmissionRequest,
    BridgeAsyncRequestTruthViewBasis, BridgeAsyncSourceDeclarationDraft,
    BridgeAsyncSourceDeclarationIdentity, BridgeAsyncSourceLegacyDeclarationIdentity,
    BridgeCommittedPatchEnvelope, BridgeCommittedPatchEnvelopeIdentity, BridgeCommittedPatchItem,
    BridgeCommittedPatchTarget, BridgePreviewSessionIdentity, BridgeSubscriptionBasisRequest,
    BridgeSubscriptionDeclarationFamilyKind, BridgeSubscriptionDeliveryIntentClass,
    BridgeSubscriptionPreviewBasisBinding, BridgeTemporalCauseRecord, BridgeTemporalSignalBasis,
    BridgeTemporalSubscriptionFamilyKind, BridgeTemporalTruthViewBasis, BridgeTemporalWakeEvidence,
    NormalizedSubscriptionSliceIntent, RuntimeBridge, SubscriptionSliceKind,
};
use forge_signal::facade::{
    ClockAdvanceOrdinal, ClockDomain, ClockTick, NodeId, ResourceNodeDeclaration,
    ResourceObservationPolicyDeclaration, ResourcePayloadContract, ResourcePayloadContractId,
    TemporalWakeId, WakeOrdinal,
};

use super::*;

pub(in crate::runtime::tests) fn canonical_truth_patch(
    branch_identity: &str,
    snapshot_identity: &str,
    commit_identity: &str,
    patch_identity: &str,
) -> BridgeCommittedPatchEnvelope {
    BridgeCommittedPatchEnvelope::new(
        BridgeCommittedPatchEnvelopeIdentity::new(
            TruthCommitIdentity::new(commit_identity),
            TruthPatchIdentity::new(patch_identity),
            TruthSnapshotIdentity::new(snapshot_identity),
            TruthBranchIdentity::new(branch_identity),
        ),
        vec![BridgeCommittedPatchItem::with_target(
            "entity-1",
            BridgeCommittedPatchTarget::entity_field_path(
                AspectLocator::new(
                    LocatorAuthority::Authoritative,
                    AspectKey::new("profile").expect("valid test aspect key"),
                ),
                CanonicalFieldPath::single(
                    FieldKey::new("name".to_owned()).expect("valid test field key"),
                ),
            ),
        )],
    )
    .expect("mixed-cause truth patch should construct")
}

pub(in crate::runtime::tests) fn authoritative_time_only_cause(
    runtime: &RuntimeBridge,
) -> BridgeTemporalCauseRecord {
    let admitted = admitted_detail_subscription(runtime);
    let temporal = runtime
        .admit_temporal_subscription(
            &admitted,
            admitted_temporal_basis(BridgeTemporalTruthViewBasis::authoritative(
                TruthBranchIdentity::new("truth-main"),
                TruthCommitIdentity::new("commit-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
            )),
            BridgeTemporalSubscriptionFamilyKind::WakeDriven,
        )
        .expect("temporal subscription should admit");
    let ready = runtime.prepare_temporal_subscription_activation(&temporal);
    let request = runtime.prepare_temporal_wake_routing(&ready);
    runtime
        .route_temporal_wake(&request, None)
        .expect("time-only cause should route")
}

pub(in crate::runtime::tests) fn authoritative_truth_plus_time_cause(
    runtime: &RuntimeBridge,
    truth_patch: &BridgeCommittedPatchEnvelope,
) -> BridgeTemporalCauseRecord {
    let admitted = admitted_detail_subscription(runtime);
    let temporal = runtime
        .admit_temporal_subscription(
            &admitted,
            admitted_temporal_basis(BridgeTemporalTruthViewBasis::authoritative(
                TruthBranchIdentity::new("truth-main"),
                TruthCommitIdentity::new("commit-a"),
                TruthSnapshotIdentity::new("snapshot-a"),
            )),
            BridgeTemporalSubscriptionFamilyKind::WakeDriven,
        )
        .expect("temporal subscription should admit");
    let ready = runtime.prepare_temporal_subscription_activation(&temporal);
    let request = runtime.prepare_temporal_wake_routing(&ready);
    runtime
        .route_temporal_wake_with_truth_patch(&request, truth_patch, None)
        .expect("truth-plus-time cause should route")
}

pub(in crate::runtime::tests) fn preview_time_only_cause(
    runtime: &RuntimeBridge,
    suffix: &str,
) -> BridgeTemporalCauseRecord {
    let admitted = admitted_detail_subscription(runtime);
    let preview_basis = admitted_preview_basis_for_truth(
        runtime,
        suffix,
        TruthBranchIdentity::new("truth-preview"),
        TruthSnapshotIdentity::new("snapshot-a"),
    );
    let preview_temporal = runtime
        .admit_preview_temporal_subscription(
            &admitted,
            &preview_basis,
            admitted_temporal_basis(BridgeTemporalTruthViewBasis::authoritative(
                TruthBranchIdentity::new("truth-preview"),
                TruthCommitIdentity::new("commit-preview"),
                TruthSnapshotIdentity::new("snapshot-a"),
            )),
            BridgeTemporalSubscriptionFamilyKind::WakeDriven,
        )
        .expect("preview temporal subscription should admit");
    let ready = runtime.prepare_preview_temporal_subscription_activation(&preview_temporal);
    let request = runtime.prepare_preview_temporal_wake_routing(&ready);
    runtime
        .route_temporal_wake(&request, None)
        .expect("preview time-only cause should route")
}

pub(in crate::runtime::tests) fn admitted_async_completion(
    runtime: &RuntimeBridge,
    node: NodeId,
    truth_basis: BridgeAsyncRequestTruthViewBasis,
    payload_byte_len: u64,
) -> AdmittedBridgeAsyncCompletion {
    admit_request_response_completion(runtime, node, truth_basis, payload_byte_len)
        .admitted_completion()
        .expect("completion should admit")
        .clone()
}

fn admitted_detail_subscription(runtime: &RuntimeBridge) -> AdmittedBridgeSubscription {
    let declaration = runtime
        .declare_subscription(
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            vec![NormalizedSubscriptionSliceIntent::try_new_entity_field(
                "entity-1",
                AspectKey::new("profile").expect("valid native subscription aspect key"),
                FieldKey::new("name".to_owned()).expect("valid native subscription field key"),
                SubscriptionSliceKind::SignalField,
            )
            .expect("slice intent should validate")],
            BridgeSubscriptionDeliveryIntentClass::None,
        )
        .expect("subscription declaration should succeed");
    runtime
        .admit_subscription(
            &declaration,
            BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new("snapshot-a")),
        )
        .expect("bridge subscription should admit")
}

fn admitted_temporal_basis(
    truth_basis: BridgeTemporalTruthViewBasis,
) -> AdmittedBridgeTemporalBasis {
    let signal_basis = BridgeTemporalSignalBasis::new(
        truth_basis.branch_identity().clone(),
        ClockDomain::MonotonicExecution,
        ClockTick::new(5),
        ClockAdvanceOrdinal::new(3),
        None,
    );
    let wake = BridgeTemporalWakeEvidence::new(
        TemporalWakeId::new(11),
        WakeOrdinal::new(7),
        ClockTick::new(5),
    );
    match AdmittedBridgeTemporalBasis::admit(truth_basis, signal_basis, Some(wake)) {
        TransitionOutcome::Success(admitted) => admitted,
        outcome => panic!("expected admitted temporal basis, got {outcome:?}"),
    }
}

fn admitted_preview_basis_for_truth(
    runtime: &RuntimeBridge,
    suffix: &str,
    truth_branch_identity: TruthBranchIdentity,
    snapshot_identity: TruthSnapshotIdentity,
) -> BridgeSubscriptionPreviewBasisBinding {
    let admitted_preview = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new(format!("preview-session:{suffix}")),
            preview_declaration_for_truth(suffix, truth_branch_identity, snapshot_identity),
        )
        .expect("preview session should admit");
    let (active_preview, execution_record) =
        runtime.activate_preview_session(admitted_preview, 3, 1, 2);
    runtime
        .admit_subscription_preview_basis(&active_preview, &execution_record)
        .expect("preview basis should admit")
}

fn preview_declaration_for_truth(
    suffix: &str,
    truth_branch_identity: TruthBranchIdentity,
    snapshot_identity: TruthSnapshotIdentity,
) -> forge_runtime_bridge::facade::BridgePreviewSessionDeclaration {
    let selector_branch_identity = truth_branch_identity.clone();
    forge_runtime_bridge::facade::BridgePreviewSessionDeclaration::new(
        forge_runtime_bridge::facade::BridgePreviewSessionDeclarationIdentity::new(format!(
            "preview-declaration:{suffix}"
        )),
        forge_runtime_bridge::facade::BridgeRequestKind::Preview,
        forge_runtime_bridge::facade::BridgeSpeculativeBranchBinding::new(
            forge_runtime_bridge::facade::BridgeSpeculativeBranchBindingIdentity::new(format!(
                "preview-binding:{suffix}"
            )),
            truth_branch_identity,
            forge_runtime_bridge::facade::BridgeSignalBranchIdentity::new(format!(
                "signal-branch:{suffix}"
            )),
        ),
        forge_runtime_bridge::facade::BridgePreviewSessionBasis::new(
            forge_runtime_bridge::facade::BridgeTruthViewSelector::branch_snapshot(
                selector_branch_identity,
                snapshot_identity,
            ),
            forge_runtime_bridge::facade::BridgeSourceCapabilitySet::new(vec![
                forge_runtime_bridge::facade::BridgeSourceCapability::SnapshotRead,
                forge_runtime_bridge::facade::BridgeSourceCapability::BranchRead,
            ]),
            forge_runtime_bridge::facade::BridgePreviewRetainedArtifactSchema::PreviewLifecycleArtifactsV1,
        ),
    )
}

fn admit_request_response_completion(
    runtime: &RuntimeBridge,
    node: NodeId,
    truth_basis: BridgeAsyncRequestTruthViewBasis,
    payload_byte_len: u64,
) -> BridgeAsyncCompletionAdmissionReport {
    let lowered = runtime
        .lower_async_source_declaration(
            &runtime
                .validate_async_source_declaration(request_response_draft(node))
                .expect("request-response declaration should validate"),
        )
        .expect("request-response declaration should lower");
    let binding = runtime.bind_async_request_basis(&lowered, truth_basis);
    let request = BridgeAsyncRequestAdmissionRequest::request_response(&lowered, &binding)
        .expect("request-response admission request should construct");
    let request_identity = runtime
        .admit_async_request_identity(request)
        .expect("request-response identity should admit");
    let validated = runtime
        .validate_async_completion_envelope(
            &request_identity,
            forge_signal::facade::RawCompletionEnvelope::new(
                request_identity.request_handle().request_id(),
                request_identity.request_handle().generation(),
                request_identity.request_handle().branch_epoch(),
                request_identity.attempt(),
                request_identity
                    .lowered()
                    .resource_descriptor()
                    .expect("request-response identity should retain resource descriptor")
                    .payload_contract_digest()
                    .clone(),
                payload_byte_len,
            ),
        )
        .expect("request-response completion envelope should validate");
    runtime
        .admit_async_completion(&request_identity, &validated)
        .expect("request-response completion should admit or deny canonically")
}

fn request_response_draft(node: NodeId) -> BridgeAsyncSourceDeclarationDraft {
    BridgeAsyncSourceDeclarationDraft::request_response(
        BridgeAsyncSourceDeclarationIdentity::new("bridge-async:request-response"),
        BridgeAsyncSourceLegacyDeclarationIdentity::new("source:legacy-request-response"),
        ResourceNodeDeclaration::new(
            forge_signal::facade::ResourceNodeId::from_node(node),
            ResourcePayloadContract::new(ResourcePayloadContractId::new(142))
                .with_max_payload_bytes(512),
        )
        .with_observation_policy(ResourceObservationPolicyDeclaration::LifecycleOnly),
    )
}
