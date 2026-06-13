#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLaneImpactClassification {
    Unaffected,
    LaneAffecting { reason: &'static str },
}

impl WorthUiLaneImpactClassification {
    pub(crate) fn surface_semantics_changed() -> Self {
        Self::LaneAffecting {
            reason: "surface-semantics-changed",
        }
    }

    pub fn requires_lane_parity(&self) -> bool {
        matches!(self, Self::LaneAffecting { .. })
    }

    pub fn reason(&self) -> Option<&'static str> {
        match self {
            Self::Unaffected => None,
            Self::LaneAffecting { reason } => Some(reason),
        }
    }
}
