use crate::facade::{
    BridgeSignalStrategyKind, BridgeSubscriptionAdmissionRejectionKind,
    BridgeSubscriptionBasisKind, BridgeSubscriptionBasisRequest,
    BridgeSubscriptionBasisResolutionFailureKind, BridgeSubscriptionDeclarationFamilyKind,
    BridgeSubscriptionDeliveryIntentClass,
    NormalizedSubscriptionSliceIntent,
};
use crate::input::envelope::{
    BridgeCommittedPatchItem, RawCommittedPatchEnvelope, TruthCommitIdentity, TruthPatchIdentity,
};
use crate::input::envelope::TruthBranchIdentity;
use crate::mapping::SubscriptionSliceKind;
use crate::policy::BridgeRuntimePolicy;
use crate::snapshot::{SnapshotReadPacket, TruthSnapshotIdentity, TruthSnapshotReader};

use super::runtime;

#[derive(Clone)]
struct MisbindingSource;

#[derive(Clone)]
struct WrongBranchHeadSource;

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
            Ok(Box::new(super::StaticSnapshotReader))
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

#[test]
fn runtime_exposes_stable_subscription_family_registry_identity() {
    let left = runtime(BridgeRuntimePolicy::development());
    let right = runtime(BridgeRuntimePolicy::development());

    assert_eq!(
        left.subscription_family_registry_identity(),
        right.subscription_family_registry_identity()
    );
}

#[test]
fn runtime_declares_detail_exact_subscription() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = runtime
        .declare_subscription(
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            vec![
                NormalizedSubscriptionSliceIntent::try_new(
                    "entity-1",
                    "profile",
                    "name",
                    SubscriptionSliceKind::SignalField,
                )
                .expect("slice intent should validate"),
                NormalizedSubscriptionSliceIntent::try_new(
                    "entity-1",
                    "profile",
                    "display_name",
                    SubscriptionSliceKind::SignalLens,
                )
                .expect("slice intent should validate"),
            ],
            BridgeSubscriptionDeliveryIntentClass::None,
        )
        .expect("detail subscription should declare");

    assert_eq!(
        declaration.requested_family_kind(),
        BridgeSubscriptionDeclarationFamilyKind::DetailExact
    );
    assert_eq!(declaration.normalized_slice_intent_count(), 2);
}

#[test]
fn runtime_declares_collection_membership_subscription() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let declaration = runtime
        .declare_subscription(
            BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
            vec![
                NormalizedSubscriptionSliceIntent::try_new(
                    "entity-1",
                    "profile",
                    "west",
                    SubscriptionSliceKind::SignalRegion,
                )
                .expect("slice intent should validate"),
                NormalizedSubscriptionSliceIntent::try_new(
                    "entity-1",
                    "profile",
                    "west-partition",
                    SubscriptionSliceKind::SignalPartition,
                )
                .expect("slice intent should validate"),
            ],
            BridgeSubscriptionDeliveryIntentClass::CanonicalMeaningfulChange,
        )
        .expect("collection subscription should declare");

    assert_eq!(
        declaration.requested_family_kind(),
        BridgeSubscriptionDeclarationFamilyKind::CollectionMembership
    );
    assert_eq!(declaration.normalized_slice_intent_count(), 2);
}

#[test]
fn runtime_rejects_unsupported_family_slice_combinations() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let rejection = runtime
        .declare_subscription(
            BridgeSubscriptionDeclarationFamilyKind::DetailExact,
            vec![NormalizedSubscriptionSliceIntent::try_new(
                "entity-1",
                "profile",
                "west",
                SubscriptionSliceKind::SignalRegion,
            )
            .expect("slice intent should validate")],
            BridgeSubscriptionDeliveryIntentClass::None,
        )
        .expect_err("detail subscriptions must reject region slices");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionDeclarationRejectionKind::UnsupportedSliceKindForFamily
    );
}

#[test]
fn runtime_declares_equivalent_subscriptions_canonically() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let left = runtime
        .declare_subscription(
            BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
            vec![
                NormalizedSubscriptionSliceIntent::try_new(
                    "entity-1",
                    "profile",
                    "west-partition",
                    SubscriptionSliceKind::SignalPartition,
                )
                .expect("slice intent should validate"),
                NormalizedSubscriptionSliceIntent::try_new(
                    "entity-1",
                    "profile",
                    "west",
                    SubscriptionSliceKind::SignalRegion,
                )
                .expect("slice intent should validate"),
                NormalizedSubscriptionSliceIntent::try_new(
                    "entity-1",
                    "profile",
                    "west",
                    SubscriptionSliceKind::SignalRegion,
                )
                .expect("slice intent should validate"),
            ],
            BridgeSubscriptionDeliveryIntentClass::None,
        )
        .expect("first subscription should declare");
    let right = runtime
        .declare_subscription(
            BridgeSubscriptionDeclarationFamilyKind::CollectionMembership,
            vec![
                NormalizedSubscriptionSliceIntent::try_new(
                    "entity-1",
                    "profile",
                    "west",
                    SubscriptionSliceKind::SignalRegion,
                )
                .expect("slice intent should validate"),
                NormalizedSubscriptionSliceIntent::try_new(
                    "entity-1",
                    "profile",
                    "west-partition",
                    SubscriptionSliceKind::SignalPartition,
                )
                .expect("slice intent should validate"),
            ],
            BridgeSubscriptionDeliveryIntentClass::None,
        )
        .expect("second subscription should declare");

    assert_eq!(left, right);
    assert_eq!(left.digest(), right.digest());
}

#[test]
fn runtime_exposes_subscription_registry_counters() {
    let runtime = runtime(BridgeRuntimePolicy::development());
    let counters = runtime.subscription_family_registry_counters();

    assert_eq!(counters.family_registry_freeze_count(), 1);
    assert_eq!(counters.family_count(), 2);
    assert_eq!(counters.family_supported_slice_kind_count(), 4);
}

#[test]
fn runtime_admits_detail_exact_subscription_against_current_snapshot_basis() {
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
            BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new(
                "snapshot-a",
            )),
        )
        .expect("admission should succeed");

    assert_eq!(
        admitted.basis_binding().basis_kind(),
        BridgeSubscriptionBasisKind::Snapshot
    );
    assert_eq!(
        admitted.signal_strategy().strategy_kind(),
        BridgeSignalStrategyKind::ExactFieldLensObservation
    );
    assert_eq!(admitted.counters().admitted_subscription_count(), 1);
}

#[test]
fn runtime_admits_collection_membership_subscription_against_branch_head_basis() {
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
            BridgeSubscriptionBasisRequest::branch_head(TruthBranchIdentity::new("analysis")),
        )
        .expect("branch-head admission should succeed");

    assert_eq!(
        admitted.basis_binding().basis_kind(),
        BridgeSubscriptionBasisKind::BranchHead
    );
    assert_eq!(
        admitted
            .basis_binding()
            .commit_identity()
            .expect("branch head should resolve commit")
            .as_str(),
        "head-analysis"
    );
    assert_eq!(
        admitted.signal_strategy().strategy_kind(),
        BridgeSignalStrategyKind::CollectionMembershipObservation
    );
}

#[test]
fn runtime_rejects_subscription_admission_when_snapshot_basis_cannot_bind() {
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

    let rejection = runtime
        .admit_subscription(
            &declaration,
            BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new(
                "snapshot-missing",
            )),
        )
        .expect_err("missing snapshot basis should reject");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionAdmissionRejectionKind::BasisResolutionFailure
    );
    assert_eq!(
        rejection.basis_resolution_failure_kind(),
        Some(
            crate::facade::BridgeSubscriptionBasisResolutionFailureKind::SnapshotAcquisitionFailure
        )
    );
    assert_eq!(rejection.counters().basis_rejection_count(), 1);
    assert_eq!(rejection.counters().signal_strategy_rejection_count(), 0);

    let explanation = runtime.inspect_subscription_admission_rejection(&rejection);
    assert_eq!(
        explanation.admission_rejection_kind(),
        Some(BridgeSubscriptionAdmissionRejectionKind::BasisResolutionFailure)
    );
    assert_eq!(
        explanation.basis_resolution_failure_kind(),
        Some(BridgeSubscriptionBasisResolutionFailureKind::SnapshotAcquisitionFailure)
    );
}

#[test]
fn runtime_rejects_subscription_admission_when_snapshot_reader_misbinds_identity() {
    let runtime = crate::builder::RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::development())
        .with_relational_source(MisbindingSource)
        .with_truth_branch_head_source(MisbindingSource)
        .with_signal_sink(super::StaticSink)
        .register_mapping(crate::mapping::BridgeMappingRegistration::new(
            crate::mapping::BridgeMappingId::new("mapping"),
            crate::mapping::TruthPatchScope::new(
                crate::mapping::MappingSelector::exact("entity-1"),
                crate::mapping::MappingSelector::exact("profile"),
                crate::mapping::MappingSelector::exact("name"),
            ),
            crate::mapping::SignalInvalidationScope::new("signal:profile"),
            crate::mapping::CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("runtime should build");
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

    let rejection = runtime
        .admit_subscription(
            &declaration,
            BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new("snapshot-a")),
        )
        .expect_err("misbound snapshot reader should reject");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionAdmissionRejectionKind::BasisResolutionFailure
    );
    assert_eq!(
        rejection.basis_resolution_failure_kind(),
        Some(
            crate::facade::BridgeSubscriptionBasisResolutionFailureKind::SnapshotIdentityMismatch
        )
    );
    assert_eq!(rejection.counters().basis_rejection_count(), 1);
    assert_eq!(rejection.counters().signal_strategy_rejection_count(), 0);
}

#[test]
fn runtime_rejects_subscription_admission_when_branch_head_source_misbinds_branch() {
    let runtime = crate::builder::RuntimeBridgeBuilder::new()
        .with_policy(BridgeRuntimePolicy::development())
        .with_relational_source(WrongBranchHeadSource)
        .with_truth_branch_head_source(WrongBranchHeadSource)
        .with_signal_sink(super::StaticSink)
        .register_mapping(crate::mapping::BridgeMappingRegistration::new(
            crate::mapping::BridgeMappingId::new("mapping"),
            crate::mapping::TruthPatchScope::new(
                crate::mapping::MappingSelector::exact("entity-1"),
                crate::mapping::MappingSelector::exact("profile"),
                crate::mapping::MappingSelector::exact("name"),
            ),
            crate::mapping::SignalInvalidationScope::new("signal:profile"),
            crate::mapping::CoarseRoutingMode::Direct,
        ))
        .build()
        .expect("runtime should build");
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

    let rejection = runtime
        .admit_subscription(
            &declaration,
            BridgeSubscriptionBasisRequest::branch_head(TruthBranchIdentity::new("analysis")),
        )
        .expect_err("branch mismatch should reject");

    assert_eq!(
        rejection.rejection_kind(),
        crate::facade::BridgeSubscriptionAdmissionRejectionKind::BasisResolutionFailure
    );
    assert_eq!(
        rejection.basis_resolution_failure_kind(),
        Some(crate::facade::BridgeSubscriptionBasisResolutionFailureKind::BranchHeadMismatch)
    );
    assert_eq!(rejection.counters().basis_rejection_count(), 1);
    assert_eq!(rejection.counters().signal_strategy_rejection_count(), 0);
}

#[test]
fn runtime_prepares_and_inspects_activation_ready_subscription() {
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
    let explanation = runtime.inspect_activation_ready_subscription(&ready);

    assert_eq!(
        ready.lifecycle_record().state_kind(),
        crate::facade::BridgeSubscriptionLifecycleStateKind::ActivationReady
    );
    assert_eq!(ready.counters().lifecycle_record_count(), 1);
    assert_eq!(
        explanation.signal_strategy_kind(),
        Some(BridgeSignalStrategyKind::ExactFieldLensObservation)
    );
    assert_eq!(
        explanation.lifecycle_state_kind(),
        Some(crate::facade::BridgeSubscriptionLifecycleStateKind::ActivationReady)
    );
    assert_eq!(explanation.counters().diagnostics_bundle_count(), 1);
}

#[test]
fn runtime_deactivates_and_replays_retained_subscription_bundle() {
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
            BridgeSubscriptionBasisRequest::branch_head(TruthBranchIdentity::new("analysis")),
        )
        .expect("admission should succeed");
    let ready = runtime.prepare_subscription_activation(&admitted);
    let replay = runtime
        .replay_subscription(ready.retained_bundle())
        .expect("replay should succeed");
    let deactivated = runtime.deactivate_subscription(ready);
    let explanation = runtime.inspect_deactivated_subscription(&deactivated);

    assert_eq!(replay.counters().replay_reconstruction_count(), 1);
    assert_eq!(
        explanation.lifecycle_state_kind(),
        Some(crate::facade::BridgeSubscriptionLifecycleStateKind::Deactivated)
    );
    assert_eq!(
        deactivated.lifecycle_record().state_kind(),
        crate::facade::BridgeSubscriptionLifecycleStateKind::Deactivated
    );
}

#[test]
fn runtime_replay_rejects_retained_bundle_when_registry_identity_drifts() {
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

    let mismatch = crate::facade::BridgeSubscriptionReplaySummary::replay(
        &crate::facade::BridgeSubscriptionFamilyRegistryIdentity::new("registry-drift"),
        ready.retained_bundle(),
    )
    .expect_err("registry drift should reject");

    assert_eq!(
        mismatch.mismatch_kind(),
        crate::facade::BridgeSubscriptionReplayMismatchKind::RegistryIdentityMismatch
    );
    assert_eq!(mismatch.counters().replay_mismatch_count(), 1);
}
