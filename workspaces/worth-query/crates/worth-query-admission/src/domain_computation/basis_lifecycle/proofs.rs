use crate::admission_digest::hash_parts;

use super::counters::BasisEligibilityCounters;
use super::decision_trace::BasisEligibilityDecisionTrace;
use super::lanes::BasisOperationLane;
pub use super::normalized_intent::NormalizedBasisIntent;
use super::taxonomy::{BasisIntentDenialKind, DeniedBasisCapabilityKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisIntentDenial {
    denial_kind: BasisIntentDenialKind,
    message: &'static str,
    counters: BasisEligibilityCounters,
}

impl BasisIntentDenial {
    #[cfg(test)]
    pub(crate) fn new(denial_kind: BasisIntentDenialKind, message: &'static str) -> Self {
        Self {
            denial_kind,
            message,
            counters: BasisEligibilityCounters::rejected(1),
        }
    }

    pub fn denial_kind(&self) -> BasisIntentDenialKind {
        self.denial_kind
    }

    pub fn message(&self) -> &'static str {
        self.message
    }

    pub fn counters(&self) -> &BasisEligibilityCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeniedBasisCapability {
    denial_kind: DeniedBasisCapabilityKind,
    decision_trace: BasisEligibilityDecisionTrace,
    counters: BasisEligibilityCounters,
}

impl DeniedBasisCapability {
    pub(crate) fn new(
        denial_kind: DeniedBasisCapabilityKind,
        normalized: &NormalizedBasisIntent,
        message: &'static str,
        counters: BasisEligibilityCounters,
    ) -> Self {
        Self {
            denial_kind,
            decision_trace: BasisEligibilityDecisionTrace::new(normalized, "violation", message),
            counters,
        }
    }

    pub fn denial_kind(&self) -> DeniedBasisCapabilityKind {
        self.denial_kind
    }

    pub fn decision_trace(&self) -> &BasisEligibilityDecisionTrace {
        &self.decision_trace
    }

    pub fn counters(&self) -> &BasisEligibilityCounters {
        &self.counters
    }

    pub(crate) fn new_readmission(
        denial_kind: DeniedBasisCapabilityKind,
        decision_trace: BasisEligibilityDecisionTrace,
        counters: BasisEligibilityCounters,
    ) -> Self {
        Self {
            denial_kind,
            decision_trace,
            counters,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisEligibility<L: BasisOperationLane> {
    pub(super) normalized: NormalizedBasisIntent,
    pub(super) lane: L,
    decision_trace: BasisEligibilityDecisionTrace,
    counters: BasisEligibilityCounters,
}

impl<L: BasisOperationLane> BasisEligibility<L> {
    pub(crate) fn new(normalized: NormalizedBasisIntent, lane: L) -> Self {
        let decision_trace =
            BasisEligibilityDecisionTrace::new(&normalized, "success", "basis lane admitted");
        Self {
            normalized,
            lane,
            decision_trace,
            counters: BasisEligibilityCounters::eligibility(0, 0, 0, 0),
        }
    }

    pub fn normalized(&self) -> &NormalizedBasisIntent {
        &self.normalized
    }

    pub fn decision_trace(&self) -> &BasisEligibilityDecisionTrace {
        &self.decision_trace
    }

    pub fn counters(&self) -> &BasisEligibilityCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdvisoryBasisEligibility<L: BasisOperationLane> {
    normalized: NormalizedBasisIntent,
    lane: L,
    decision_trace: BasisEligibilityDecisionTrace,
}

impl<L: BasisOperationLane> AdvisoryBasisEligibility<L> {
    pub(crate) fn new(normalized: NormalizedBasisIntent, lane: L) -> Self {
        let decision_trace =
            BasisEligibilityDecisionTrace::new(&normalized, "advisory", "basis lane is advisory");
        Self {
            normalized,
            lane,
            decision_trace,
        }
    }

    pub fn decision_trace(&self) -> &BasisEligibilityDecisionTrace {
        &self.decision_trace
    }

    pub fn normalized(&self) -> &NormalizedBasisIntent {
        &self.normalized
    }

    pub fn authoring_digest(&self) -> String {
        hash_parts(&[
            "advisory_basis_authoring_v1".to_string(),
            format!("normalized:{}", self.normalized.normalized_digest()),
            format!("lane:{}", L::lane_name()),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeferredBasisEligibility<L: BasisOperationLane> {
    normalized: NormalizedBasisIntent,
    lane: L,
    denial_kind: DeniedBasisCapabilityKind,
    decision_trace: BasisEligibilityDecisionTrace,
    counters: BasisEligibilityCounters,
}

impl<L: BasisOperationLane> DeferredBasisEligibility<L> {
    pub(crate) fn new(
        normalized: NormalizedBasisIntent,
        lane: L,
        denial_kind: DeniedBasisCapabilityKind,
        message: &'static str,
    ) -> Self {
        let decision_trace = BasisEligibilityDecisionTrace::new(&normalized, "deferred", message);
        Self {
            normalized,
            lane,
            denial_kind,
            decision_trace,
            counters: BasisEligibilityCounters::eligibility(0, 0, 1, 0),
        }
    }

    pub fn normalized(&self) -> &NormalizedBasisIntent {
        &self.normalized
    }

    pub fn denial_kind(&self) -> DeniedBasisCapabilityKind {
        self.denial_kind
    }

    pub fn decision_trace(&self) -> &BasisEligibilityDecisionTrace {
        &self.decision_trace
    }

    pub fn counters(&self) -> &BasisEligibilityCounters {
        &self.counters
    }

    pub fn authoring_digest(&self) -> String {
        hash_parts(&[
            "deferred_basis_authoring_v1".to_string(),
            format!("normalized:{}", self.normalized.normalized_digest()),
            format!("lane:{}", L::lane_name()),
            format!("denial_kind:{}", self.denial_kind.as_str()),
        ])
    }
}
