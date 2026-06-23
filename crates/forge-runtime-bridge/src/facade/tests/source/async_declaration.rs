use super::*;
use crate::source::{
    BridgeAsyncSignalLoweringFamilyKind, BridgeAsyncSourceDeclarationDraft,
    BridgeAsyncSourceDeclarationIdentity, BridgeAsyncSourceDeclarationRejectionKind,
    BridgeAsyncSourceLegacyDeclarationIdentity,
};
use forge_signal::facade::{
    AsyncNodeCapabilityDeclaration, AsyncNodePayloadContract, AsyncNodePayloadContractId, NodeId,
    ResourceNodeDeclaration, ResourceNodeId, ResourceObservationPolicyDeclaration,
    ResourcePayloadContract, ResourcePayloadContractId,
};

#[test]
fn equivalent_request_response_declarations_validate_and_lower_identically() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let first = runtime
        .validate_async_source_declaration(request_response_draft(
            "bridge-async:request-response",
            "source:legacy-request-response",
            ResourceObservationPolicyDeclaration::LifecycleOnly,
            NodeId::new(11, 0),
        ))
        .expect("request-response declaration should validate");
    let second = runtime
        .validate_async_source_declaration(request_response_draft(
            "bridge-async:request-response",
            "source:legacy-request-response",
            ResourceObservationPolicyDeclaration::LifecycleOnly,
            NodeId::new(88, 0),
        ))
        .expect("adapter-local request-response node variation should still validate");

    assert_eq!(first.family_kind(), second.family_kind());
    assert_eq!(first.canonical_basis(), second.canonical_basis());
    assert_eq!(first.digest(), second.digest());
    assert_eq!(first.counters().async_source_declaration_count(), 1);
    assert_eq!(first.counters().request_response_family_count(), 1);

    let lowered_first = runtime
        .lower_async_source_declaration(&first)
        .expect("validated request-response declaration should lower");
    let lowered_second = runtime
        .lower_async_source_declaration(&second)
        .expect("equivalent validated request-response declaration should lower");

    assert_eq!(
        lowered_first.lowering_family_kind(),
        BridgeAsyncSignalLoweringFamilyKind::ResourceDescriptor
    );
    assert_eq!(
        lowered_first.declaration_identity(),
        first.declaration_identity()
    );
    assert_eq!(
        lowered_first.canonical_basis(),
        lowered_second.canonical_basis()
    );
    assert_eq!(lowered_first.digest(), lowered_second.digest());
    assert_eq!(
        lowered_first.lowering_identity(),
        lowered_second.lowering_identity()
    );
    assert_eq!(
        lowered_first
            .counters()
            .signal_resource_descriptor_lowering_count(),
        1
    );
    assert_eq!(
        lowered_first
            .resource_descriptor()
            .map(|descriptor| descriptor.node()),
        lowered_second
            .resource_descriptor()
            .map(|descriptor| descriptor.node())
    );
    assert_eq!(
        lowered_first
            .resource_descriptor()
            .map(|descriptor| descriptor.node()),
        Some(ResourceNodeId::from_node(NodeId::new(0, 0)))
    );
}

#[test]
fn equivalent_subscription_backed_declarations_validate_and_lower_identically() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let first = runtime
        .validate_async_source_declaration(subscription_backed_draft(
            "bridge-async:subscription-backed",
            "source:legacy-subscription-backed",
            ResourceObservationPolicyDeclaration::LifecycleAndOutput,
            NodeId::new(19, 0),
        ))
        .expect("subscription-backed declaration should validate");
    let second = runtime
        .validate_async_source_declaration(subscription_backed_draft(
            "bridge-async:subscription-backed",
            "source:legacy-subscription-backed",
            ResourceObservationPolicyDeclaration::LifecycleAndOutput,
            NodeId::new(101, 0),
        ))
        .expect("adapter-local subscription-backed node variation should still validate");

    assert_eq!(first.family_kind(), second.family_kind());
    assert_eq!(first.canonical_basis(), second.canonical_basis());
    assert_eq!(first.digest(), second.digest());
    assert_eq!(first.counters().async_source_declaration_count(), 1);
    assert_eq!(first.counters().subscription_backed_family_count(), 1);

    let lowered_first = runtime
        .lower_async_source_declaration(&first)
        .expect("validated subscription-backed declaration should lower");
    let lowered_second = runtime
        .lower_async_source_declaration(&second)
        .expect("equivalent validated subscription-backed declaration should lower");

    assert_eq!(
        lowered_first.lowering_family_kind(),
        BridgeAsyncSignalLoweringFamilyKind::AsyncNodeCapability
    );
    assert_eq!(
        lowered_first.declaration_identity(),
        first.declaration_identity()
    );
    assert_eq!(
        lowered_first.canonical_basis(),
        lowered_second.canonical_basis()
    );
    assert_eq!(lowered_first.digest(), lowered_second.digest());
    assert_eq!(
        lowered_first.lowering_identity(),
        lowered_second.lowering_identity()
    );
    assert_eq!(
        lowered_first
            .counters()
            .signal_async_node_capability_lowering_count(),
        1
    );
    assert_eq!(
        lowered_first
            .async_node_capability_bundle()
            .map(|bundle| bundle.node()),
        lowered_second
            .async_node_capability_bundle()
            .map(|bundle| bundle.node())
    );
    assert_eq!(
        lowered_first
            .async_node_capability_bundle()
            .map(|bundle| bundle.node()),
        Some(NodeId::new(0, 0))
    );
}

#[test]
fn request_response_family_rejects_output_bearing_observation_policy() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let rejection = runtime
        .validate_async_source_declaration(request_response_draft(
            "bridge-async:bad-request-response",
            "source:legacy-bad-request-response",
            ResourceObservationPolicyDeclaration::LifecycleAndOutput,
            NodeId::new(13, 0),
        ))
        .expect_err("output-bearing request-response declaration should reject");

    assert_eq!(
        rejection.kind(),
        BridgeAsyncSourceDeclarationRejectionKind::RequestResponseObservationPolicyMismatch
    );
    assert_eq!(
        rejection
            .counters()
            .async_source_declaration_rejection_count(),
        1
    );
}

#[test]
fn subscription_backed_family_rejects_lifecycle_only_observation_policy() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let rejection = runtime
        .validate_async_source_declaration(subscription_backed_draft(
            "bridge-async:bad-subscription-backed",
            "source:legacy-bad-subscription-backed",
            ResourceObservationPolicyDeclaration::LifecycleOnly,
            NodeId::new(23, 0),
        ))
        .expect_err("lifecycle-only subscription-backed declaration should reject");

    assert_eq!(
        rejection.kind(),
        BridgeAsyncSourceDeclarationRejectionKind::SubscriptionBackedObservationPolicyMismatch
    );
    assert_eq!(
        rejection
            .counters()
            .async_source_declaration_rejection_count(),
        1
    );
}

#[test]
fn async_source_families_produce_distinct_validated_and_lowered_digests() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let request_response = runtime
        .validate_async_source_declaration(request_response_draft(
            "bridge-async:request-response-distinct",
            "source:legacy-request-response-distinct",
            ResourceObservationPolicyDeclaration::LifecycleOnly,
            NodeId::new(31, 0),
        ))
        .expect("request-response declaration should validate");
    let subscription_backed = runtime
        .validate_async_source_declaration(subscription_backed_draft(
            "bridge-async:subscription-backed-distinct",
            "source:legacy-subscription-backed-distinct",
            ResourceObservationPolicyDeclaration::LifecycleAndOutput,
            NodeId::new(41, 0),
        ))
        .expect("subscription-backed declaration should validate");

    assert_ne!(request_response.digest(), subscription_backed.digest());

    let lowered_request_response = runtime
        .lower_async_source_declaration(&request_response)
        .expect("request-response declaration should lower");
    let lowered_subscription_backed = runtime
        .lower_async_source_declaration(&subscription_backed)
        .expect("subscription-backed declaration should lower");

    assert_ne!(
        lowered_request_response.lowering_family_kind(),
        lowered_subscription_backed.lowering_family_kind()
    );
    assert_ne!(
        lowered_request_response.digest(),
        lowered_subscription_backed.digest()
    );
}

fn request_response_draft(
    declaration_identity: &str,
    legacy_declaration_identity: &str,
    observation_policy: ResourceObservationPolicyDeclaration,
    node: NodeId,
) -> BridgeAsyncSourceDeclarationDraft {
    let declaration = ResourceNodeDeclaration::new(
        ResourceNodeId::from_node(node),
        ResourcePayloadContract::new(ResourcePayloadContractId::new(41))
            .with_max_payload_bytes(512),
    )
    .with_observation_policy(observation_policy)
    .with_retry_max_attempts(3);

    BridgeAsyncSourceDeclarationDraft::request_response(
        BridgeAsyncSourceDeclarationIdentity::admit_bridge_owned(declaration_identity),
        BridgeAsyncSourceLegacyDeclarationIdentity::admit_bridge_owned(legacy_declaration_identity),
        declaration,
    )
}

fn subscription_backed_draft(
    declaration_identity: &str,
    legacy_declaration_identity: &str,
    observation_policy: ResourceObservationPolicyDeclaration,
    node: NodeId,
) -> BridgeAsyncSourceDeclarationDraft {
    let declaration = AsyncNodeCapabilityDeclaration::new(
        node,
        AsyncNodePayloadContract::new(AsyncNodePayloadContractId::new(73))
            .with_max_payload_bytes(256),
    )
    .with_observation_policy(observation_policy)
    .with_retry_max_attempts(2);

    BridgeAsyncSourceDeclarationDraft::subscription_backed(
        BridgeAsyncSourceDeclarationIdentity::admit_bridge_owned(declaration_identity),
        BridgeAsyncSourceLegacyDeclarationIdentity::admit_bridge_owned(legacy_declaration_identity),
        declaration,
    )
}
