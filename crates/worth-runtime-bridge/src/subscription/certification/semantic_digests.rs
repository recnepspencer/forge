use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{
    BridgeSubscriptionBundleFieldState, BridgeSubscriptionCertificationAssemblyPlan,
    BridgeSubscriptionCertificationCostProfile, BridgeSubscriptionCertificationCounterSnapshot,
    BridgeSubscriptionCertificationScratch, BridgeSubscriptionCertificationSemanticSourceKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BridgeSubscriptionCertificationSemanticDigestField {
    Subscription,
    SubscriptionRegistry,
    SubscriptionBasis,
    SubscriptionLifecycle,
    SubscriptionDelivery,
    SubscriptionShare,
    SubscriptionContinuation,
    ConsumerContract,
    Checkpoint,
    Routing,
    Replay,
    Diagnostics,
    Failure,
    Residue,
    StrategyLowering,
}

impl BridgeSubscriptionCertificationSemanticDigestField {
    const fn digest_domain(self) -> &'static str {
        match self {
            Self::Subscription => "subscription-digest",
            Self::SubscriptionRegistry => "subscription-registry-digest",
            Self::SubscriptionBasis => "subscription-basis-digest",
            Self::SubscriptionLifecycle => "subscription-lifecycle-digest",
            Self::SubscriptionDelivery => "subscription-delivery-digest",
            Self::SubscriptionShare => "subscription-share-digest",
            Self::SubscriptionContinuation => "subscription-continuation-digest",
            Self::ConsumerContract => "consumer-contract-digest",
            Self::Checkpoint => "checkpoint-digest",
            Self::Routing => "routing-digest",
            Self::Replay => "replay-digest",
            Self::Diagnostics => "diagnostics-digest",
            Self::Failure => "failure-digest",
            Self::Residue => "residue-digest",
            Self::StrategyLowering => "strategy-lowering-digest",
        }
    }

    const fn source_kind(self) -> BridgeSubscriptionCertificationSemanticSourceKind {
        match self {
            Self::Subscription => BridgeSubscriptionCertificationSemanticSourceKind::Subscription,
            Self::SubscriptionRegistry => {
                BridgeSubscriptionCertificationSemanticSourceKind::SubscriptionRegistry
            }
            Self::SubscriptionBasis => {
                BridgeSubscriptionCertificationSemanticSourceKind::SubscriptionBasis
            }
            Self::SubscriptionLifecycle => {
                BridgeSubscriptionCertificationSemanticSourceKind::SubscriptionLifecycle
            }
            Self::SubscriptionDelivery => {
                BridgeSubscriptionCertificationSemanticSourceKind::SubscriptionDelivery
            }
            Self::SubscriptionShare => {
                BridgeSubscriptionCertificationSemanticSourceKind::SubscriptionShare
            }
            Self::SubscriptionContinuation => {
                BridgeSubscriptionCertificationSemanticSourceKind::SubscriptionContinuation
            }
            Self::ConsumerContract => {
                BridgeSubscriptionCertificationSemanticSourceKind::ConsumerContract
            }
            Self::Checkpoint => BridgeSubscriptionCertificationSemanticSourceKind::Checkpoint,
            Self::Routing => BridgeSubscriptionCertificationSemanticSourceKind::Routing,
            Self::Replay => BridgeSubscriptionCertificationSemanticSourceKind::Replay,
            Self::Diagnostics => BridgeSubscriptionCertificationSemanticSourceKind::Diagnostics,
            Self::Failure => BridgeSubscriptionCertificationSemanticSourceKind::Failure,
            Self::Residue => BridgeSubscriptionCertificationSemanticSourceKind::Residue,
            Self::StrategyLowering => {
                BridgeSubscriptionCertificationSemanticSourceKind::StrategyLowering
            }
        }
    }
}

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
        let subscription_digest = semantic_present_digest(
            BridgeSubscriptionCertificationSemanticDigestField::Subscription,
            assembly_plan,
        );
        let subscription_registry_digest = semantic_present_digest(
            BridgeSubscriptionCertificationSemanticDigestField::SubscriptionRegistry,
            assembly_plan,
        );
        let subscription_basis_digest = semantic_present_digest(
            BridgeSubscriptionCertificationSemanticDigestField::SubscriptionBasis,
            assembly_plan,
        );
        let subscription_lifecycle_digest = semantic_present_digest(
            BridgeSubscriptionCertificationSemanticDigestField::SubscriptionLifecycle,
            assembly_plan,
        );
        let subscription_delivery_digest = semantic_present_digest(
            BridgeSubscriptionCertificationSemanticDigestField::SubscriptionDelivery,
            assembly_plan,
        );
        let subscription_share_digest = semantic_field_state_digest(
            BridgeSubscriptionCertificationSemanticDigestField::SubscriptionShare,
            optional_semantic_field_state(
                BridgeSubscriptionCertificationSemanticDigestField::SubscriptionShare,
                BridgeSubscriptionBundleFieldState::NotExercised,
                assembly_plan,
            ),
            assembly_plan,
        );
        let subscription_continuation_digest = semantic_field_state_digest(
            BridgeSubscriptionCertificationSemanticDigestField::SubscriptionContinuation,
            optional_semantic_field_state(
                BridgeSubscriptionCertificationSemanticDigestField::SubscriptionContinuation,
                BridgeSubscriptionBundleFieldState::NotExercised,
                assembly_plan,
            ),
            assembly_plan,
        );
        let consumer_contract_digest = semantic_present_digest(
            BridgeSubscriptionCertificationSemanticDigestField::ConsumerContract,
            assembly_plan,
        );
        let checkpoint_digest = semantic_field_state_digest(
            BridgeSubscriptionCertificationSemanticDigestField::Checkpoint,
            optional_semantic_field_state(
                BridgeSubscriptionCertificationSemanticDigestField::Checkpoint,
                BridgeSubscriptionBundleFieldState::NotExercised,
                assembly_plan,
            ),
            assembly_plan,
        );
        let routing_digest = semantic_present_digest(
            BridgeSubscriptionCertificationSemanticDigestField::Routing,
            assembly_plan,
        );
        let replay_digest = semantic_field_state_digest(
            BridgeSubscriptionCertificationSemanticDigestField::Replay,
            optional_semantic_field_state(
                BridgeSubscriptionCertificationSemanticDigestField::Replay,
                BridgeSubscriptionBundleFieldState::RejectedBeforeProduced,
                assembly_plan,
            ),
            assembly_plan,
        );
        let diagnostics_digest = semantic_diagnostics_digest(assembly_plan, cost_profile);
        let failure_digest = semantic_field_state_digest(
            BridgeSubscriptionCertificationSemanticDigestField::Failure,
            optional_semantic_field_state(
                BridgeSubscriptionCertificationSemanticDigestField::Failure,
                BridgeSubscriptionBundleFieldState::RejectedBeforeProduced,
                assembly_plan,
            ),
            assembly_plan,
        );
        let residue_digest = semantic_field_state_digest(
            BridgeSubscriptionCertificationSemanticDigestField::Residue,
            optional_semantic_field_state(
                BridgeSubscriptionCertificationSemanticDigestField::Residue,
                BridgeSubscriptionBundleFieldState::NotExercised,
                assembly_plan,
            ),
            assembly_plan,
        );
        let strategy_lowering_digest = semantic_present_digest(
            BridgeSubscriptionCertificationSemanticDigestField::StrategyLowering,
            assembly_plan,
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

fn semantic_present_digest(
    field: BridgeSubscriptionCertificationSemanticDigestField,
    assembly_plan: &BridgeSubscriptionCertificationAssemblyPlan,
) -> Arc<str> {
    let source_kind = field.source_kind();
    let source_digest = assembly_plan
        .semantic_source_digests()
        .source_digest_for(source_kind);
    let digest_domain = field.digest_domain();
    let digest = Sha256::digest(
        format!(
            "{digest_domain}|source-kind={}|source-present={}|source-digest={}",
            source_kind.as_str(),
            source_digest.source_present(),
            source_digest.digest()
        )
        .as_bytes(),
    );
    Arc::from(format!(
        "bridge-subscription-certification-{digest_domain}:sha256:{digest:x}"
    ))
}

fn optional_semantic_field_state(
    field: BridgeSubscriptionCertificationSemanticDigestField,
    absent_state: BridgeSubscriptionBundleFieldState,
    assembly_plan: &BridgeSubscriptionCertificationAssemblyPlan,
) -> BridgeSubscriptionBundleFieldState {
    let source_digest = assembly_plan
        .semantic_source_digests()
        .source_digest_for(field.source_kind());
    if source_digest.source_present() {
        BridgeSubscriptionBundleFieldState::Present
    } else {
        absent_state
    }
}

fn semantic_field_state_digest(
    field: BridgeSubscriptionCertificationSemanticDigestField,
    field_state: BridgeSubscriptionBundleFieldState,
    assembly_plan: &BridgeSubscriptionCertificationAssemblyPlan,
) -> Arc<str> {
    let source_kind = field.source_kind();
    let source_digest = assembly_plan
        .semantic_source_digests()
        .source_digest_for(source_kind);
    let digest_domain = field.digest_domain();
    let digest = Sha256::digest(
        format!(
            "{digest_domain}|field-state={}|source-kind={}|source-present={}|source-digest={}",
            field_state.as_str(),
            source_kind.as_str(),
            source_digest.source_present(),
            source_digest.digest()
        )
        .as_bytes(),
    );
    Arc::from(format!(
        "bridge-subscription-certification-{digest_domain}:sha256:{digest:x}"
    ))
}

fn semantic_diagnostics_digest(
    assembly_plan: &BridgeSubscriptionCertificationAssemblyPlan,
    cost_profile: &BridgeSubscriptionCertificationCostProfile,
) -> Arc<str> {
    let field = BridgeSubscriptionCertificationSemanticDigestField::Diagnostics;
    let source_kind = field.source_kind();
    let source_digest = assembly_plan
        .semantic_source_digests()
        .source_digest_for(source_kind);
    let digest_domain = field.digest_domain();
    let digest = Sha256::digest(
        format!(
            "{digest_domain}|source-kind={}|source-present={}|source-digest={}|cost-profile={}",
            source_kind.as_str(),
            source_digest.source_present(),
            source_digest.digest(),
            cost_profile.digest(),
        )
        .as_bytes(),
    );
    Arc::from(format!(
        "bridge-subscription-certification-{digest_domain}:sha256:{digest:x}"
    ))
}
