use crate::runtime::{
    WorthUiDurableStateFamilyId, WorthUiDurableStateReconciliationDenial,
    WorthUiDurableStateReconciliationOutcome, WorthUiIdentityStateQueryCertificationCounters,
    WorthUiQueryBindingDriftDenialKind,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiIdentityStateQueryCertificationDenial {
    reason: Box<WorthUiIdentityStateQueryCertificationDenialReason>,
    counters: Box<WorthUiIdentityStateQueryCertificationCounters>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiIdentityStateQueryCertificationDenialReason {
    EmptyScenario,
    StatePlanDigestMismatch {
        plan_active_artifact_digest: u64,
        reconciliation_active_artifact_digest: u64,
        plan_candidate_artifact_digest: u64,
        reconciliation_candidate_artifact_digest: u64,
    },
    StatePlanActiveRuntimeMismatch {
        label: String,
        active_runtime_artifact_digest: u64,
        plan_active_artifact_digest: u64,
    },
    QueryPlanActiveRuntimeMismatch {
        label: String,
        active_runtime_artifact_digest: u64,
        plan_active_artifact_digest: u64,
    },
    SnapshotDigestMismatch {
        active_snapshot_digest: u64,
        provided_snapshot_digest: u64,
    },
    AmbiguousIdentityPreservedDurableState {
        label: String,
        identity_basis: String,
        family_id: WorthUiDurableStateFamilyId,
    },
    StateReceiptTransitionMismatch {
        label: String,
        identity_basis: String,
        outcome: WorthUiDurableStateReconciliationOutcome,
    },
    StateReconciliationDenied {
        label: String,
        denial: WorthUiDurableStateReconciliationDenial,
    },
    QueryPlanDigestMismatch {
        label: String,
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
    },
    StateQueryResidue {
        label: String,
    },
    MissingTypedQueryDriftDenial {
        label: String,
    },
    UnexpectedTypedQueryDriftDenial {
        label: String,
        expected: WorthUiQueryBindingDriftDenialKind,
    },
}

impl WorthUiIdentityStateQueryCertificationDenial {
    pub(crate) fn new(
        reason: WorthUiIdentityStateQueryCertificationDenialReason,
        counters: WorthUiIdentityStateQueryCertificationCounters,
    ) -> Self {
        Self {
            reason: Box::new(reason),
            counters: Box::new(counters),
        }
    }

    pub fn reason(&self) -> &WorthUiIdentityStateQueryCertificationDenialReason {
        &self.reason
    }

    pub fn counters(&self) -> WorthUiIdentityStateQueryCertificationCounters {
        *self.counters
    }
}
