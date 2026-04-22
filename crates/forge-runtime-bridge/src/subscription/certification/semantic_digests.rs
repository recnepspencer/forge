use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionBundleFieldState, BridgeSubscriptionCertificationAssemblyPlan,
    BridgeSubscriptionCertificationCostProfile, BridgeSubscriptionCertificationCounterSnapshot,
    BridgeSubscriptionCertificationScratch, BridgeSubscriptionCertificationSemanticSourceKind,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationSemanticDigests {
    subscription_digest: Arc<str>,
    subscription_registry_digest: Arc<str>,
    subscription_basis_digest: Arc<str>,
    subscription_lifecycle_digest: Arc<str>,
    subscription_delivery_digest: Arc<str>,
    subscription_share_digest: Arc<str>,
    subscription_continuation_digest: Arc<str>,
    consumer_contract_digest: Arc<str>,
    checkpoint_digest: Arc<str>,
    routing_digest: Arc<str>,
    replay_digest: Arc<str>,
    diagnostics_digest: Arc<str>,
    failure_digest: Arc<str>,
    residue_digest: Arc<str>,
    strategy_lowering_digest: Arc<str>,
    counter_snapshot_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationSemanticDigests {
    pub(crate) fn from_assembly_parts(
        assembly_plan: &BridgeSubscriptionCertificationAssemblyPlan,
        cost_profile: &BridgeSubscriptionCertificationCostProfile,
        scratch: &BridgeSubscriptionCertificationScratch,
        counters: &BridgeSubscriptionCertificationCounterSnapshot,
    ) -> Self {
        let present = |label: &str,
                       source_kind: BridgeSubscriptionCertificationSemanticSourceKind|
         -> Arc<str> {
            let source_digest = assembly_plan
                .semantic_source_digests()
                .source_digest_for(source_kind);
            let digest = Sha256::digest(
                format!(
                    "{label}|source-kind={}|source-present={}|source-digest={}",
                    source_kind.as_str(),
                    source_digest.source_present(),
                    source_digest.digest()
                )
                .as_bytes(),
            );
            Arc::from(format!(
                "bridge-subscription-certification-{label}:sha256:{digest:x}"
            ))
        };
        let optional_field_state =
            |source_kind: BridgeSubscriptionCertificationSemanticSourceKind,
             absent_state: BridgeSubscriptionBundleFieldState|
             -> BridgeSubscriptionBundleFieldState {
                let source_digest = assembly_plan
                    .semantic_source_digests()
                    .source_digest_for(source_kind);
                if source_digest.source_present() {
                    BridgeSubscriptionBundleFieldState::Present
                } else {
                    absent_state
                }
            };
        let field_state = |label: &str,
                           state: BridgeSubscriptionBundleFieldState,
                           source_kind: BridgeSubscriptionCertificationSemanticSourceKind|
         -> Arc<str> {
            let source_digest = assembly_plan
                .semantic_source_digests()
                .source_digest_for(source_kind);
            let digest = Sha256::digest(
                format!(
                    "{label}|field-state={}|source-kind={}|source-present={}|source-digest={}",
                    state.as_str(),
                    source_kind.as_str(),
                    source_digest.source_present(),
                    source_digest.digest()
                )
                .as_bytes(),
            );
            Arc::from(format!(
                "bridge-subscription-certification-{label}:sha256:{digest:x}"
            ))
        };

        let subscription_digest = present(
            "subscription-digest",
            BridgeSubscriptionCertificationSemanticSourceKind::Subscription,
        );
        let subscription_registry_digest = present(
            "subscription-registry-digest",
            BridgeSubscriptionCertificationSemanticSourceKind::SubscriptionRegistry,
        );
        let subscription_basis_digest = present(
            "subscription-basis-digest",
            BridgeSubscriptionCertificationSemanticSourceKind::SubscriptionBasis,
        );
        let subscription_lifecycle_digest = present(
            "subscription-lifecycle-digest",
            BridgeSubscriptionCertificationSemanticSourceKind::SubscriptionLifecycle,
        );
        let subscription_delivery_digest = present(
            "subscription-delivery-digest",
            BridgeSubscriptionCertificationSemanticSourceKind::SubscriptionDelivery,
        );
        let subscription_share_digest = field_state(
            "subscription-share-digest",
            optional_field_state(
                BridgeSubscriptionCertificationSemanticSourceKind::SubscriptionShare,
                BridgeSubscriptionBundleFieldState::NotExercised,
            ),
            BridgeSubscriptionCertificationSemanticSourceKind::SubscriptionShare,
        );
        let subscription_continuation_digest = field_state(
            "subscription-continuation-digest",
            optional_field_state(
                BridgeSubscriptionCertificationSemanticSourceKind::SubscriptionContinuation,
                BridgeSubscriptionBundleFieldState::NotExercised,
            ),
            BridgeSubscriptionCertificationSemanticSourceKind::SubscriptionContinuation,
        );
        let consumer_contract_digest = present(
            "consumer-contract-digest",
            BridgeSubscriptionCertificationSemanticSourceKind::ConsumerContract,
        );
        let checkpoint_digest = field_state(
            "checkpoint-digest",
            optional_field_state(
                BridgeSubscriptionCertificationSemanticSourceKind::Checkpoint,
                BridgeSubscriptionBundleFieldState::NotExercised,
            ),
            BridgeSubscriptionCertificationSemanticSourceKind::Checkpoint,
        );
        let routing_digest = present(
            "routing-digest",
            BridgeSubscriptionCertificationSemanticSourceKind::Routing,
        );
        let replay_digest = field_state(
            "replay-digest",
            optional_field_state(
                BridgeSubscriptionCertificationSemanticSourceKind::Replay,
                BridgeSubscriptionBundleFieldState::RejectedBeforeProduced,
            ),
            BridgeSubscriptionCertificationSemanticSourceKind::Replay,
        );
        let diagnostics_source = assembly_plan
            .semantic_source_digests()
            .source_digest_for(BridgeSubscriptionCertificationSemanticSourceKind::Diagnostics);
        let diagnostics_digest = {
            let digest = Sha256::digest(
                format!(
                    "diagnostics-digest|source-kind={}|source-present={}|source-digest={}|cost-profile={}",
                    BridgeSubscriptionCertificationSemanticSourceKind::Diagnostics.as_str(),
                    diagnostics_source.source_present(),
                    diagnostics_source.digest(),
                    cost_profile.digest(),
                )
                .as_bytes(),
            );
            Arc::from(format!(
                "bridge-subscription-certification-diagnostics-digest:sha256:{digest:x}"
            ))
        };
        let failure_digest = field_state(
            "failure-digest",
            optional_field_state(
                BridgeSubscriptionCertificationSemanticSourceKind::Failure,
                BridgeSubscriptionBundleFieldState::RejectedBeforeProduced,
            ),
            BridgeSubscriptionCertificationSemanticSourceKind::Failure,
        );
        let residue_digest = field_state(
            "residue-digest",
            optional_field_state(
                BridgeSubscriptionCertificationSemanticSourceKind::Residue,
                BridgeSubscriptionBundleFieldState::NotExercised,
            ),
            BridgeSubscriptionCertificationSemanticSourceKind::Residue,
        );
        let strategy_lowering_digest = present(
            "strategy-lowering-digest",
            BridgeSubscriptionCertificationSemanticSourceKind::StrategyLowering,
        );
        let counter_snapshot_digest = counters.digest();
        let canonical_basis = Arc::<str>::from(format!(
            concat!(
                "bridge-subscription-certification-semantic-digests|subscription={}|registry={}|",
                "basis={}|lifecycle={}|delivery={}|share={}|continuation={}|consumer={}|",
                "checkpoint={}|routing={}|replay={}|diagnostics={}|failure={}|residue={}|",
                "strategy-lowering={}|counters={}|scratch={}"
            ),
            subscription_digest,
            subscription_registry_digest,
            subscription_basis_digest,
            subscription_lifecycle_digest,
            subscription_delivery_digest,
            subscription_share_digest,
            subscription_continuation_digest,
            consumer_contract_digest,
            checkpoint_digest,
            routing_digest,
            replay_digest,
            diagnostics_digest,
            failure_digest,
            residue_digest,
            strategy_lowering_digest,
            counter_snapshot_digest,
            scratch.digest(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            subscription_digest,
            subscription_registry_digest,
            subscription_basis_digest,
            subscription_lifecycle_digest,
            subscription_delivery_digest,
            subscription_share_digest,
            subscription_continuation_digest,
            consumer_contract_digest,
            checkpoint_digest,
            routing_digest,
            replay_digest,
            diagnostics_digest,
            failure_digest,
            residue_digest,
            strategy_lowering_digest,
            counter_snapshot_digest,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-semantic-digests:sha256:{digest:x}"
            )),
        }
    }

    pub fn subscription_digest(&self) -> &str {
        self.subscription_digest.as_ref()
    }

    pub fn subscription_registry_digest(&self) -> &str {
        self.subscription_registry_digest.as_ref()
    }

    pub fn subscription_basis_digest(&self) -> &str {
        self.subscription_basis_digest.as_ref()
    }

    pub fn subscription_lifecycle_digest(&self) -> &str {
        self.subscription_lifecycle_digest.as_ref()
    }

    pub fn subscription_delivery_digest(&self) -> &str {
        self.subscription_delivery_digest.as_ref()
    }

    pub fn subscription_share_digest(&self) -> &str {
        self.subscription_share_digest.as_ref()
    }

    pub fn subscription_continuation_digest(&self) -> &str {
        self.subscription_continuation_digest.as_ref()
    }

    pub fn consumer_contract_digest(&self) -> &str {
        self.consumer_contract_digest.as_ref()
    }

    pub fn checkpoint_digest(&self) -> &str {
        self.checkpoint_digest.as_ref()
    }

    pub fn routing_digest(&self) -> &str {
        self.routing_digest.as_ref()
    }

    pub fn replay_digest(&self) -> &str {
        self.replay_digest.as_ref()
    }

    pub fn diagnostics_digest(&self) -> &str {
        self.diagnostics_digest.as_ref()
    }

    pub fn failure_digest(&self) -> &str {
        self.failure_digest.as_ref()
    }

    pub fn residue_digest(&self) -> &str {
        self.residue_digest.as_ref()
    }

    pub fn strategy_lowering_digest(&self) -> &str {
        self.strategy_lowering_digest.as_ref()
    }

    pub fn counter_snapshot_digest(&self) -> &str {
        self.counter_snapshot_digest.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
