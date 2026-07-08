use crate::{
    artifact_family::PhysicalArtifactFamily,
    execution::{
        attempt_cost::S8AccessAttemptCostReceipt, S8AccessLoweringBasis, S8AccessPathCounterSnapshot,
    },
    key_domain::PhysicalKeyDomain,
    materialization::{S8LayoutCoverageWitness, S8MaterializationDenial},
};
use forge_store_budgets::S8PreExecutionPlanBinding;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8AccessLoweringDenied {
    ReadmissionWitnessMismatch {
        basis: S8AccessLoweringBasis,
        expected: S8AccessLoweringBasis,
        actual: S8AccessLoweringBasis,
    },
    ReadmissionPlannedCountersMismatch {
        basis: S8AccessLoweringBasis,
        expected: S8AccessPathCounterSnapshot,
        actual: S8AccessPathCounterSnapshot,
    },
    RebindWitnessMismatch {
        basis: S8AccessLoweringBasis,
        expected: S8AccessLoweringBasis,
        actual: S8AccessLoweringBasis,
    },
    CoverageDenied {
        basis: S8AccessLoweringBasis,
        denial: S8MaterializationDenial,
    },
    LifecycleFamilyMismatch {
        basis: S8AccessLoweringBasis,
        expected: PhysicalArtifactFamily,
        actual: PhysicalArtifactFamily,
    },
    KeyDomainMismatch {
        basis: S8AccessLoweringBasis,
        expected: PhysicalKeyDomain,
        actual: PhysicalKeyDomain,
    },
    CurrentCoverageMismatch {
        basis: S8AccessLoweringBasis,
        expected: S8LayoutCoverageWitness,
        actual: S8LayoutCoverageWitness,
    },
    ExecutedCounterWitnessPathMismatch {
        expected: S8AccessLoweringBasis,
        actual_path_kind: crate::execution::S8AccessPathKind,
        observed: crate::execution::S8ObservedAccessPathCounters,
    },
    ExecutedCounterWitnessPlanBindingMismatch {
        expected: S8AccessLoweringBasis,
        expected_plan_binding: S8PreExecutionPlanBinding,
        actual_plan_binding: S8PreExecutionPlanBinding,
        observed: crate::execution::S8ObservedAccessPathCounters,
    },
    ObservedCounterBasisMismatch {
        expected: S8AccessLoweringBasis,
        actual: S8AccessLoweringBasis,
        observed: crate::execution::S8AdmittedExecutedCounters,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8AccessLoweringDeferred {
    RuntimeLeaseRequired {
        basis: S8AccessLoweringBasis,
    },
}

impl S8AccessLoweringDenied {
    pub const fn basis(self) -> S8AccessLoweringBasis {
        match self {
            Self::ReadmissionWitnessMismatch { basis, .. }
            | Self::ReadmissionPlannedCountersMismatch { basis, .. }
            | Self::RebindWitnessMismatch { basis, .. }
            | Self::CoverageDenied { basis, .. }
            | Self::LifecycleFamilyMismatch { basis, .. }
            | Self::KeyDomainMismatch { basis, .. }
            | Self::CurrentCoverageMismatch { basis, .. }
            | Self::ExecutedCounterWitnessPlanBindingMismatch { expected: basis, .. } => basis,
            Self::ExecutedCounterWitnessPathMismatch { expected, .. }
            | Self::ObservedCounterBasisMismatch { expected, .. } => expected,
        }
    }

    pub const fn spent_cost_receipt(self) -> S8AccessAttemptCostReceipt {
        match self {
            Self::ExecutedCounterWitnessPlanBindingMismatch { observed, .. }
            | Self::ExecutedCounterWitnessPathMismatch { observed, .. } => {
                S8AccessAttemptCostReceipt::DeniedObservedExecutionCost {
                    fingerprint: observed.basis().fingerprint(),
                    path_kind: observed.basis().path_kind(),
                    observed: observed.snapshot(),
                    counter_strength: observed.strength(),
                }
            }
            Self::ObservedCounterBasisMismatch { observed, .. } => S8AccessAttemptCostReceipt::DeniedObservedExecutionCost {
                fingerprint: observed.basis().fingerprint(),
                path_kind: observed.basis().path_kind(),
                observed: observed.snapshot(),
                counter_strength: observed.strength(),
            },
            denial => {
                let basis = denial.basis();
                S8AccessAttemptCostReceipt::NoExecutionCountersSpent {
                    fingerprint: basis.fingerprint(),
                    path_kind: basis.path_kind(),
                }
            }
        }
    }
}

impl S8AccessLoweringDeferred {
    pub const fn basis(self) -> S8AccessLoweringBasis {
        match self {
            Self::RuntimeLeaseRequired { basis } => basis,
        }
    }

    pub const fn spent_cost_receipt(self) -> S8AccessAttemptCostReceipt {
        let basis = self.basis();
        S8AccessAttemptCostReceipt::NoExecutionCountersSpent {
            fingerprint: basis.fingerprint(),
            path_kind: basis.path_kind(),
        }
    }
}
