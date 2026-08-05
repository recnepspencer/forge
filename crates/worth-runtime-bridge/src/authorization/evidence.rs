use std::sync::Arc;

use worth_signal::facade::{
    SignalAuthorizationDecision, SignalAuthorizationDecisionEvidence,
    SignalAuthorizationEvaluationCounters,
};

use super::{BridgeAuthorizationCorrespondenceIdentity, BridgeAuthorizationRuleEffect};

pub(crate) struct BridgeAuthorizationCorrespondenceAuthority {
    pub(crate) _seal: (),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BridgeAuthorizationRuleDecisionEvidence {
    effect: BridgeAuthorizationRuleEffect,
    matched: bool,
}

impl BridgeAuthorizationRuleDecisionEvidence {
    pub(crate) const fn new(effect: BridgeAuthorizationRuleEffect, matched: bool) -> Self {
        Self { effect, matched }
    }

    pub const fn effect(self) -> BridgeAuthorizationRuleEffect {
        self.effect
    }

    pub const fn matched(self) -> bool {
        self.matched
    }
}

pub struct BridgeAuthorizationDecisionEvidence {
    correspondence: BridgeAuthorizationCorrespondenceIdentity,
    dependency_identity: [u8; 32],
    signal: SignalAuthorizationDecisionEvidence,
    rule_decisions: Vec<BridgeAuthorizationRuleDecisionEvidence>,
    authority: Arc<BridgeAuthorizationCorrespondenceAuthority>,
}

impl BridgeAuthorizationDecisionEvidence {
    pub(crate) fn mint(
        correspondence: BridgeAuthorizationCorrespondenceIdentity,
        dependency_identity: [u8; 32],
        signal: SignalAuthorizationDecisionEvidence,
        rule_decisions: Vec<BridgeAuthorizationRuleDecisionEvidence>,
        authority: Arc<BridgeAuthorizationCorrespondenceAuthority>,
    ) -> Self {
        Self {
            correspondence,
            dependency_identity,
            signal,
            rule_decisions,
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

    pub fn rule_decisions(&self) -> &[BridgeAuthorizationRuleDecisionEvidence] {
        &self.rule_decisions
    }

    pub(crate) fn signal(&self) -> &SignalAuthorizationDecisionEvidence {
        &self.signal
    }

    pub(crate) fn authority(&self) -> &Arc<BridgeAuthorizationCorrespondenceAuthority> {
        &self.authority
    }
}
