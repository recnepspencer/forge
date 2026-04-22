use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    AdmittedBridgeSubscription, BridgeSubscriptionCounters, BridgeSubscriptionLifecycleRecord,
    BridgeSubscriptionReplayIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeRetainedSubscriptionBundle {
    registry_identity: super::BridgeSubscriptionFamilyRegistryIdentity,
    declaration: super::BridgeSubscriptionDeclaration,
    admitted: AdmittedBridgeSubscription,
    lifecycle_record: BridgeSubscriptionLifecycleRecord,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeRetainedSubscriptionBundle {
    pub(crate) fn new(
        registry_identity: super::BridgeSubscriptionFamilyRegistryIdentity,
        admitted: &AdmittedBridgeSubscription,
        lifecycle_record: BridgeSubscriptionLifecycleRecord,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-retained-subscription-bundle|registry={}|declaration={}|admitted={}|lifecycle={}",
            registry_identity.as_str(),
            admitted.declaration().digest(),
            admitted.digest(),
            lifecycle_record.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            registry_identity,
            declaration: admitted.declaration().clone(),
            admitted: admitted.clone(),
            lifecycle_record,
            counters: BridgeSubscriptionCounters::from_lifecycle_record(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-retained-subscription-bundle:sha256:{digest:x}"
            )),
        }
    }

    pub fn registry_identity(&self) -> &super::BridgeSubscriptionFamilyRegistryIdentity {
        &self.registry_identity
    }

    pub fn declaration(&self) -> &super::BridgeSubscriptionDeclaration {
        &self.declaration
    }

    pub fn admitted(&self) -> &AdmittedBridgeSubscription {
        &self.admitted
    }

    pub fn lifecycle_record(&self) -> &BridgeSubscriptionLifecycleRecord {
        &self.lifecycle_record
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeSubscriptionReplayMismatchKind {
    RegistryIdentityMismatch,
    AdmittedDeclarationMismatch,
    LifecycleAdmittedMismatch,
}

impl BridgeSubscriptionReplayMismatchKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RegistryIdentityMismatch => "registry_identity_mismatch",
            Self::AdmittedDeclarationMismatch => "admitted_declaration_mismatch",
            Self::LifecycleAdmittedMismatch => "lifecycle_admitted_mismatch",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReplayMismatch {
    mismatch_kind: BridgeSubscriptionReplayMismatchKind,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionReplayMismatch {
    fn new(
        mismatch_kind: BridgeSubscriptionReplayMismatchKind,
        bundle: &BridgeRetainedSubscriptionBundle,
    ) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-replay-mismatch|kind={}|bundle={}",
            mismatch_kind.as_str(),
            bundle.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            mismatch_kind,
            counters: BridgeSubscriptionCounters::from_replay_mismatch(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-replay-mismatch:sha256:{digest:x}"
            )),
        }
    }

    pub fn mismatch_kind(&self) -> BridgeSubscriptionReplayMismatchKind {
        self.mismatch_kind
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionReplaySummary {
    replay_identity: BridgeSubscriptionReplayIdentity,
    retained_bundle: BridgeRetainedSubscriptionBundle,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionReplaySummary {
    pub(crate) fn replay(
        current_registry_identity: &super::BridgeSubscriptionFamilyRegistryIdentity,
        bundle: &BridgeRetainedSubscriptionBundle,
    ) -> Result<Self, BridgeSubscriptionReplayMismatch> {
        if bundle.registry_identity() != current_registry_identity {
            return Err(BridgeSubscriptionReplayMismatch::new(
                BridgeSubscriptionReplayMismatchKind::RegistryIdentityMismatch,
                bundle,
            ));
        }
        if bundle.declaration().declaration_identity()
            != bundle.admitted().declaration().declaration_identity()
        {
            return Err(BridgeSubscriptionReplayMismatch::new(
                BridgeSubscriptionReplayMismatchKind::AdmittedDeclarationMismatch,
                bundle,
            ));
        }
        if bundle.lifecycle_record().admitted_subscription_identity()
            != bundle.admitted().admitted_subscription_identity()
        {
            return Err(BridgeSubscriptionReplayMismatch::new(
                BridgeSubscriptionReplayMismatchKind::LifecycleAdmittedMismatch,
                bundle,
            ));
        }

        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-replay-summary|registry={}|bundle={}",
            current_registry_identity.as_str(),
            bundle.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Ok(Self {
            replay_identity: BridgeSubscriptionReplayIdentity::new(format!(
                "bridge-subscription-replay-id:sha256:{digest:x}"
            )),
            retained_bundle: bundle.clone(),
            counters: BridgeSubscriptionCounters::from_replay_reconstruction(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-replay-summary:sha256:{digest:x}"
            )),
        })
    }

    pub fn replay_identity(&self) -> &BridgeSubscriptionReplayIdentity {
        &self.replay_identity
    }

    pub fn retained_bundle(&self) -> &BridgeRetainedSubscriptionBundle {
        &self.retained_bundle
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use crate::facade::{
        BridgeRuntimePolicy, BridgeSubscriptionBasisRequest,
        BridgeSubscriptionDeclarationFamilyKind, BridgeSubscriptionDeliveryIntentClass,
        NormalizedSubscriptionSliceIntent,
    };
    use crate::input::envelope::{
        BridgeCommittedPatchItem, RawCommittedPatchEnvelope, TruthBranchIdentity,
        TruthCommitIdentity, TruthPatchIdentity,
    };
    use crate::mapping::SubscriptionSliceKind;
    use crate::snapshot::{SnapshotReadPacket, TruthSnapshotIdentity, TruthSnapshotReader};

    use super::{
        super::BridgeSubscriptionLifecycleRecord, super::BridgeSubscriptionLifecycleStateKind,
        BridgeRetainedSubscriptionBundle, BridgeSubscriptionReplayMismatchKind,
        BridgeSubscriptionReplaySummary,
    };

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
        ) -> Result<
            crate::snapshot::SnapshotReadPacketResult,
            crate::snapshot::BridgeSnapshotReadError,
        > {
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
        ) -> Result<RawCommittedPatchEnvelope, crate::adapter::RelationalBridgeSourceError>
        {
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
        ) -> Result<Box<dyn TruthSnapshotReader>, crate::adapter::RelationalBridgeSourceError>
        {
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
        ) -> Result<RawCommittedPatchEnvelope, crate::adapter::RelationalBridgeSourceError>
        {
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
        ) -> Result<crate::delivery::BridgeDeliveryReceipt, crate::adapter::SignalBridgeSinkError>
        {
            Ok(crate::delivery::BridgeDeliveryReceipt::new(
                delivery.invalidation_targets().len(),
                delivery.source_snapshot().clone(),
            ))
        }
    }

    fn runtime(policy: BridgeRuntimePolicy) -> crate::facade::RuntimeBridge {
        crate::builder::RuntimeBridgeBuilder::new()
            .with_policy(policy)
            .with_relational_source(StaticSource)
            .with_truth_branch_head_source(StaticSource)
            .with_signal_sink(StaticSink)
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
            .expect("runtime should build")
    }

    fn declare_detail(
        runtime: &crate::facade::RuntimeBridge,
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

    fn declare_collection(
        runtime: &crate::facade::RuntimeBridge,
    ) -> crate::facade::BridgeSubscriptionDeclaration {
        runtime
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
            .expect("collection declaration should succeed")
    }

    #[test]
    fn replay_rejects_admitted_declaration_mismatch() {
        let runtime = runtime(BridgeRuntimePolicy::development());
        let declaration = declare_detail(&runtime);
        let admitted = runtime
            .admit_subscription(
                &declaration,
                BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new("snapshot-a")),
            )
            .expect("admission should succeed");
        let lifecycle = BridgeSubscriptionLifecycleRecord::new(
            &admitted,
            BridgeSubscriptionLifecycleStateKind::ActivationReady,
        );

        let mut bundle = BridgeRetainedSubscriptionBundle::new(
            runtime.subscription_family_registry_identity().clone(),
            &admitted,
            lifecycle,
        );
        bundle.declaration = declare_collection(&runtime);

        let mismatch = BridgeSubscriptionReplaySummary::replay(
            runtime.subscription_family_registry_identity(),
            &bundle,
        )
        .expect_err("tampered declaration should reject");

        assert_eq!(
            mismatch.mismatch_kind(),
            BridgeSubscriptionReplayMismatchKind::AdmittedDeclarationMismatch
        );
    }

    #[test]
    fn replay_rejects_lifecycle_admitted_mismatch() {
        let runtime = runtime(BridgeRuntimePolicy::development());
        let detail = declare_detail(&runtime);
        let detail_admitted = runtime
            .admit_subscription(
                &detail,
                BridgeSubscriptionBasisRequest::snapshot(TruthSnapshotIdentity::new("snapshot-a")),
            )
            .expect("detail admission should succeed");
        let collection = declare_collection(&runtime);
        let collection_admitted = runtime
            .admit_subscription(
                &collection,
                BridgeSubscriptionBasisRequest::branch_head(TruthBranchIdentity::new("analysis")),
            )
            .expect("collection admission should succeed");

        let mismatched_lifecycle = BridgeSubscriptionLifecycleRecord::new(
            &collection_admitted,
            BridgeSubscriptionLifecycleStateKind::ActivationReady,
        );
        let bundle = BridgeRetainedSubscriptionBundle::new(
            runtime.subscription_family_registry_identity().clone(),
            &detail_admitted,
            mismatched_lifecycle,
        );

        let mismatch = BridgeSubscriptionReplaySummary::replay(
            runtime.subscription_family_registry_identity(),
            &bundle,
        )
        .expect_err("mismatched lifecycle should reject");

        assert_eq!(
            mismatch.mismatch_kind(),
            BridgeSubscriptionReplayMismatchKind::LifecycleAdmittedMismatch
        );
    }
}
