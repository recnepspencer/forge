use crate::strategy::registry::{LayoutAdmissionDenial, LayoutAdmissionDenialCase};
use crate::strategy::LayoutStrategyFamily;
use forge_store_budgets::PreExecutionBudgetDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionCandidateRejection {
    RegistryDenied(LayoutAdmissionDenial),
    OperationUnsupported {
        family: LayoutStrategyFamily,
        shape: crate::observation::AccessShape,
    },
    MissingPlannedCounterEnvelope,
    NotApplicableToExplicitDegradedScan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectionCandidateRejectionCase {
    RegistryDenied(LayoutAdmissionDenialCase),
    OperationUnsupported,
    MissingPlannedCounterEnvelope,
    NotApplicableToExplicitDegradedScan,
}

impl SelectionCandidateRejectionCase {
    pub const ALL: [Self; 20] = [
        Self::RegistryDenied(LayoutAdmissionDenialCase::StrategyVocabularyDenied),
        Self::RegistryDenied(LayoutAdmissionDenialCase::RequestedLaneDoesNotMatchFamilyLane),
        Self::RegistryDenied(LayoutAdmissionDenialCase::RequestedScopeDoesNotMatchKeyDomain),
        Self::RegistryDenied(
            LayoutAdmissionDenialCase::MaintenanceModeIncompatibleWithRequestedLane,
        ),
        Self::RegistryDenied(LayoutAdmissionDenialCase::MutationShapeIncompatibleWithStrategy),
        Self::RegistryDenied(LayoutAdmissionDenialCase::MigrationPostureIncompatibleWithStrategy),
        Self::RegistryDenied(LayoutAdmissionDenialCase::StrategyDoesNotSupportRequestedCapability),
        Self::RegistryDenied(LayoutAdmissionDenialCase::ComparatorLawRequired),
        Self::RegistryDenied(LayoutAdmissionDenialCase::PrefixLawRequired),
        Self::RegistryDenied(LayoutAdmissionDenialCase::RangeBoundLawRequired),
        Self::RegistryDenied(LayoutAdmissionDenialCase::HashEqualityLawDoesNotMatchKeyDomain),
        Self::RegistryDenied(LayoutAdmissionDenialCase::CompositeOrderingLawDoesNotMatchKeyDomain),
        Self::RegistryDenied(LayoutAdmissionDenialCase::CoverageFamilyDoesNotMatchStrategy),
        Self::RegistryDenied(
            LayoutAdmissionDenialCase::LiveExactMaintenanceWitnessDoesNotMatchStrategy,
        ),
        Self::RegistryDenied(
            LayoutAdmissionDenialCase::LiveExactMaintenanceCoverageDoesNotMatchRequest,
        ),
        Self::RegistryDenied(LayoutAdmissionDenialCase::ExactMaterializationRequired),
        Self::RegistryDenied(LayoutAdmissionDenialCase::ExactCoverageDenied),
        Self::OperationUnsupported,
        Self::MissingPlannedCounterEnvelope,
        Self::NotApplicableToExplicitDegradedScan,
    ];
}

impl SelectionCandidateRejection {
    pub const fn case(&self) -> SelectionCandidateRejectionCase {
        match self {
            Self::RegistryDenied(denial) => {
                SelectionCandidateRejectionCase::RegistryDenied(denial.case())
            }
            Self::OperationUnsupported { .. } => {
                SelectionCandidateRejectionCase::OperationUnsupported
            }
            Self::MissingPlannedCounterEnvelope => {
                SelectionCandidateRejectionCase::MissingPlannedCounterEnvelope
            }
            Self::NotApplicableToExplicitDegradedScan => {
                SelectionCandidateRejectionCase::NotApplicableToExplicitDegradedScan
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessPlanSelectionDenied {
    NoEligibleAlternative,
    OverlappingEligibleStrategyAuthority {
        first_family: LayoutStrategyFamily,
        second_family: LayoutStrategyFamily,
    },
    CostDenied(super::AccessPlanCostDenial),
    BudgetDenied(PreExecutionBudgetDenial),
}
