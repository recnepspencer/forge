use crate::runtime::{WorthUiLaneMeaningParity, WorthUiPlanExecutionLane};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLaneTransitionParity {
    identity_basis: String,
    active_lane: Option<WorthUiPlanExecutionLane>,
    candidate_lane: Option<WorthUiPlanExecutionLane>,
    mechanics_changed: bool,
    meaning_parity: Vec<WorthUiLaneMeaningParity>,
}

impl WorthUiLaneTransitionParity {
    pub(crate) fn new(
        identity_basis: impl Into<String>,
        active_lane: Option<WorthUiPlanExecutionLane>,
        candidate_lane: Option<WorthUiPlanExecutionLane>,
        mechanics_changed: bool,
        meaning_parity: Vec<WorthUiLaneMeaningParity>,
    ) -> Self {
        Self {
            identity_basis: identity_basis.into(),
            active_lane,
            candidate_lane,
            mechanics_changed,
            meaning_parity,
        }
    }

    pub fn identity_basis(&self) -> &str {
        &self.identity_basis
    }

    pub fn active_lane(&self) -> Option<WorthUiPlanExecutionLane> {
        self.active_lane
    }

    pub fn candidate_lane(&self) -> Option<WorthUiPlanExecutionLane> {
        self.candidate_lane
    }

    pub fn meaning_parity(&self) -> &[WorthUiLaneMeaningParity] {
        &self.meaning_parity
    }

    pub fn mechanics_changed(&self) -> bool {
        self.mechanics_changed
    }
}
