use crate::facade::{
    BridgeRuntimePolicy, BridgeSignalStrategyKind, BridgeSubscriptionAdmissionRejectionKind,
    BridgeSubscriptionBasisKind, BridgeSubscriptionBasisRequest,
    BridgeSubscriptionBasisResolutionFailureKind, BridgeSubscriptionDeclarationFamilyKind,
    BridgeSubscriptionDeliveryIntentClass, BridgeSubscriptionReplayMismatchKind,
    NormalizedSubscriptionSliceIntent, RuntimeBridge,
};
use crate::builder::RuntimeBridgeBuilder;
use crate::input::envelope::{
    BridgeCommittedPatchItem, RawCommittedPatchEnvelope, TruthBranchIdentity,
    TruthCommitIdentity, TruthPatchIdentity,
};
use crate::mapping::{
    BridgeMappingId, BridgeMappingRegistration, CoarseRoutingMode, MappingSelector,
    SignalInvalidationScope, SubscriptionSliceKind, TruthPatchScope,
};
use crate::snapshot::{SnapshotReadPacket, TruthSnapshotIdentity, TruthSnapshotReader};

#[derive(Clone)]
struct StaticSource;

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
struct MisbindingSource;

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
struct WrongBranchHeadSource;

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

fn runtime(policy: BridgeRuntimePolicy) -> RuntimeBridge {
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

fn runtime_with_sources<S>(policy: BridgeRuntimePolicy, source: S) -> RuntimeBridge
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

fn detail_subscription(runtime: &RuntimeBridge) -> crate::facade::BridgeSubscriptionDeclaration {
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

#[test]
fn bridge_harness_subscription_suite_28_declaration_equivalence_is_canonical_and_policy_invariant() {
    let development = runtime(BridgeRuntimePolicy::development());
    let forensic = runtime(BridgeRuntimePolicy::forensic());

    let left = development
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
            BridgeSubscriptionDeliveryIntentClass::CanonicalMeaningfulChange,
        )
        .expect("left declaration should succeed");
    let right = forensic
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
        .expect("right declaration should succeed");

    assert_eq!(left.digest(), right.digest());
    assert_eq!(left.declaration_identity(), right.declaration_identity());
    assert_eq!(
        development.subscription_family_registry_identity(),
        forensic.subscription_family_registry_identity()
    );
    assert_eq!(left.normalized_slice_intent_count(), 2);
    assert_eq!(left.counters().family_lookup_count(), 1);
    assert_eq!(left.counters().declaration_count(), 1);
    assert_eq!(left.counters().declaration_rejection_count(), 0);
    assert_eq!(left.counters().basis_request_count(), 0);
    assert_eq!(left.counters().signal_strategy_selection_count(), 0);

    let left_admitted = development
        .admit_subscription(
            &left,
            BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new("snapshot-a")),
        )
        .expect("left admission should succeed");
    let right_admitted = forensic
        .admit_subscription(
            &right,
            BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new("snapshot-a")),
        )
        .expect("right admission should succeed");

    assert_eq!(left_admitted.digest(), right_admitted.digest());
    assert_eq!(
        left_admitted.basis_binding().digest(),
        right_admitted.basis_binding().digest()
    );
    assert_eq!(
        left_admitted.signal_strategy().digest(),
        right_admitted.signal_strategy().digest()
    );

    let different_family = detail_subscription(&development);
    assert_ne!(left.digest(), different_family.digest());
}

#[test]
fn bridge_harness_subscription_suite_29_basis_binding_is_explicit_and_fail_closed() {
    let baseline = runtime(BridgeRuntimePolicy::development());
    let declaration = detail_subscription(&baseline);

    let snapshot_admitted = baseline
        .admit_subscription(
            &declaration,
            BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new("snapshot-a")),
        )
        .expect("snapshot admission should succeed");
    let branch_admitted = baseline
        .admit_subscription(
            &declaration,
            BridgeSubscriptionBasisRequest::branch_head(TruthBranchIdentity::new("analysis")),
        )
        .expect("branch-head admission should succeed");

    assert_eq!(
        snapshot_admitted.basis_binding().basis_kind(),
        BridgeSubscriptionBasisKind::Snapshot
    );
    assert_eq!(
        branch_admitted.basis_binding().basis_kind(),
        BridgeSubscriptionBasisKind::BranchHead
    );
    assert_ne!(
        snapshot_admitted.basis_binding().digest(),
        branch_admitted.basis_binding().digest()
    );
    assert_ne!(snapshot_admitted.digest(), branch_admitted.digest());
    assert_eq!(
        snapshot_admitted.signal_strategy().strategy_kind(),
        BridgeSignalStrategyKind::ExactFieldLensObservation
    );
    assert_eq!(snapshot_admitted.counters().basis_request_count(), 1);
    assert_eq!(snapshot_admitted.counters().basis_binding_count(), 1);
    assert_eq!(snapshot_admitted.counters().basis_rejection_count(), 0);
    assert_eq!(snapshot_admitted.counters().signal_strategy_selection_count(), 1);

    let missing_snapshot = baseline
        .admit_subscription(
            &declaration,
            BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new(
                "snapshot-missing",
            )),
        )
        .expect_err("missing snapshot should reject");
    assert_eq!(
        missing_snapshot.rejection_kind(),
        BridgeSubscriptionAdmissionRejectionKind::BasisResolutionFailure
    );
    assert_eq!(
        missing_snapshot.basis_resolution_failure_kind(),
        Some(BridgeSubscriptionBasisResolutionFailureKind::SnapshotAcquisitionFailure)
    );

    let misbound_runtime = runtime_with_sources(BridgeRuntimePolicy::development(), MisbindingSource);
    let misbound_declaration = detail_subscription(&misbound_runtime);
    let snapshot_mismatch = misbound_runtime
        .admit_subscription(
            &misbound_declaration,
            BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new("snapshot-a")),
        )
        .expect_err("snapshot identity mismatch should reject");
    assert_eq!(
        snapshot_mismatch.basis_resolution_failure_kind(),
        Some(BridgeSubscriptionBasisResolutionFailureKind::SnapshotIdentityMismatch)
    );

    let wrong_branch_runtime =
        runtime_with_sources(BridgeRuntimePolicy::development(), WrongBranchHeadSource);
    let wrong_branch_declaration = detail_subscription(&wrong_branch_runtime);
    let branch_mismatch = wrong_branch_runtime
        .admit_subscription(
            &wrong_branch_declaration,
            BridgeSubscriptionBasisRequest::branch_head(TruthBranchIdentity::new("analysis")),
        )
        .expect_err("branch mismatch should reject");
    assert_eq!(
        branch_mismatch.basis_resolution_failure_kind(),
        Some(BridgeSubscriptionBasisResolutionFailureKind::BranchHeadMismatch)
    );
    assert_eq!(branch_mismatch.counters().basis_request_count(), 1);
    assert_eq!(branch_mismatch.counters().basis_binding_count(), 0);
    assert_eq!(branch_mismatch.counters().basis_rejection_count(), 1);
    assert_eq!(branch_mismatch.counters().signal_strategy_selection_count(), 0);
}

#[test]
fn bridge_harness_subscription_suite_30_lifecycle_replay_parity_is_canonical() {
    let development = runtime(BridgeRuntimePolicy::development());
    let forensic = runtime(BridgeRuntimePolicy::forensic());

    let development_declaration = detail_subscription(&development);
    let forensic_declaration = detail_subscription(&forensic);

    let development_admitted = development
        .admit_subscription(
            &development_declaration,
            BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new("snapshot-a")),
        )
        .expect("development admission should succeed");
    let forensic_admitted = forensic
        .admit_subscription(
            &forensic_declaration,
            BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new("snapshot-a")),
        )
        .expect("forensic admission should succeed");

    let development_ready = development.prepare_subscription_activation(&development_admitted);
    let forensic_ready = forensic.prepare_subscription_activation(&forensic_admitted);

    assert_eq!(
        development_ready.lifecycle_record().digest(),
        forensic_ready.lifecycle_record().digest()
    );
    assert_eq!(
        development_ready.retained_bundle().digest(),
        forensic_ready.retained_bundle().digest()
    );

    let development_replay = development
        .replay_subscription(development_ready.retained_bundle())
        .expect("development replay should succeed");
    let forensic_replay = forensic
        .replay_subscription(forensic_ready.retained_bundle())
        .expect("forensic replay should succeed");

    assert_eq!(development_replay.digest(), forensic_replay.digest());
    assert_eq!(development_replay.counters().replay_reconstruction_count(), 1);
    assert_eq!(development_replay.counters().basis_request_count(), 0);
    assert_eq!(development_replay.counters().signal_strategy_selection_count(), 0);

    let deactivated = development.deactivate_subscription(development_ready);
    assert_ne!(
        deactivated.lifecycle_record().digest(),
        forensic_ready.lifecycle_record().digest()
    );
    assert_eq!(
        deactivated.lifecycle_record().state_kind(),
        crate::facade::BridgeSubscriptionLifecycleStateKind::Deactivated
    );

    let mismatch = crate::facade::BridgeSubscriptionReplaySummary::replay(
        &crate::facade::BridgeSubscriptionFamilyRegistryIdentity::new("registry-drift"),
        forensic_ready.retained_bundle(),
    )
    .expect_err("registry drift should reject");
    assert_eq!(
        mismatch.mismatch_kind(),
        BridgeSubscriptionReplayMismatchKind::RegistryIdentityMismatch
    );
    assert_eq!(mismatch.counters().replay_mismatch_count(), 1);
}
