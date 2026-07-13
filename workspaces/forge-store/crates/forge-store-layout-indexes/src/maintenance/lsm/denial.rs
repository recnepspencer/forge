use forge_store_budgets::PreExecutionBudgetDenial;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LsmMaintenanceAdmissionDenied {
    ArtifactFamily,
    SecurityScope,
    KeyDomain,
    ConcreteKey,
    Shape,
    RequestAdmission(crate::PhysicalAccessRequestAdmissionDenied),
    NoEligibleLayout,
    AmbiguousLayout,
    Cost(crate::AccessPlanCostDenial),
    Budget(PreExecutionBudgetDenial),
    UnexpectedSelectedOperation,
}

pub(super) const fn map_selection_denial(
    denial: crate::AccessPlanSelectionDenied,
) -> LsmMaintenanceAdmissionDenied {
    match denial {
        crate::AccessPlanSelectionDenied::NoEligibleAlternative => {
            LsmMaintenanceAdmissionDenied::NoEligibleLayout
        }
        crate::AccessPlanSelectionDenied::OverlappingEligibleStrategyAuthority { .. } => {
            LsmMaintenanceAdmissionDenied::AmbiguousLayout
        }
        crate::AccessPlanSelectionDenied::CostDenied(denial) => {
            LsmMaintenanceAdmissionDenied::Cost(denial)
        }
        crate::AccessPlanSelectionDenied::BudgetDenied(denial) => {
            LsmMaintenanceAdmissionDenied::Budget(denial)
        }
    }
}

pub(super) const fn map_key_domain_denial(
    denial: crate::ArtifactFamilyDenial,
) -> LsmMaintenanceAdmissionDenied {
    match denial {
        crate::ArtifactFamilyDenial::CrossKeyScopePartitionDenied
        | crate::ArtifactFamilyDenial::CrossTenantScopePartitionDenied
        | crate::ArtifactFamilyDenial::AuthenticityBoundaryDenied
        | crate::ArtifactFamilyDenial::CustodyBoundaryDenied
        | crate::ArtifactFamilyDenial::SecurityAuthorityMismatch => {
            LsmMaintenanceAdmissionDenied::SecurityScope
        }
        _ => LsmMaintenanceAdmissionDenied::KeyDomain,
    }
}
