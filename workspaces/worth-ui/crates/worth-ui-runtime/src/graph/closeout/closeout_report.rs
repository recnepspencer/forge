use crate::graph::closeout::milestone33_closeout_profile::MILESTONE33_CLOSEOUT_PROFILE;
use crate::graph::{
    UiGraphClosedSemanticLane, UiGraphCloseoutGuarantee, UiGraphCloseoutNonGoal,
    UiGraphInspectionSupportReport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiGraphCloseoutReport {
    closed_semantic_lanes: &'static [UiGraphClosedSemanticLane],
    guarantees: &'static [UiGraphCloseoutGuarantee],
    non_goals: &'static [UiGraphCloseoutNonGoal],
    inspection_support: UiGraphInspectionSupportReport,
}

impl UiGraphCloseoutReport {
    pub(crate) const fn new(
        closed_semantic_lanes: &'static [UiGraphClosedSemanticLane],
        guarantees: &'static [UiGraphCloseoutGuarantee],
        non_goals: &'static [UiGraphCloseoutNonGoal],
        inspection_support: UiGraphInspectionSupportReport,
    ) -> Self {
        Self {
            closed_semantic_lanes,
            guarantees,
            non_goals,
            inspection_support,
        }
    }

    pub const fn milestone33() -> Self {
        MILESTONE33_CLOSEOUT_PROFILE
    }

    pub const fn closed_semantic_lanes(self) -> &'static [UiGraphClosedSemanticLane] {
        self.closed_semantic_lanes
    }

    pub const fn guarantees(self) -> &'static [UiGraphCloseoutGuarantee] {
        self.guarantees
    }

    pub const fn non_goals(self) -> &'static [UiGraphCloseoutNonGoal] {
        self.non_goals
    }

    pub const fn inspection_support(self) -> UiGraphInspectionSupportReport {
        self.inspection_support
    }
}
