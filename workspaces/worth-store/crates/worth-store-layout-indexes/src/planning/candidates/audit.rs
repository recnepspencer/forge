use crate::catalog::AuthorityRole;
use crate::strategy::LayoutStrategyFamily;

use super::super::denial::SelectionCandidateRejection;
use super::super::selection_basis::SelectionCandidateEligibility;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionCandidateAudit {
    family: LayoutStrategyFamily,
    authority_role: AuthorityRole,
    outcome: SelectionCandidateOutcome,
}

impl SelectionCandidateAudit {
    pub(crate) const fn new(
        family: LayoutStrategyFamily,
        authority_role: AuthorityRole,
        outcome: SelectionCandidateOutcome,
    ) -> Self {
        Self {
            family,
            authority_role,
            outcome,
        }
    }

    pub const fn family(&self) -> LayoutStrategyFamily {
        self.family
    }

    pub const fn authority_role(&self) -> AuthorityRole {
        self.authority_role
    }

    pub const fn outcome(&self) -> &SelectionCandidateOutcome {
        &self.outcome
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionCandidateOutcome {
    Eligible(SelectionCandidateEligibility),
    Rejected(SelectionCandidateRejection),
}
