use forge_store_budgets::PreExecutionBudgetDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BTreeReplayDenied {
    ArtifactFamily,
    SecurityScope,
    KeyDomain,
    ConcreteKey,
    ExactCoverage,
    Shape,
    RequestAdmission(crate::PhysicalAccessRequestAdmissionDenied),
    NoEligibleLayout,
    AmbiguousLayout,
    Cost(crate::AccessPlanCostDenial),
    Budget(PreExecutionBudgetDenial),
    UnexpectedSelectedOperation,
    Execution(crate::BaselineBTreeExecutionDenial),
}

pub(super) const fn map_artifact_denial(denial: crate::ArtifactFamilyDenial) -> BTreeReplayDenied {
    match denial {
        crate::ArtifactFamilyDenial::CrossKeyScopePartitionDenied
        | crate::ArtifactFamilyDenial::CrossTenantScopePartitionDenied
        | crate::ArtifactFamilyDenial::AuthenticityBoundaryDenied
        | crate::ArtifactFamilyDenial::CustodyBoundaryDenied => BTreeReplayDenied::SecurityScope,
        _ => BTreeReplayDenied::ArtifactFamily,
    }
}

pub(super) const fn map_key_domain_denial(
    denial: crate::ArtifactFamilyDenial,
) -> BTreeReplayDenied {
    match denial {
        crate::ArtifactFamilyDenial::CrossKeyScopePartitionDenied
        | crate::ArtifactFamilyDenial::CrossTenantScopePartitionDenied
        | crate::ArtifactFamilyDenial::AuthenticityBoundaryDenied
        | crate::ArtifactFamilyDenial::CustodyBoundaryDenied
        | crate::ArtifactFamilyDenial::SecurityAuthorityMismatch => {
            BTreeReplayDenied::SecurityScope
        }
        _ => BTreeReplayDenied::KeyDomain,
    }
}

pub(super) const fn map_selection_denial(
    denial: crate::AccessPlanSelectionDenied,
) -> BTreeReplayDenied {
    match denial {
        crate::AccessPlanSelectionDenied::NoEligibleAlternative => {
            BTreeReplayDenied::NoEligibleLayout
        }
        crate::AccessPlanSelectionDenied::OverlappingEligibleStrategyAuthority { .. } => {
            BTreeReplayDenied::AmbiguousLayout
        }
        crate::AccessPlanSelectionDenied::CostDenied(denial) => BTreeReplayDenied::Cost(denial),
        crate::AccessPlanSelectionDenied::BudgetDenied(denial) => BTreeReplayDenied::Budget(denial),
    }
}
