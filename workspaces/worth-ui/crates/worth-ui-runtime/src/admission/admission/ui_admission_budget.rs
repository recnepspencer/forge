#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiAdmissionSelectionBudget {
    Unbounded,
    OrdinaryLaneBudget { lane_limit: u8 },
}

impl UiAdmissionSelectionBudget {
    pub const fn unbounded() -> Self {
        Self::Unbounded
    }

    pub const fn ordinary_lane_budget(lane_limit: u8) -> Self {
        Self::OrdinaryLaneBudget { lane_limit }
    }

    pub const fn admits_lane_cost(self, lane_cost: u8) -> bool {
        match self {
            Self::Unbounded => true,
            Self::OrdinaryLaneBudget { lane_limit } => lane_cost <= lane_limit,
        }
    }
}
