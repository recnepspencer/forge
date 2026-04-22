use std::sync::Arc;

use sha2::{Digest, Sha256};

use super::BridgeSubscriptionCertificationCounterSnapshot;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeSubscriptionSourceArtifactKind {
    Declaration,
    BasisBinding,
    AdmittedSubscription,
    Lifecycle,
    ActiveDelivery,
    DeliveryWindow,
    Fanout,
    Checkpoint,
    Resume,
    Continuation,
    Preview,
    RetainedReplay,
    StrategyLowering,
    Failure,
    LaneIdentity,
}

impl BridgeSubscriptionSourceArtifactKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Declaration => "declaration",
            Self::BasisBinding => "basis_binding",
            Self::AdmittedSubscription => "admitted_subscription",
            Self::Lifecycle => "lifecycle",
            Self::ActiveDelivery => "active_delivery",
            Self::DeliveryWindow => "delivery_window",
            Self::Fanout => "fanout",
            Self::Checkpoint => "checkpoint",
            Self::Resume => "resume",
            Self::Continuation => "continuation",
            Self::Preview => "preview",
            Self::RetainedReplay => "retained_replay",
            Self::StrategyLowering => "strategy_lowering",
            Self::Failure => "failure",
            Self::LaneIdentity => "lane_identity",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionSourceArtifactInput {
    artifact_kind: BridgeSubscriptionSourceArtifactKind,
    artifact_identity: Arc<str>,
    artifact_digest: Arc<str>,
}

impl BridgeSubscriptionSourceArtifactInput {
    pub fn new(
        artifact_kind: BridgeSubscriptionSourceArtifactKind,
        artifact_identity: impl Into<Arc<str>>,
        artifact_digest: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            artifact_kind,
            artifact_identity: artifact_identity.into(),
            artifact_digest: artifact_digest.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionSourceArtifactRecord {
    artifact_kind: BridgeSubscriptionSourceArtifactKind,
    artifact_identity: Arc<str>,
    artifact_digest: Arc<str>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionSourceArtifactRecord {
    fn from_input(input: BridgeSubscriptionSourceArtifactInput) -> Self {
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-source-artifact|kind={}|identity={}|digest={}",
            input.artifact_kind.as_str(),
            input.artifact_identity,
            input.artifact_digest,
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            artifact_kind: input.artifact_kind,
            artifact_identity: input.artifact_identity,
            artifact_digest: input.artifact_digest,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-source-artifact:sha256:{digest:x}"
            )),
        }
    }

    pub fn artifact_kind(&self) -> BridgeSubscriptionSourceArtifactKind {
        self.artifact_kind
    }

    pub fn artifact_identity(&self) -> &str {
        self.artifact_identity.as_ref()
    }

    pub fn artifact_digest(&self) -> &str {
        self.artifact_digest.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeSubscriptionSourceArtifactIndex {
    records: Vec<BridgeSubscriptionSourceArtifactRecord>,
    counters: BridgeSubscriptionCertificationCounterSnapshot,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeSubscriptionSourceArtifactIndex {
    pub(crate) fn build(inputs: Vec<BridgeSubscriptionSourceArtifactInput>) -> Self {
        let scanned_input_count = inputs.len();
        let mut records: Vec<_> = inputs
            .into_iter()
            .map(BridgeSubscriptionSourceArtifactRecord::from_input)
            .collect();
        records.sort_by(|left, right| {
            left.artifact_kind
                .cmp(&right.artifact_kind)
                .then_with(|| left.artifact_identity.cmp(&right.artifact_identity))
                .then_with(|| left.artifact_digest.cmp(&right.artifact_digest))
        });
        records.dedup_by(|left, right| {
            left.artifact_kind == right.artifact_kind
                && left.artifact_identity == right.artifact_identity
                && left.artifact_digest == right.artifact_digest
        });
        let record_digests = records
            .iter()
            .map(|record| record.digest.as_ref())
            .collect::<Vec<_>>()
            .join(",");
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-source-artifact-index|records={record_digests}"
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            counters: BridgeSubscriptionCertificationCounterSnapshot::from_source_artifact_index(
                records.len(),
                scanned_input_count,
            ),
            records,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-subscription-source-artifact-index:sha256:{digest:x}"
            )),
        }
    }

    pub fn records(&self) -> &[BridgeSubscriptionSourceArtifactRecord] {
        &self.records
    }

    pub fn counters(&self) -> &BridgeSubscriptionCertificationCounterSnapshot {
        &self.counters
    }

    pub(crate) fn artifact_kind_digest(
        &self,
        artifact_kind: BridgeSubscriptionSourceArtifactKind,
    ) -> Arc<str> {
        let matching_digests = self
            .records
            .iter()
            .filter(|record| record.artifact_kind == artifact_kind)
            .map(|record| record.digest.as_ref())
            .collect::<Vec<_>>()
            .join(",");
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-subscription-source-artifact-kind-index|kind={}|records={matching_digests}",
            artifact_kind.as_str(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Arc::from(format!(
            "bridge-subscription-source-artifact-kind-index:sha256:{digest:x}"
        ))
    }

    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
