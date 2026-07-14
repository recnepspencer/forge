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
    Cost(crate::AccessPlanCostDenial),
    Budget(PreExecutionBudgetDenial),
    UnexpectedSelectedOperation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LsmMaintenanceAdmissionDenialKind {
    ArtifactFamily,
    SecurityScope,
    KeyDomain,
    ConcreteKey,
    Shape,
    RequestAdmission,
    NoEligibleLayout,
    Cost,
    Budget,
    UnexpectedSelectedOperation,
}

impl LsmMaintenanceAdmissionDenialKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArtifactFamily => "artifact_family",
            Self::SecurityScope => "security_scope",
            Self::KeyDomain => "key_domain",
            Self::ConcreteKey => "concrete_key",
            Self::Shape => "shape",
            Self::RequestAdmission => "request_admission",
            Self::NoEligibleLayout => "no_eligible_layout",
            Self::Cost => "cost",
            Self::Budget => "budget",
            Self::UnexpectedSelectedOperation => "unexpected_selected_operation",
        }
    }
}

impl LsmMaintenanceAdmissionDenied {
    pub const fn kind(&self) -> LsmMaintenanceAdmissionDenialKind {
        match self {
            Self::ArtifactFamily => LsmMaintenanceAdmissionDenialKind::ArtifactFamily,
            Self::SecurityScope => LsmMaintenanceAdmissionDenialKind::SecurityScope,
            Self::KeyDomain => LsmMaintenanceAdmissionDenialKind::KeyDomain,
            Self::ConcreteKey => LsmMaintenanceAdmissionDenialKind::ConcreteKey,
            Self::Shape => LsmMaintenanceAdmissionDenialKind::Shape,
            Self::RequestAdmission(_) => LsmMaintenanceAdmissionDenialKind::RequestAdmission,
            Self::NoEligibleLayout => LsmMaintenanceAdmissionDenialKind::NoEligibleLayout,
            Self::Cost(_) => LsmMaintenanceAdmissionDenialKind::Cost,
            Self::Budget(_) => LsmMaintenanceAdmissionDenialKind::Budget,
            Self::UnexpectedSelectedOperation => {
                LsmMaintenanceAdmissionDenialKind::UnexpectedSelectedOperation
            }
        }
    }
}

pub(super) const fn map_selection_denial(
    denial: crate::AccessPlanSelectionDenied,
) -> LsmMaintenanceAdmissionDenied {
    match denial {
        crate::AccessPlanSelectionDenied::NoEligibleAlternative => {
            LsmMaintenanceAdmissionDenied::NoEligibleLayout
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
