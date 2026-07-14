use forge_store_budgets::PreExecutionBudgetDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutReadAdmissionDenied {
    ArtifactFamily,
    SecurityScope,
    KeyDomain,
    ConcreteKey,
    ExactCoverage(crate::MaterializationDenial),
    ReadShapeUnsupported,
    RequestAdmission(crate::PhysicalAccessRequestAdmissionDenied),
    NoEligibleLayout,
    StrategyInvariant(crate::strategy::StrategyDenial),
    Cost(crate::AccessPlanCostDenial),
    Budget(PreExecutionBudgetDenial),
    UnexpectedSelectedOperation,
    StaleMaterialization(crate::StaleLayoutMaterialization),
    CounterEnvelope(crate::CounterEnvelopeViolation),
}

pub(super) const fn map_artifact_denial(
    denial: crate::ArtifactFamilyDenial,
) -> LayoutReadAdmissionDenied {
    match denial {
        crate::ArtifactFamilyDenial::CrossKeyScopePartitionDenied
        | crate::ArtifactFamilyDenial::CrossTenantScopePartitionDenied
        | crate::ArtifactFamilyDenial::AuthenticityBoundaryDenied
        | crate::ArtifactFamilyDenial::CustodyBoundaryDenied => {
            LayoutReadAdmissionDenied::SecurityScope
        }
        _ => LayoutReadAdmissionDenied::ArtifactFamily,
    }
}

pub(super) const fn map_key_domain_denial(
    denial: crate::ArtifactFamilyDenial,
) -> LayoutReadAdmissionDenied {
    match denial {
        crate::ArtifactFamilyDenial::CrossKeyScopePartitionDenied
        | crate::ArtifactFamilyDenial::CrossTenantScopePartitionDenied
        | crate::ArtifactFamilyDenial::AuthenticityBoundaryDenied
        | crate::ArtifactFamilyDenial::CustodyBoundaryDenied
        | crate::ArtifactFamilyDenial::SecurityAuthorityMismatch => {
            LayoutReadAdmissionDenied::SecurityScope
        }
        _ => LayoutReadAdmissionDenied::KeyDomain,
    }
}

pub(super) const fn map_selection_denial(
    denial: crate::AccessPlanSelectionDenied,
) -> LayoutReadAdmissionDenied {
    match denial {
        crate::AccessPlanSelectionDenied::NoEligibleAlternative => {
            LayoutReadAdmissionDenied::NoEligibleLayout
        }
        crate::AccessPlanSelectionDenied::CostDenied(denial) => {
            LayoutReadAdmissionDenied::Cost(denial)
        }
        crate::AccessPlanSelectionDenied::BudgetDenied(denial) => {
            LayoutReadAdmissionDenied::Budget(denial)
        }
    }
}
