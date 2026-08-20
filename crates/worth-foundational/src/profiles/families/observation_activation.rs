use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ObservationActivationProfile {
    OnDemand,
    Continuous,
}

impl ObservationActivationProfile {
    pub const fn token(self) -> &'static str {
        match self {
            Self::OnDemand => "on-demand",
            Self::Continuous => "continuous",
        }
    }
}
