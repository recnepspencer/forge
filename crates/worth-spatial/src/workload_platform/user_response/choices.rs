#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthPolicyDecision {
    TreatCandidateLoopAsInsideFace,
    TreatCandidateLoopAsOutsideFace,
    PauseForManualInspection,
}

impl WorthPolicyDecision {
    pub const fn treat_candidate_as_inside_face() -> Self {
        Self::TreatCandidateLoopAsInsideFace
    }

    pub const fn treat_candidate_as_outside_face() -> Self {
        Self::TreatCandidateLoopAsOutsideFace
    }

    pub const fn pause_for_manual_inspection() -> Self {
        Self::PauseForManualInspection
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::TreatCandidateLoopAsInsideFace => "Treat the candidate loop as inside this face.",
            Self::TreatCandidateLoopAsOutsideFace => {
                "Treat the candidate loop as outside this face."
            }
            Self::PauseForManualInspection => "Pause boolean certification for manual inspection.",
        }
    }
}

pub(crate) fn overlap_policy_choices() -> Vec<WorthPolicyDecision> {
    vec![
        WorthPolicyDecision::treat_candidate_as_inside_face(),
        WorthPolicyDecision::treat_candidate_as_outside_face(),
        WorthPolicyDecision::pause_for_manual_inspection(),
    ]
}
