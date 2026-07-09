use worth_store_budgets::CounterEvidenceStrength;

use crate::planning::S8PlanFingerprint;

use super::{S8AccessPathCounterSnapshot, S8AccessPathKind, S8StoreLayoutPerformanceReceipt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8AccessAttemptCostReceipt {
    NoExecutionCountersSpent {
        fingerprint: S8PlanFingerprint,
        path_kind: S8AccessPathKind,
    },
    DeniedObservedExecutionCost {
        fingerprint: S8PlanFingerprint,
        path_kind: S8AccessPathKind,
        observed: S8AccessPathCounterSnapshot,
        counter_strength: CounterEvidenceStrength,
    },
    ObservedExecutionCost(S8StoreLayoutPerformanceReceipt),
}

impl S8AccessAttemptCostReceipt {
    pub const fn fingerprint(self) -> S8PlanFingerprint {
        match self {
            Self::NoExecutionCountersSpent { fingerprint, .. } => fingerprint,
            Self::DeniedObservedExecutionCost { fingerprint, .. } => fingerprint,
            Self::ObservedExecutionCost(receipt) => receipt.fingerprint(),
        }
    }

    pub const fn path_kind(self) -> S8AccessPathKind {
        match self {
            Self::NoExecutionCountersSpent { path_kind, .. } => path_kind,
            Self::DeniedObservedExecutionCost { path_kind, .. } => path_kind,
            Self::ObservedExecutionCost(receipt) => receipt.planned_vs_observed().path_kind(),
        }
    }
}
