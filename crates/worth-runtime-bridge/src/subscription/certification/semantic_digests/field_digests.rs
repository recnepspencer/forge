use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::super::{
    BridgeSubscriptionBundleFieldState, BridgeSubscriptionCertificationAssemblyPlan,
    BridgeSubscriptionCertificationCostProfile, BridgeSubscriptionCertificationSemanticSourceKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum BridgeSubscriptionCertificationSemanticDigestField {
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

pub(super) fn semantic_present_digest(
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

pub(super) fn optional_semantic_field_state(
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

pub(super) fn semantic_field_state_digest(
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

pub(super) fn semantic_diagnostics_digest(
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
