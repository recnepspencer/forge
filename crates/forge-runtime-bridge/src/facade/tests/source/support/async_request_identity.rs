use crate::facade::{
    AdmittedBridgeAsyncRequestIdentity, BridgeAsyncRequestAdmissionRequest,
    BridgeAsyncRequestIdentityRejection, BridgeAsyncRequestSubscriptionInstance,
    BridgeAsyncRequestTruthViewBasis, BridgeAsyncSourceDeclarationDraft,
    BridgeAsyncSourceDeclarationIdentity, BridgeAsyncSourceLegacyDeclarationIdentity,
    BridgePreviewActiveSubscription, BridgePreviewRetainedArtifactSchema,
    BridgePreviewSessionBasis, BridgePreviewSessionDeclaration,
    BridgePreviewSessionDeclarationIdentity, BridgePreviewSessionIdentity, BridgeRequestKind,
    BridgeSignalBranchIdentity, BridgeSourceCapability, BridgeSourceCapabilitySet,
    BridgeSpeculativeBranchBinding, BridgeSpeculativeBranchBindingIdentity,
    BridgeSubscriptionActivationReady, BridgeSubscriptionBasisRequest,
    BridgeSubscriptionConsumerBackpressurePosture, BridgeSubscriptionConsumerContractFamily,
    BridgeSubscriptionConsumerDiagnosticsRetention, BridgeSubscriptionConsumerPacingCapability,
    BridgeSubscriptionDeclarationFamilyKind, BridgeSubscriptionDeliveryDensityPosture,
    BridgeSubscriptionDeliveryIntentClass, BridgeTruthViewSelector,
    LoweredBridgeAsyncSourceDeclaration, RuntimeBridge,
};
use crate::input::envelope::TruthBranchIdentity;
use crate::mapping::SubscriptionSliceKind;
use crate::snapshot::TruthSnapshotIdentity;
use crate::subscription::NormalizedSubscriptionSliceIntent;
use forge_foundational::facade::{AspectKey, FieldKey};
use forge_signal::facade::{
    AsyncNodeCapabilityDeclaration, AsyncNodePayloadContract, AsyncNodePayloadContractId, NodeId,
    ResourceNodeDeclaration, ResourceNodeId, ResourceObservationPolicyDeclaration,
    ResourcePayloadContract, ResourcePayloadContractId, ResourceTimeoutPolicyDeclaration,
    TemporalDuration,
};

pub(crate) fn admit_request_response_identity(
    runtime: &RuntimeBridge,
    node: NodeId,
    truth_basis: BridgeAsyncRequestTruthViewBasis,
) -> AdmittedBridgeAsyncRequestIdentity {
    let lowered = lowered_request_response(runtime, node);
    let binding = runtime.bind_async_request_basis(&lowered, truth_basis);
    let request = BridgeAsyncRequestAdmissionRequest::request_response(&lowered, &binding)
        .expect("request-response basis binding should construct");
    runtime
        .admit_async_request_identity(request)
        .expect("request-response identity should admit")
}

pub(crate) fn admit_timeout_request_response_identity(
    runtime: &RuntimeBridge,
    node: NodeId,
    truth_basis: BridgeAsyncRequestTruthViewBasis,
    timeout_ms: u64,
) -> AdmittedBridgeAsyncRequestIdentity {
    let lowered = lowered_timeout_request_response(runtime, node, timeout_ms);
    let binding = runtime.bind_async_request_basis(&lowered, truth_basis);
    let request = BridgeAsyncRequestAdmissionRequest::request_response(&lowered, &binding)
        .expect("timeout request-response basis binding should construct");
    runtime
        .admit_async_request_identity(request)
        .expect("timeout request-response identity should admit")
}

pub(crate) fn admit_subscription_backed_identity(
    runtime: &RuntimeBridge,
    node: NodeId,
    truth_basis: BridgeAsyncRequestTruthViewBasis,
    subscription_instance: BridgeAsyncRequestSubscriptionInstance,
) -> Result<AdmittedBridgeAsyncRequestIdentity, BridgeAsyncRequestIdentityRejection> {
    let lowered = lowered_subscription_backed(runtime, node);
    let binding = runtime.bind_async_request_basis(&lowered, truth_basis);
    let request = BridgeAsyncRequestAdmissionRequest::subscription_backed(
        &lowered,
        &binding,
        subscription_instance,
    )?;
    runtime.admit_async_request_identity(request)
}

pub(crate) fn activation_ready_for_snapshot(
    runtime: &RuntimeBridge,
    snapshot_identity: TruthSnapshotIdentity,
) -> BridgeSubscriptionActivationReady {
    activation_ready_for_basis(
        runtime,
        BridgeSubscriptionBasisRequest::snapshot(snapshot_identity),
    )
}

pub(crate) fn activation_ready_for_branch_head(
    runtime: &RuntimeBridge,
    branch_identity: TruthBranchIdentity,
) -> BridgeSubscriptionActivationReady {
    activation_ready_for_basis(
        runtime,
        BridgeSubscriptionBasisRequest::branch_head(branch_identity),
    )
}

pub(crate) fn preview_active_subscription(
    runtime: &RuntimeBridge,
    suffix: &str,
) -> BridgePreviewActiveSubscription {
    preview_active_subscription_with_basis(
        runtime,
        suffix,
        crate::truth_identity_fixtures::truth_branch_fixture(format!("truth-branch:{suffix}")),
        crate::truth_identity_fixtures::truth_snapshot_fixture(format!("snapshot:{suffix}")),
    )
}

pub(crate) fn preview_active_subscription_with_basis(
    runtime: &RuntimeBridge,
    suffix: &str,
    truth_branch_identity: TruthBranchIdentity,
    truth_snapshot_identity: TruthSnapshotIdentity,
) -> BridgePreviewActiveSubscription {
    let ready = activation_ready_for_snapshot(
        runtime,
        crate::truth_identity_fixtures::truth_snapshot_fixture("snapshot-a"),
    );
    let admitted_preview = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new(format!("preview-session:{suffix}")),
            BridgePreviewSessionDeclaration::new(
                BridgePreviewSessionDeclarationIdentity::new(format!(
                    "preview-declaration:{suffix}"
                )),
                BridgeRequestKind::Preview,
                BridgeSpeculativeBranchBinding::new(
                    BridgeSpeculativeBranchBindingIdentity::new(format!(
                        "preview-binding:{suffix}"
                    )),
                    truth_branch_identity.clone(),
                    BridgeSignalBranchIdentity::new(format!("signal-branch:{suffix}")),
                ),
                BridgePreviewSessionBasis::new(
                    BridgeTruthViewSelector::branch_snapshot(
                        truth_branch_identity,
                        truth_snapshot_identity,
                    ),
                    BridgeSourceCapabilitySet::new(vec![
                        BridgeSourceCapability::SnapshotRead,
                        BridgeSourceCapability::BranchRead,
                    ]),
                    BridgePreviewRetainedArtifactSchema::PreviewLifecycleArtifactsV1,
                ),
            ),
        )
        .expect("preview session should admit");
    let (active_preview, execution_record) =
        runtime.activate_preview_session(admitted_preview, 3, 1, 2);
    let preview_basis = runtime
        .admit_subscription_preview_basis(&active_preview, &execution_record)
        .expect("preview basis should admit");
    let cost_profile = runtime
        .admit_subscription_delivery_cost_profile(
            BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
            4,
            4,
            1,
        )
        .expect("cost profile should admit");
    let consumer = runtime
        .admit_subscription_consumer_contract(
            BridgeSubscriptionConsumerContractFamily::CanonicalDelivery,
            BridgeSubscriptionConsumerPacingCapability::Immediate,
            BridgeSubscriptionConsumerBackpressurePosture::PacingOnly,
            true,
            BridgeSubscriptionConsumerDiagnosticsRetention::MinimalReference,
        )
        .expect("consumer contract should admit");
    runtime.activate_preview_subscription_delivery(ready, preview_basis, cost_profile, consumer)
}

pub(crate) fn lowered_request_response(
    runtime: &RuntimeBridge,
    node: NodeId,
) -> LoweredBridgeAsyncSourceDeclaration {
    let validated = runtime
        .validate_async_source_declaration(request_response_draft(node))
        .expect("request-response declaration should validate");
    runtime
        .lower_async_source_declaration(&validated)
        .expect("request-response declaration should lower")
}

pub(crate) fn lowered_timeout_request_response(
    runtime: &RuntimeBridge,
    node: NodeId,
    timeout_ms: u64,
) -> LoweredBridgeAsyncSourceDeclaration {
    let validated = runtime
        .validate_async_source_declaration(timeout_request_response_draft(node, timeout_ms))
        .expect("timeout request-response declaration should validate");
    runtime
        .lower_async_source_declaration(&validated)
        .expect("timeout request-response declaration should lower")
}

pub(crate) fn lowered_subscription_backed(
    runtime: &RuntimeBridge,
    node: NodeId,
) -> LoweredBridgeAsyncSourceDeclaration {
    let validated = runtime
        .validate_async_source_declaration(subscription_backed_draft(node))
        .expect("subscription-backed declaration should validate");
    runtime
        .lower_async_source_declaration(&validated)
        .expect("subscription-backed declaration should lower")
}

pub(crate) fn request_response_draft(node: NodeId) -> BridgeAsyncSourceDeclarationDraft {
    BridgeAsyncSourceDeclarationDraft::request_response(
        BridgeAsyncSourceDeclarationIdentity::new("bridge-async:request-response"),
        BridgeAsyncSourceLegacyDeclarationIdentity::new("source:legacy-request-response"),
        ResourceNodeDeclaration::new(
            ResourceNodeId::from_node(node),
            ResourcePayloadContract::new(ResourcePayloadContractId::new(41))
                .with_max_payload_bytes(512),
        )
        .with_observation_policy(ResourceObservationPolicyDeclaration::LifecycleOnly)
        .with_retry_max_attempts(3),
    )
}

fn timeout_request_response_draft(
    node: NodeId,
    timeout_ms: u64,
) -> BridgeAsyncSourceDeclarationDraft {
    BridgeAsyncSourceDeclarationDraft::request_response(
        BridgeAsyncSourceDeclarationIdentity::new("bridge-async:request-response-timeout"),
        BridgeAsyncSourceLegacyDeclarationIdentity::new("source:legacy-request-response-timeout"),
        ResourceNodeDeclaration::new(
            ResourceNodeId::from_node(node),
            ResourcePayloadContract::new(ResourcePayloadContractId::new(42))
                .with_max_payload_bytes(512),
        )
        .with_observation_policy(ResourceObservationPolicyDeclaration::LifecycleOnly)
        .with_timeout_policy(ResourceTimeoutPolicyDeclaration::FixedTimeout {
            timeout: TemporalDuration::temporal_duration(timeout_ms)
                .expect("timeout duration should validate"),
        })
        .with_retry_max_attempts(3),
    )
}

fn subscription_backed_draft(node: NodeId) -> BridgeAsyncSourceDeclarationDraft {
    BridgeAsyncSourceDeclarationDraft::subscription_backed(
        BridgeAsyncSourceDeclarationIdentity::new("bridge-async:subscription-backed"),
        BridgeAsyncSourceLegacyDeclarationIdentity::new("source:legacy-subscription-backed"),
        AsyncNodeCapabilityDeclaration::new(
            node,
            AsyncNodePayloadContract::new(AsyncNodePayloadContractId::new(73))
                .with_max_payload_bytes(256),
        )
        .with_observation_policy(ResourceObservationPolicyDeclaration::LifecycleAndOutput)
        .with_retry_max_attempts(2),
    )
}

fn activation_ready_for_basis(
    runtime: &RuntimeBridge,
    basis_request: BridgeSubscriptionBasisRequest,
) -> BridgeSubscriptionActivationReady {
    let declaration = runtime
        .declare_subscription(
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            vec![NormalizedSubscriptionSliceIntent::try_new_entity_field(
                "entity-1",
                AspectKey::new("profile").expect("valid aspect key"),
                FieldKey::new("name".to_owned()).expect("valid field key"),
                SubscriptionSliceKind::SignalField,
            )
            .expect("slice intent should validate")],
            BridgeSubscriptionDeliveryIntentClass::None,
        )
        .expect("declaration should succeed");
    let admitted = runtime
        .admit_subscription(&declaration, basis_request)
        .expect("subscription admission should succeed");
    runtime.prepare_subscription_activation(&admitted)
}
