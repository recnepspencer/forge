#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ProjectionFactParityLane {
    Live,
    Projected,
    ProjectionConsumed,
    Retained,
    Replayed,
    Transformed,
    Recovered,
    LocalRebuild,
    Diagnostics,
}

impl ProjectionFactParityLane {
    pub const REQUIRED: [Self; 9] = [
        Self::Live,
        Self::Projected,
        Self::ProjectionConsumed,
        Self::Retained,
        Self::Replayed,
        Self::Transformed,
        Self::Recovered,
        Self::LocalRebuild,
        Self::Diagnostics,
    ];

    pub fn human_name(self) -> &'static str {
        match self {
            Self::Live => "live geometry lane",
            Self::Projected => "projected geometry lane",
            Self::ProjectionConsumed => "projection-consumed fact lane",
            Self::Retained => "retained fact lane",
            Self::Replayed => "replayed retained fact lane",
            Self::Transformed => "transformed geometry lane",
            Self::Recovered => "recovery lane",
            Self::LocalRebuild => "local rebuild lane",
            Self::Diagnostics => "diagnostic lane",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionFactParityLaneStatus {
    Admitted,
    Denied,
    PolicyRequired,
}
