use super::{
    UiObligationClosedSemanticLane, UiObligationCloseoutGuarantee, UiObligationCloseoutNonGoal,
};
use crate::obligations::closeout::milestone34_closeout_profile::MILESTONE34_CLOSEOUT_PROFILE;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiObligationCloseoutReport {
    closed_semantic_lanes: &'static [UiObligationClosedSemanticLane],
    guarantees: &'static [UiObligationCloseoutGuarantee],
    non_goals: &'static [UiObligationCloseoutNonGoal],
}

impl UiObligationCloseoutReport {
    pub(crate) const fn new(
        closed_semantic_lanes: &'static [UiObligationClosedSemanticLane],
        guarantees: &'static [UiObligationCloseoutGuarantee],
        non_goals: &'static [UiObligationCloseoutNonGoal],
    ) -> Self {
        Self {
            closed_semantic_lanes,
            guarantees,
            non_goals,
        }
    }

    pub const fn milestone34() -> Self {
        MILESTONE34_CLOSEOUT_PROFILE
    }

    pub const fn closed_semantic_lanes(self) -> &'static [UiObligationClosedSemanticLane] {
        self.closed_semantic_lanes
    }

    pub const fn guarantees(self) -> &'static [UiObligationCloseoutGuarantee] {
        self.guarantees
    }

    pub const fn non_goals(self) -> &'static [UiObligationCloseoutNonGoal] {
        self.non_goals
    }
}
