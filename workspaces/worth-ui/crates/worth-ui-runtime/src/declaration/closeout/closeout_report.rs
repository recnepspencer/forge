use crate::declaration::{
    UiDeclarationClosedSemanticLane, UiDeclarationCloseoutGuarantee, UiDeclarationCloseoutNonGoal,
    UiDeclarationFamilyKind,
};
use crate::declaration::closeout::milestone32_closeout_profile::MILESTONE32_CLOSEOUT_PROFILE;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiDeclarationCloseoutReport {
    admitted_families: &'static [UiDeclarationFamilyKind],
    closed_semantic_lanes: &'static [UiDeclarationClosedSemanticLane],
    guarantees: &'static [UiDeclarationCloseoutGuarantee],
    non_goals: &'static [UiDeclarationCloseoutNonGoal],
}

impl UiDeclarationCloseoutReport {
    pub(crate) const fn new(
        admitted_families: &'static [UiDeclarationFamilyKind],
        closed_semantic_lanes: &'static [UiDeclarationClosedSemanticLane],
        guarantees: &'static [UiDeclarationCloseoutGuarantee],
        non_goals: &'static [UiDeclarationCloseoutNonGoal],
    ) -> Self {
        Self {
            admitted_families,
            closed_semantic_lanes,
            guarantees,
            non_goals,
        }
    }

    pub const fn milestone32() -> Self {
        MILESTONE32_CLOSEOUT_PROFILE
    }

    pub const fn admitted_families(self) -> &'static [UiDeclarationFamilyKind] {
        self.admitted_families
    }

    pub const fn closed_semantic_lanes(self) -> &'static [UiDeclarationClosedSemanticLane] {
        self.closed_semantic_lanes
    }

    pub const fn guarantees(self) -> &'static [UiDeclarationCloseoutGuarantee] {
        self.guarantees
    }

    pub const fn non_goals(self) -> &'static [UiDeclarationCloseoutNonGoal] {
        self.non_goals
    }
}
