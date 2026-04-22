use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::{BridgeSubscriptionSourceArtifactIndex, BridgeSubscriptionSourceArtifactKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionCertificationSemanticSourceKind {
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

impl BridgeSubscriptionCertificationSemanticSourceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Subscription => "subscription",
            Self::SubscriptionRegistry => "subscription_registry",
            Self::SubscriptionBasis => "subscription_basis",
            Self::SubscriptionLifecycle => "subscription_lifecycle",
            Self::SubscriptionDelivery => "subscription_delivery",
            Self::SubscriptionShare => "subscription_share",
            Self::SubscriptionContinuation => "subscription_continuation",
            Self::ConsumerContract => "consumer_contract",
            Self::Checkpoint => "checkpoint",
            Self::Routing => "routing",
            Self::Replay => "replay",
            Self::Diagnostics => "diagnostics",
            Self::Failure => "failure",
            Self::Residue => "residue",
            Self::StrategyLowering => "strategy_lowering",
        }
    }

    const fn artifact_kind(self) -> Option<BridgeSubscriptionSourceArtifactKind> {
        match self {
            Self::Subscription => Some(BridgeSubscriptionSourceArtifactKind::AdmittedSubscription),
            Self::SubscriptionRegistry => Some(BridgeSubscriptionSourceArtifactKind::Declaration),
            Self::SubscriptionBasis => Some(BridgeSubscriptionSourceArtifactKind::BasisBinding),
            Self::SubscriptionLifecycle => Some(BridgeSubscriptionSourceArtifactKind::Lifecycle),
            Self::SubscriptionDelivery => {
                Some(BridgeSubscriptionSourceArtifactKind::ActiveDelivery)
            }
            Self::SubscriptionShare => Some(BridgeSubscriptionSourceArtifactKind::Fanout),
            Self::SubscriptionContinuation => {
                Some(BridgeSubscriptionSourceArtifactKind::Continuation)
            }
            Self::ConsumerContract => Some(BridgeSubscriptionSourceArtifactKind::ActiveDelivery),
            Self::Checkpoint => Some(BridgeSubscriptionSourceArtifactKind::Checkpoint),
            Self::Routing => Some(BridgeSubscriptionSourceArtifactKind::DeliveryWindow),
            Self::Replay => Some(BridgeSubscriptionSourceArtifactKind::RetainedReplay),
            Self::Diagnostics => None,
            Self::Failure => Some(BridgeSubscriptionSourceArtifactKind::Failure),
            Self::Residue => Some(BridgeSubscriptionSourceArtifactKind::Preview),
            Self::StrategyLowering => Some(BridgeSubscriptionSourceArtifactKind::StrategyLowering),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationSemanticSourceDigest {
    source_kind: BridgeSubscriptionCertificationSemanticSourceKind,
    source_present: bool,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationSemanticSourceDigest {
    fn from_index(
        source_kind: BridgeSubscriptionCertificationSemanticSourceKind,
        source_artifact_index: &BridgeSubscriptionSourceArtifactIndex,
    ) -> Self {
        let source_present = source_kind.artifact_kind().is_some_and(|artifact_kind| {
            source_artifact_index
                .records()
                .iter()
                .any(|record| record.artifact_kind() == artifact_kind)
        });
        let basis = match source_kind.artifact_kind() {
            Some(artifact_kind) => source_artifact_index.artifact_kind_digest(artifact_kind),
            None => Arc::from("diagnostics-derived-from-cost-profile"),
        };
        let digest = Sha256::digest(
            format!(
                "bridge-subscription-certification-semantic-source|kind={}|present={source_present}|basis={basis}",
                source_kind.as_str(),
            )
            .as_bytes(),
        );
        Self {
            source_kind,
            source_present,
            digest: Arc::from(format!(
                "bridge-subscription-certification-semantic-source:sha256:{digest:x}"
            )),
        }
    }

    pub fn source_kind(&self) -> BridgeSubscriptionCertificationSemanticSourceKind {
        self.source_kind
    }

    pub fn source_present(&self) -> bool {
        self.source_present
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionCertificationSemanticSourceDigestSet {
    digests: Vec<BridgeSubscriptionCertificationSemanticSourceDigest>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionCertificationSemanticSourceDigestSet {
    pub(crate) fn from_source_artifact_index(
        source_artifact_index: &BridgeSubscriptionSourceArtifactIndex,
    ) -> Self {
        let digests = [
            BridgeSubscriptionCertificationSemanticSourceKind::Subscription,
            BridgeSubscriptionCertificationSemanticSourceKind::SubscriptionRegistry,
            BridgeSubscriptionCertificationSemanticSourceKind::SubscriptionBasis,
            BridgeSubscriptionCertificationSemanticSourceKind::SubscriptionLifecycle,
            BridgeSubscriptionCertificationSemanticSourceKind::SubscriptionDelivery,
            BridgeSubscriptionCertificationSemanticSourceKind::SubscriptionShare,
            BridgeSubscriptionCertificationSemanticSourceKind::SubscriptionContinuation,
            BridgeSubscriptionCertificationSemanticSourceKind::ConsumerContract,
            BridgeSubscriptionCertificationSemanticSourceKind::Checkpoint,
            BridgeSubscriptionCertificationSemanticSourceKind::Routing,
            BridgeSubscriptionCertificationSemanticSourceKind::Replay,
            BridgeSubscriptionCertificationSemanticSourceKind::Diagnostics,
            BridgeSubscriptionCertificationSemanticSourceKind::Failure,
            BridgeSubscriptionCertificationSemanticSourceKind::Residue,
            BridgeSubscriptionCertificationSemanticSourceKind::StrategyLowering,
        ]
        .into_iter()
        .map(|source_kind| Self::source_digest(source_kind, source_artifact_index))
        .collect::<Vec<_>>();
        let digest_basis = digests
            .iter()
            .map(BridgeSubscriptionCertificationSemanticSourceDigest::digest)
            .collect::<Vec<_>>()
            .join(",");
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-certification-semantic-source-set|digests={digest_basis}"
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            digests,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-certification-semantic-source-set:sha256:{digest:x}"
            )),
        }
    }

    fn source_digest(
        source_kind: BridgeSubscriptionCertificationSemanticSourceKind,
        source_artifact_index: &BridgeSubscriptionSourceArtifactIndex,
    ) -> BridgeSubscriptionCertificationSemanticSourceDigest {
        BridgeSubscriptionCertificationSemanticSourceDigest::from_index(
            source_kind,
            source_artifact_index,
        )
    }

    pub fn source_digest_for(
        &self,
        source_kind: BridgeSubscriptionCertificationSemanticSourceKind,
    ) -> &BridgeSubscriptionCertificationSemanticSourceDigest {
        self.digests
            .iter()
            .find(|digest| digest.source_kind == source_kind)
            .expect("semantic source digest set is constructed exhaustively")
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
