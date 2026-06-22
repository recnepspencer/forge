use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::input::envelope::BridgeCommittedPatchEnvelope;
use crate::source::{
    AdmittedBridgeAsyncCompletion, BridgeAsyncClassifiedDeniedCompletion, BridgeAsyncRetryLineage,
    BridgeAsyncRevalidationLineage,
};
use crate::subscription::{
    BridgeSubscriptionCounters, BridgeSubscriptionMixedCauseOrderingRequestIdentity,
    BridgeTemporalCauseRecord,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeMixedCauseOrderingLaneKind {
    Authoritative,
    Preview,
}

impl BridgeMixedCauseOrderingLaneKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authoritative => "authoritative",
            Self::Preview => "preview",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeMixedCauseOrderingInput {
    TruthPatch(BridgeCommittedPatchEnvelope),
    Temporal(BridgeTemporalCauseRecord),
    AsyncCompletion(AdmittedBridgeAsyncCompletion),
    AsyncClassifiedDeniedCompletion(BridgeAsyncClassifiedDeniedCompletion),
    AsyncRetryLineage(BridgeAsyncRetryLineage),
    AsyncRevalidationLineage(BridgeAsyncRevalidationLineage),
}

impl BridgeMixedCauseOrderingInput {
    pub(crate) fn descriptor(&self) -> String {
        match self {
            Self::TruthPatch(patch) => format!("truth-patch:{}", patch.digest().as_str()),
            Self::Temporal(cause) => format!("temporal:{}", cause.digest()),
            Self::AsyncCompletion(completion) => {
                format!("async-completion:{}", completion.digest())
            }
            Self::AsyncClassifiedDeniedCompletion(denied) => {
                format!("async-denied:{}", denied.receipt().digest())
            }
            Self::AsyncRetryLineage(lineage) => format!("async-retry:{}", lineage.digest()),
            Self::AsyncRevalidationLineage(lineage) => {
                format!("async-revalidation:{}", lineage.digest())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeMixedCauseOrderingRequest {
    ordering_request_identity: BridgeSubscriptionMixedCauseOrderingRequestIdentity,
    lane_kind: BridgeMixedCauseOrderingLaneKind,
    inputs: Vec<BridgeMixedCauseOrderingInput>,
    counters: BridgeSubscriptionCounters,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgeMixedCauseOrderingRequest {
    pub fn new(
        lane_kind: BridgeMixedCauseOrderingLaneKind,
        inputs: Vec<BridgeMixedCauseOrderingInput>,
    ) -> Self {
        let mut descriptors = inputs
            .iter()
            .map(BridgeMixedCauseOrderingInput::descriptor)
            .collect::<Vec<_>>();
        descriptors.sort();
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-mixed-cause-ordering-request|lane={}|inputs={}",
            lane_kind.as_str(),
            descriptors.join(","),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            ordering_request_identity:
                BridgeSubscriptionMixedCauseOrderingRequestIdentity::admit_bridge_owned(format!(
                    "bridge-mixed-cause-ordering-request-id:sha256:{digest:x}"
                )),
            lane_kind,
            inputs,
            counters: BridgeSubscriptionCounters::from_mixed_cause_ordering_request(),
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-mixed-cause-ordering-request:sha256:{digest:x}"
            )),
        }
    }

    pub fn ordering_request_identity(
        &self,
    ) -> &BridgeSubscriptionMixedCauseOrderingRequestIdentity {
        &self.ordering_request_identity
    }

    pub fn lane_kind(&self) -> BridgeMixedCauseOrderingLaneKind {
        self.lane_kind
    }

    pub fn inputs(&self) -> &[BridgeMixedCauseOrderingInput] {
        &self.inputs
    }

    pub fn counters(&self) -> &BridgeSubscriptionCounters {
        &self.counters
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}
