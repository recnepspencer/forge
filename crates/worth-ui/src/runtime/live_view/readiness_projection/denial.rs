#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewReadinessProjectionDenial {
    InvalidReadinessId {
        readiness_id: String,
    },
    EmptyRequiredSet {
        readiness_id: String,
    },
    UnknownRequiredBinding {
        readiness_id: String,
        binding_id: String,
    },
}

impl WorthUiLiveViewReadinessProjectionDenial {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidReadinessId { .. } => "live_view_readiness.invalid_id",
            Self::EmptyRequiredSet { .. } => "live_view_readiness.empty_required_set",
            Self::UnknownRequiredBinding { .. } => "live_view_readiness.unknown_required_binding",
        }
    }
}
