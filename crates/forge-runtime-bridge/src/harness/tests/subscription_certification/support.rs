use crate::builder::RuntimeBridgeBuilder;
use crate::facade::{
    BridgePreviewSessionDeclaration, BridgePreviewSessionDeclarationIdentity,
    BridgePreviewSessionIdentity, BridgeRequestKind, BridgeRuntimePolicy,
    BridgeSignalBranchIdentity, BridgeSpeculativeBranchBinding,
    BridgeSpeculativeBranchBindingIdentity, BridgeSubscriptionBasisRequest,
    BridgeSubscriptionConsumerBackpressurePosture, BridgeSubscriptionConsumerContract,
    BridgeSubscriptionConsumerContractFamily, BridgeSubscriptionConsumerDiagnosticsRetention,
    BridgeSubscriptionConsumerPacingCapability, BridgeSubscriptionDeclarationFamilyKind,
    BridgeSubscriptionDeliveryDensityPosture, BridgeSubscriptionDeliveryFamilyKind,
    BridgeSubscriptionDeliveryIntentClass, BridgeSubscriptionDeliveryMemberClass,
    BridgeSubscriptionDeliveryMemberInput, BridgeSubscriptionDuplicateReplayPolicyKind,
    NormalizedSubscriptionSliceIntent, RuntimeBridge,
};
use crate::input::envelope::{
    BridgeCommittedPatchItem, RawCommittedPatchEnvelope, TruthBranchIdentity, TruthCommitIdentity,
    TruthPatchIdentity,
};
use crate::mapping::{
    BridgeMappingId, BridgeMappingRegistration, CoarseRoutingMode, MappingSelector,
    SignalInvalidationScope, SubscriptionSliceKind, TruthPatchScope,
};
use crate::snapshot::{SnapshotReadPacket, TruthSnapshotIdentity, TruthSnapshotReader};

#[derive(Clone)]
pub(crate) struct StaticSource;

struct StaticSnapshotReader;

impl TruthSnapshotReader for StaticSnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        TruthSnapshotIdentity::new("snapshot-a")
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<crate::snapshot::SnapshotReadPacketResult, crate::snapshot::BridgeSnapshotReadError>
    {
        Ok(crate::snapshot::SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::new("snapshot-a"),
            request
                .reads()
                .iter()
                .map(|read| {
                    crate::snapshot::SnapshotReadRecord::new(
                        read.request_key(),
                        b"fixture-value".to_vec(),
                    )
                })
                .collect(),
        ))
    }
}

impl crate::adapter::CommittedPatchSource for StaticSource {
    fn load_committed_patch(
        &self,
        request: crate::adapter::RelationalCommittedPatchRequest,
    ) -> Result<RawCommittedPatchEnvelope, crate::adapter::RelationalBridgeSourceError> {
        Ok(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new(request.commit_identity()),
            TruthPatchIdentity::new(format!("patch-for-{}", request.commit_identity())),
            TruthSnapshotIdentity::new("snapshot-a"),
            TruthBranchIdentity::new("analysis"),
            vec![BridgeCommittedPatchItem::new("entity-1", "profile", "name")],
        ))
    }
}

impl crate::adapter::SnapshotReadSource for StaticSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, crate::adapter::RelationalBridgeSourceError> {
        if identity.as_str() == "snapshot-a" {
            Ok(Box::new(StaticSnapshotReader))
        } else {
            Err(crate::adapter::RelationalBridgeSourceError::new(format!(
                "unknown snapshot `{}`",
                identity.as_str()
            )))
        }
    }
}

impl crate::adapter::TruthBranchHeadSource for StaticSource {
    fn load_branch_head_patch(
        &self,
        branch_identity: &TruthBranchIdentity,
    ) -> Result<RawCommittedPatchEnvelope, crate::adapter::RelationalBridgeSourceError> {
        Ok(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new(format!("head-{}", branch_identity.as_str())),
            TruthPatchIdentity::new(format!("patch-{}", branch_identity.as_str())),
            TruthSnapshotIdentity::new("snapshot-a"),
            branch_identity.clone(),
            vec![BridgeCommittedPatchItem::new("entity-1", "profile", "name")],
        ))
    }
}

struct StaticSink;

impl crate::adapter::InvalidationSink for StaticSink {
    fn deliver_invalidation(
        &self,
        delivery: crate::routing::BridgeSignalInvalidationDelivery,
    ) -> Result<crate::delivery::BridgeDeliveryReceipt, crate::adapter::SignalBridgeSinkError> {
        Ok(crate::delivery::BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}

#[derive(Clone)]
pub(crate) struct MisbindingSource;

struct MisbindingSnapshotReader;

impl TruthSnapshotReader for MisbindingSnapshotReader {
    fn snapshot_identity(&self) -> TruthSnapshotIdentity {
        TruthSnapshotIdentity::new("snapshot-bad")
    }

    fn read_packet(
        &self,
        request: &SnapshotReadPacket,
    ) -> Result<crate::snapshot::SnapshotReadPacketResult, crate::snapshot::BridgeSnapshotReadError>
    {
        Ok(crate::snapshot::SnapshotReadPacketResult::new(
            TruthSnapshotIdentity::new("snapshot-bad"),
            request
                .reads()
                .iter()
                .map(|read| {
                    crate::snapshot::SnapshotReadRecord::new(
                        read.request_key(),
                        b"fixture-value".to_vec(),
                    )
                })
                .collect(),
        ))
    }
}

impl crate::adapter::CommittedPatchSource for MisbindingSource {
    fn load_committed_patch(
        &self,
        request: crate::adapter::RelationalCommittedPatchRequest,
    ) -> Result<RawCommittedPatchEnvelope, crate::adapter::RelationalBridgeSourceError> {
        StaticSource.load_committed_patch(request)
    }
}

impl crate::adapter::SnapshotReadSource for MisbindingSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, crate::adapter::RelationalBridgeSourceError> {
        if identity.as_str() == "snapshot-a" {
            Ok(Box::new(MisbindingSnapshotReader))
        } else {
            Err(crate::adapter::RelationalBridgeSourceError::new(format!(
                "unknown snapshot `{}`",
                identity.as_str()
            )))
        }
    }
}

impl crate::adapter::TruthBranchHeadSource for MisbindingSource {
    fn load_branch_head_patch(
        &self,
        branch_identity: &TruthBranchIdentity,
    ) -> Result<RawCommittedPatchEnvelope, crate::adapter::RelationalBridgeSourceError> {
        StaticSource.load_branch_head_patch(branch_identity)
    }
}

#[derive(Clone)]
pub(crate) struct WrongBranchHeadSource;

impl crate::adapter::CommittedPatchSource for WrongBranchHeadSource {
    fn load_committed_patch(
        &self,
        request: crate::adapter::RelationalCommittedPatchRequest,
    ) -> Result<RawCommittedPatchEnvelope, crate::adapter::RelationalBridgeSourceError> {
        StaticSource.load_committed_patch(request)
    }
}

impl crate::adapter::SnapshotReadSource for WrongBranchHeadSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, crate::adapter::RelationalBridgeSourceError> {
        StaticSource.open_snapshot(identity)
    }
}

impl crate::adapter::TruthBranchHeadSource for WrongBranchHeadSource {
    fn load_branch_head_patch(
        &self,
        _branch_identity: &TruthBranchIdentity,
    ) -> Result<RawCommittedPatchEnvelope, crate::adapter::RelationalBridgeSourceError> {
        Ok(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new("head-wrong"),
            TruthPatchIdentity::new("patch-wrong"),
            TruthSnapshotIdentity::new("snapshot-a"),
            TruthBranchIdentity::new("wrong-branch"),
            vec![BridgeCommittedPatchItem::new("entity-1", "profile", "name")],
        ))
    }
}

pub(crate) fn runtime(policy: BridgeRuntimePolicy) -> RuntimeBridge {
    RuntimeBridgeBuilder::new()
        .with_policy(policy)
        .with_relational_source(StaticSource)
        .with_truth_branch_head_source(StaticSource)
        .with_signal_sink(StaticSink)
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("mapping"),
            TruthPatchScope::new(
                MappingSelector::exact("entity-1"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
            ),
            SignalInvalidationScope::new("signal:profile"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("runtime should build")
}

pub(crate) fn runtime_with_sources<S>(policy: BridgeRuntimePolicy, source: S) -> RuntimeBridge
where
    S: crate::adapter::CommittedPatchSource
        + crate::adapter::SnapshotReadSource
        + crate::adapter::TruthBranchHeadSource
        + Clone,
{
    RuntimeBridgeBuilder::new()
        .with_policy(policy)
        .with_relational_source(source.clone())
        .with_truth_branch_head_source(source)
        .with_signal_sink(StaticSink)
        .register_mapping(BridgeMappingRegistration::new(
            BridgeMappingId::new("mapping"),
            TruthPatchScope::new(
                MappingSelector::exact("entity-1"),
                MappingSelector::exact("profile"),
                MappingSelector::exact("name"),
            ),
            SignalInvalidationScope::new("signal:profile"),
            CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("runtime should build")
}

pub(crate) fn detail_subscription(
    runtime: &RuntimeBridge,
) -> crate::facade::BridgeSubscriptionDeclaration {
    runtime
        .declare_subscription(
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            vec![NormalizedSubscriptionSliceIntent::try_new(
                "entity-1",
                "profile",
                "name",
                SubscriptionSliceKind::SignalField,
            )
            .expect("slice intent should validate")],
            BridgeSubscriptionDeliveryIntentClass::None,
        )
        .expect("detail declaration should succeed")
}

pub(crate) fn collection_subscription(
    runtime: &RuntimeBridge,
) -> crate::facade::BridgeSubscriptionDeclaration {
    runtime
        .declare_subscription(
            BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
            vec![
                NormalizedSubscriptionSliceIntent::try_new(
                    "entity-1",
                    "profile",
                    "west",
                    SubscriptionSliceKind::SignalRegion,
                )
                .expect("region slice intent should validate"),
                NormalizedSubscriptionSliceIntent::try_new(
                    "entity-1",
                    "profile",
                    "west-partition",
                    SubscriptionSliceKind::SignalPartition,
                )
                .expect("partition slice intent should validate"),
            ],
            BridgeSubscriptionDeliveryIntentClass::CanonicalMeaningfulChange,
        )
        .expect("collection declaration should succeed")
}

pub(crate) fn activation_ready_for(
    runtime: &RuntimeBridge,
    declaration: &crate::facade::BridgeSubscriptionDeclaration,
) -> crate::facade::BridgeSubscriptionActivationReady {
    let admitted = runtime
        .admit_subscription(
            declaration,
            BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new("snapshot-a")),
        )
        .expect("subscription admission should succeed");
    runtime.prepare_subscription_activation(&admitted)
}

pub(crate) fn canonical_consumer(runtime: &RuntimeBridge) -> BridgeSubscriptionConsumerContract {
    runtime
        .admit_subscription_consumer_contract(
            BridgeSubscriptionConsumerContractFamily::CanonicalDelivery,
            BridgeSubscriptionConsumerPacingCapability::Immediate,
            BridgeSubscriptionConsumerBackpressurePosture::PacingOnly,
            true,
            BridgeSubscriptionConsumerDiagnosticsRetention::MinimalReference,
        )
        .expect("canonical consumer should admit")
}

pub(crate) fn active_subscription_for(
    runtime: &RuntimeBridge,
    declaration: &crate::facade::BridgeSubscriptionDeclaration,
    posture: BridgeSubscriptionDeliveryDensityPosture,
    max_fanout_width: usize,
) -> crate::facade::BridgeActiveSubscription {
    let ready = activation_ready_for(runtime, declaration);
    let cost_profile = runtime
        .admit_subscription_delivery_cost_profile(posture, 4, 4, max_fanout_width)
        .expect("cost profile should admit");
    runtime.activate_subscription_delivery(ready, cost_profile, canonical_consumer(runtime))
}

pub(crate) fn fixture_members(count: usize) -> Vec<BridgeSubscriptionDeliveryMemberInput> {
    (0..count)
        .map(|index| {
            BridgeSubscriptionDeliveryMemberInput::payload_digest(
                "slice:entity-1/profile/name",
                format!("routing:harness:{index}"),
                BridgeSubscriptionDeliveryMemberClass::Update,
                format!("payload:harness:{index}"),
            )
        })
        .collect()
}

pub(crate) fn sealed_window_with_members(
    runtime: &RuntimeBridge,
    active: &crate::facade::BridgeActiveSubscription,
    family_kind: BridgeSubscriptionDeliveryFamilyKind,
    sequence: u64,
    members: Vec<BridgeSubscriptionDeliveryMemberInput>,
) -> crate::facade::BridgeSubscriptionDeliveryWindowSealed {
    let open = runtime.open_subscription_delivery_window(active, family_kind, sequence);
    runtime
        .seal_subscription_delivery_window(open, members)
        .expect("delivery window should seal")
}

pub(crate) fn checkpoint_from_sealed(
    runtime: &RuntimeBridge,
    active: &crate::facade::BridgeActiveSubscription,
    sealed: &crate::facade::BridgeSubscriptionDeliveryWindowSealed,
    acknowledged_sequence: usize,
    duplicate_policy: BridgeSubscriptionDuplicateReplayPolicyKind,
) -> crate::facade::BridgeSubscriptionCheckpoint {
    let acknowledged = &sealed.members()[acknowledged_sequence];
    let frontier = runtime
        .admit_subscription_acknowledgement_frontier(
            sealed,
            acknowledged_sequence,
            acknowledged.delivery_member_identity(),
            acknowledged.digest(),
        )
        .expect("acknowledgement frontier should admit");
    let ready = runtime.prepare_subscription_checkpoint(frontier);
    runtime
        .publish_subscription_checkpoint(ready, active, duplicate_policy)
        .expect("checkpoint should publish")
}

pub(crate) fn preview_declaration(suffix: &str) -> BridgePreviewSessionDeclaration {
    BridgePreviewSessionDeclaration::new(
        BridgePreviewSessionDeclarationIdentity::new(format!("preview:subscription:{suffix}")),
        BridgeRequestKind::Preview,
        BridgeSpeculativeBranchBinding::new(
            BridgeSpeculativeBranchBindingIdentity::new(format!(
                "preview-binding:subscription:{suffix}"
            )),
            TruthBranchIdentity::new("analysis"),
            BridgeSignalBranchIdentity::new(format!("signal:subscription:{suffix}")),
        ),
        "truth-view:subscription-preview",
        format!("source-capability:subscription:{suffix}"),
        format!("request-shape:subscription:{suffix}"),
        format!("retained-artifact-schema:subscription:{suffix}"),
    )
}

pub(crate) fn preview_active_subscription_for(
    runtime: &RuntimeBridge,
    suffix: &str,
    declaration: &crate::facade::BridgeSubscriptionDeclaration,
) -> crate::facade::BridgePreviewActiveSubscription {
    let ready = activation_ready_for(runtime, declaration);
    let admitted_preview = runtime
        .admit_preview_session(
            BridgePreviewSessionIdentity::new(format!("preview-session:subscription:{suffix}")),
            preview_declaration(suffix),
        )
        .expect("preview session should admit");
    let (active_preview_session, execution_record) =
        runtime.activate_preview_session(admitted_preview, 3, 1, 2);
    let preview_basis = runtime
        .admit_subscription_preview_basis(&active_preview_session, &execution_record)
        .expect("preview basis should admit");
    let cost_profile = runtime
        .admit_subscription_delivery_cost_profile(
            BridgeSubscriptionDeliveryDensityPosture::SparseMemberDelivery,
            4,
            4,
            1,
        )
        .expect("cost profile should admit");
    runtime.activate_preview_subscription_delivery(
        ready,
        preview_basis,
        cost_profile,
        canonical_consumer(runtime),
    )
}
