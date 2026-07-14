use worth_store_budgets::PreExecutionBudgetDenial;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BTreeReplayDenied {
    SecurityScope,
    Budget(PreExecutionBudgetDenial),
    Execution(Box<crate::BaselineBTreeExecutionDenial>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BTreeReplayDenialKind {
    SecurityScope,
    Budget,
    Execution,
}

impl BTreeReplayDenialKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SecurityScope => "security_scope",
            Self::Budget => "budget",
            Self::Execution => "execution",
        }
    }
}

impl BTreeReplayDenied {
    pub const fn kind(&self) -> BTreeReplayDenialKind {
        match self {
            Self::SecurityScope => BTreeReplayDenialKind::SecurityScope,
            Self::Budget(_) => BTreeReplayDenialKind::Budget,
            Self::Execution(_) => BTreeReplayDenialKind::Execution,
        }
    }
}

pub(super) const fn map_security_denial(denial: crate::ArtifactFamilyDenial) -> BTreeReplayDenied {
    match denial {
        crate::ArtifactFamilyDenial::CrossKeyScopePartitionDenied
        | crate::ArtifactFamilyDenial::CrossTenantScopePartitionDenied
        | crate::ArtifactFamilyDenial::AuthenticityBoundaryDenied
        | crate::ArtifactFamilyDenial::CustodyBoundaryDenied
        | crate::ArtifactFamilyDenial::SecurityAuthorityMismatch => {
            BTreeReplayDenied::SecurityScope
        }
        _ => BTreeReplayDenied::SecurityScope,
    }
}

pub(super) fn map_selection_denial(denial: crate::AccessPlanSelectionDenied) -> BTreeReplayDenied {
    match denial {
        crate::AccessPlanSelectionDenied::BudgetDenied(denial) => BTreeReplayDenied::Budget(denial),
        crate::AccessPlanSelectionDenied::NoEligibleAlternative
        | crate::AccessPlanSelectionDenied::CostDenied(_) => {
            unreachable!("admitted B-tree replay has a fixed eligible and costed operation")
        }
    }
}
