use std::sync::Arc;

use worth_signal::facade::{
    SignalAuthorizationDecision, SignalAuthorizationDecisionEvidence,
    SignalAuthorizationEvaluationCounters,
};

use super::BridgeAuthorizationCorrespondenceIdentity;

pub(crate) struct BridgeAuthorizationCorrespondenceAuthority {
    pub(crate) _seal: (),
}

pub struct BridgeAuthorizationDecisionEvidence {
    correspondence: BridgeAuthorizationCorrespondenceIdentity,
    dependency_identity: [u8; 32],
    signal: SignalAuthorizationDecisionEvidence,
    authority: Arc<BridgeAuthorizationCorrespondenceAuthority>,
}

impl BridgeAuthorizationDecisionEvidence {
    pub(crate) fn mint(
        correspondence: BridgeAuthorizationCorrespondenceIdentity,
        dependency_identity: [u8; 32],
        signal: SignalAuthorizationDecisionEvidence,
        authority: Arc<BridgeAuthorizationCorrespondenceAuthority>,
    ) -> Self {
        Self {
            correspondence,
            dependency_identity,
            signal,
            authority,
        }
    }

    pub const fn correspondence(&self) -> BridgeAuthorizationCorrespondenceIdentity {
        self.correspondence
    }

    pub const fn dependency_identity(&self) -> &[u8; 32] {
        &self.dependency_identity
    }

    pub const fn decision(&self) -> SignalAuthorizationDecision {
        self.signal.decision()
    }

    pub const fn is_allowed(&self) -> bool {
        matches!(self.signal.decision(), SignalAuthorizationDecision::Allowed)
    }

    pub const fn counters(&self) -> SignalAuthorizationEvaluationCounters {
        self.signal.counters()
    }

    pub(crate) fn signal(&self) -> &SignalAuthorizationDecisionEvidence {
        &self.signal
    }

    pub(crate) fn authority(&self) -> &Arc<BridgeAuthorizationCorrespondenceAuthority> {
        &self.authority
    }
}
