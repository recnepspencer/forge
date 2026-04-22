pub(crate) use crate::facade::{
    BridgeSignalStrategyKind, BridgeSubscriptionAdmissionRejectionKind,
    BridgeSubscriptionBasisKind, BridgeSubscriptionBasisRequest,
    BridgeSubscriptionBasisResolutionFailureKind, BridgeSubscriptionConsumerBackpressurePosture,
    BridgeSubscriptionConsumerContractFamily, BridgeSubscriptionConsumerDiagnosticsRetention,
    BridgeSubscriptionConsumerPacingCapability, BridgeSubscriptionDeclarationFamilyKind,
    BridgeSubscriptionDeliveryDensityPosture, BridgeSubscriptionDeliveryFamilyKind,
    BridgeSubscriptionDeliveryIntentClass, BridgeSubscriptionDeliveryMemberClass,
    BridgeSubscriptionDeliveryMemberInput, BridgeSubscriptionPayloadOmissionReason,
    NormalizedSubscriptionSliceIntent,
};
pub(crate) use crate::input::envelope::TruthBranchIdentity;
pub(crate) use crate::input::envelope::{
    BridgeCommittedPatchItem, RawCommittedPatchEnvelope, TruthCommitIdentity, TruthPatchIdentity,
};
pub(crate) use crate::mapping::SubscriptionSliceKind;
pub(crate) use crate::policy::BridgeRuntimePolicy;
pub(crate) use crate::snapshot::{SnapshotReadPacket, TruthSnapshotIdentity, TruthSnapshotReader};
pub(crate) use std::sync::Arc;

pub(crate) fn runtime(policy: BridgeRuntimePolicy) -> crate::facade::RuntimeBridge {
    super::super::runtime(policy)
}

#[derive(Clone)]
pub(crate) struct MisbindingSource;

#[derive(Clone)]
pub(crate) struct WrongBranchHeadSource;

pub(crate) struct MisbindingSnapshotReader;

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
        Ok(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new(request.commit_identity()),
            TruthPatchIdentity::new(format!("patch-for-{}", request.commit_identity())),
            TruthSnapshotIdentity::new("snapshot-a"),
            TruthBranchIdentity::new("analysis"),
            vec![BridgeCommittedPatchItem::new("entity-1", "profile", "name")],
        ))
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
        Ok(RawCommittedPatchEnvelope::new(
            TruthCommitIdentity::new(format!("head-{}", branch_identity.as_str())),
            TruthPatchIdentity::new(format!("patch-{}", branch_identity.as_str())),
            TruthSnapshotIdentity::new("snapshot-a"),
            branch_identity.clone(),
            vec![BridgeCommittedPatchItem::new("entity-1", "profile", "name")],
        ))
    }
}

impl crate::adapter::CommittedPatchSource for WrongBranchHeadSource {
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

impl crate::adapter::SnapshotReadSource for WrongBranchHeadSource {
    fn open_snapshot(
        &self,
        identity: &TruthSnapshotIdentity,
    ) -> Result<Box<dyn TruthSnapshotReader>, crate::adapter::RelationalBridgeSourceError> {
        if identity.as_str() == "snapshot-a" {
            Ok(Box::new(super::super::StaticSnapshotReader))
        } else {
            Err(crate::adapter::RelationalBridgeSourceError::new(format!(
                "unknown snapshot `{}`",
                identity.as_str()
            )))
        }
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

pub(crate) fn activation_ready_detail_subscription() -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgeSubscriptionActivationReady,
) {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = runtime
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
        .expect("declaration should succeed");
    let admitted = runtime
        .admit_subscription(
            &declaration,
            BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new("snapshot-a")),
        )
        .expect("admission should succeed");
    let ready = runtime.prepare_subscription_activation(&admitted);
    (runtime, ready)
}

pub(crate) fn activation_ready_collection_subscription() -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgeSubscriptionActivationReady,
) {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = runtime
        .declare_subscription(
            BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
            vec![NormalizedSubscriptionSliceIntent::try_new(
                "entity-1",
                "profile",
                "west",
                SubscriptionSliceKind::SignalRegion,
            )
            .expect("slice intent should validate")],
            BridgeSubscriptionDeliveryIntentClass::None,
        )
        .expect("declaration should succeed");
    let admitted = runtime
        .admit_subscription(
            &declaration,
            BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new("snapshot-a")),
        )
        .expect("admission should succeed");
    let ready = runtime.prepare_subscription_activation(&admitted);
    (runtime, ready)
}

pub(crate) fn preview_active_detail_subscription(
    suffix: &str,
) -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgePreviewActiveSubscription,
) {
    preview_active_subscription_from_ready(activation_ready_detail_subscription(), suffix)
}

pub(crate) fn preview_active_collection_subscription(
    suffix: &str,
) -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgePreviewActiveSubscription,
) {
    preview_active_subscription_from_ready(activation_ready_collection_subscription(), suffix)
}

pub(crate) fn preview_active_subscription_from_ready(
    (runtime, ready): (
        crate::facade::RuntimeBridge,
        crate::facade::BridgeSubscriptionActivationReady,
    ),
    suffix: &str,
) -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgePreviewActiveSubscription,
) {
    let admitted_preview = runtime
        .admit_preview_session(
            crate::facade::BridgePreviewSessionIdentity::new(format!("preview-session:{suffix}")),
            subscription_preview_declaration(suffix),
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
    let consumer = canonical_consumer_contract(&runtime);
    let preview_active = runtime.activate_preview_subscription_delivery(
        ready,
        preview_basis,
        cost_profile,
        consumer,
    );
    (runtime, preview_active)
}

pub(crate) fn active_detail_subscription(
    posture: BridgeSubscriptionDeliveryDensityPosture,
) -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgeActiveSubscription,
) {
    active_detail_subscription_with_fanout(posture, 1)
}

pub(crate) fn active_detail_subscription_with_fanout(
    posture: BridgeSubscriptionDeliveryDensityPosture,
    max_fanout_width: usize,
) -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgeActiveSubscription,
) {
    let (runtime, ready) = activation_ready_detail_subscription();
    let cost_profile = runtime
        .admit_subscription_delivery_cost_profile(posture, 4, 4, max_fanout_width)
        .expect("cost profile should admit");
    let consumer = canonical_consumer_contract(&runtime);
    let active = runtime.activate_subscription_delivery(ready, cost_profile, consumer);
    (runtime, active)
}

pub(crate) fn active_detail_subscription_with_member_limit(
    posture: BridgeSubscriptionDeliveryDensityPosture,
    max_member_count: usize,
) -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgeActiveSubscription,
) {
    let (runtime, ready) = activation_ready_detail_subscription();
    let cost_profile = runtime
        .admit_subscription_delivery_cost_profile(posture, max_member_count, max_member_count, 1)
        .expect("cost profile should admit");
    let consumer = canonical_consumer_contract(&runtime);
    let active = runtime.activate_subscription_delivery(ready, cost_profile, consumer);
    (runtime, active)
}

pub(crate) fn active_detail_subscription_with_consumer(
    posture: BridgeSubscriptionDeliveryDensityPosture,
    max_fanout_width: usize,
    consumer: crate::facade::BridgeSubscriptionConsumerContract,
) -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgeActiveSubscription,
) {
    let (runtime, ready) = activation_ready_detail_subscription();
    let cost_profile = runtime
        .admit_subscription_delivery_cost_profile(posture, 4, 4, max_fanout_width)
        .expect("cost profile should admit");
    let active = runtime.activate_subscription_delivery(ready, cost_profile, consumer);
    (runtime, active)
}

pub(crate) fn active_collection_subscription(
    posture: BridgeSubscriptionDeliveryDensityPosture,
) -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgeActiveSubscription,
) {
    active_collection_subscription_with_fanout(posture, 1)
}

pub(crate) fn active_collection_subscription_with_fanout(
    posture: BridgeSubscriptionDeliveryDensityPosture,
    max_fanout_width: usize,
) -> (
    crate::facade::RuntimeBridge,
    crate::facade::BridgeActiveSubscription,
) {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = runtime
        .declare_subscription(
            BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
            vec![NormalizedSubscriptionSliceIntent::try_new(
                "entity-1",
                "profile",
                "west",
                SubscriptionSliceKind::SignalRegion,
            )
            .expect("slice intent should validate")],
            BridgeSubscriptionDeliveryIntentClass::None,
        )
        .expect("declaration should succeed");
    let admitted = runtime
        .admit_subscription(
            &declaration,
            BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new("snapshot-a")),
        )
        .expect("admission should succeed");
    let ready = runtime.prepare_subscription_activation(&admitted);
    let cost_profile = runtime
        .admit_subscription_delivery_cost_profile(posture, 4, 4, max_fanout_width)
        .expect("cost profile should admit");
    let consumer = canonical_consumer_contract(&runtime);
    let active = runtime.activate_subscription_delivery(ready, cost_profile, consumer);
    (runtime, active)
}

pub(crate) fn canonical_consumer_contract(
    runtime: &crate::facade::RuntimeBridge,
) -> crate::facade::BridgeSubscriptionConsumerContract {
    runtime
        .admit_subscription_consumer_contract(
            BridgeSubscriptionConsumerContractFamily::CanonicalDelivery,
            BridgeSubscriptionConsumerPacingCapability::Immediate,
            BridgeSubscriptionConsumerBackpressurePosture::PacingOnly,
            true,
            BridgeSubscriptionConsumerDiagnosticsRetention::MinimalReference,
        )
        .expect("consumer contract should admit")
}

pub(crate) fn subscription_preview_declaration(
    suffix: &str,
) -> crate::facade::BridgePreviewSessionDeclaration {
    crate::facade::BridgePreviewSessionDeclaration::new(
        crate::facade::BridgePreviewSessionDeclarationIdentity::new(format!(
            "preview-declaration:{suffix}"
        )),
        crate::facade::BridgeRequestKind::Preview,
        crate::facade::BridgeSpeculativeBranchBinding::new(
            crate::facade::BridgeSpeculativeBranchBindingIdentity::new(format!(
                "preview-binding:{suffix}"
            )),
            TruthBranchIdentity::new(format!("truth-branch:{suffix}")),
            crate::facade::BridgeSignalBranchIdentity::new(format!("signal-branch:{suffix}")),
        ),
        format!("truth-view:{suffix}"),
        format!("source-capability:{suffix}"),
        format!("request-shape:{suffix}"),
        format!("artifact-schema:{suffix}"),
    )
}

pub(crate) fn zero_preview_residue_inputs(
    suffix: &str,
) -> Vec<crate::facade::BridgeSubscriptionPreviewResidueArtifactInput> {
    [
        crate::facade::BridgeSubscriptionPreviewResidueCategory::AuthoritativeTruthSubscription,
        crate::facade::BridgeSubscriptionPreviewResidueCategory::BridgeSubscriptionRegistry,
        crate::facade::BridgeSubscriptionPreviewResidueCategory::ActiveDelivery,
        crate::facade::BridgeSubscriptionPreviewResidueCategory::FanoutConsumerContract,
        crate::facade::BridgeSubscriptionPreviewResidueCategory::Continuation,
        crate::facade::BridgeSubscriptionPreviewResidueCategory::CheckpointReplay,
        crate::facade::BridgeSubscriptionPreviewResidueCategory::SignalVisible,
    ]
    .into_iter()
    .map(|category| {
        crate::facade::BridgeSubscriptionPreviewResidueArtifactInput::zero(
            category,
            format!("preview-residue-evidence:{suffix}:{}", category.as_str()),
        )
    })
    .collect()
}

pub(crate) fn preview_residue_inputs_with_count(
    suffix: &str,
    nonzero_category: crate::facade::BridgeSubscriptionPreviewResidueCategory,
    residue_count: usize,
) -> Vec<crate::facade::BridgeSubscriptionPreviewResidueArtifactInput> {
    zero_preview_residue_inputs(suffix)
        .into_iter()
        .map(|input| {
            if input.category() == nonzero_category {
                crate::facade::BridgeSubscriptionPreviewResidueArtifactInput::new(
                    nonzero_category,
                    residue_count,
                    format!(
                        "preview-residue-evidence:{suffix}:{}:nonzero",
                        nonzero_category.as_str()
                    ),
                )
            } else {
                input
            }
        })
        .collect()
}

pub(crate) fn sealed_window(
    runtime: &crate::facade::RuntimeBridge,
    active: &crate::facade::BridgeActiveSubscription,
    family_kind: BridgeSubscriptionDeliveryFamilyKind,
) -> crate::facade::BridgeSubscriptionDeliveryWindowSealed {
    sealed_window_with_member(
        runtime,
        active,
        family_kind,
        0,
        BridgeSubscriptionDeliveryMemberInput::payload_digest(
            "slice:entity-1/profile/name",
            "routing:fixture",
            BridgeSubscriptionDeliveryMemberClass::Update,
            "payload:fixture",
        ),
    )
}

pub(crate) fn sealed_window_with_member(
    runtime: &crate::facade::RuntimeBridge,
    active: &crate::facade::BridgeActiveSubscription,
    family_kind: BridgeSubscriptionDeliveryFamilyKind,
    delivery_window_sequence: u64,
    member: BridgeSubscriptionDeliveryMemberInput,
) -> crate::facade::BridgeSubscriptionDeliveryWindowSealed {
    let open =
        runtime.open_subscription_delivery_window(active, family_kind, delivery_window_sequence);
    runtime
        .seal_subscription_delivery_window(open, vec![member])
        .expect("delivery window should seal")
}

pub(crate) fn sealed_window_with_members(
    runtime: &crate::facade::RuntimeBridge,
    active: &crate::facade::BridgeActiveSubscription,
    family_kind: BridgeSubscriptionDeliveryFamilyKind,
    delivery_window_sequence: u64,
    members: Vec<BridgeSubscriptionDeliveryMemberInput>,
) -> crate::facade::BridgeSubscriptionDeliveryWindowSealed {
    let open =
        runtime.open_subscription_delivery_window(active, family_kind, delivery_window_sequence);
    runtime
        .seal_subscription_delivery_window(open, members)
        .expect("delivery window should seal")
}

pub(crate) fn checkpoint_from_sealed(
    runtime: &crate::facade::RuntimeBridge,
    active: &crate::facade::BridgeActiveSubscription,
    sealed: &crate::facade::BridgeSubscriptionDeliveryWindowSealed,
    acknowledged_sequence: usize,
    duplicate_replay_policy_kind: crate::facade::BridgeSubscriptionDuplicateReplayPolicyKind,
) -> crate::facade::BridgeSubscriptionCheckpoint {
    let acknowledged = &sealed.members()[acknowledged_sequence];
    let frontier = runtime
        .admit_subscription_acknowledgement_frontier(
            sealed,
            acknowledged_sequence,
            acknowledged.delivery_member_identity(),
            acknowledged.digest(),
        )
        .expect("frontier should admit");
    runtime
        .publish_subscription_checkpoint(
            runtime.prepare_subscription_checkpoint(frontier),
            active,
            duplicate_replay_policy_kind,
        )
        .expect("checkpoint should publish")
}
