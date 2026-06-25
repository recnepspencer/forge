use crate::runtime::live_view::digest::digest_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewConditionalProjectionDenial {
    UnknownControl {
        control_id: String,
    },
    UnknownConditionBinding {
        control_id: String,
        binding_id: String,
    },
    UnsupportedCondition {
        control_id: String,
        condition: String,
    },
    UnsupportedParticipation {
        control_id: String,
        posture: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewConditionalProjectionAdmissionReport {
    denials: Vec<WorthUiLiveViewConditionalProjectionDenial>,
    denial_set_digest: u64,
}

impl WorthUiLiveViewConditionalProjectionDenial {
    pub fn code(&self) -> &'static str {
        match self {
            Self::UnknownControl { .. } => "live_view_condition.unknown_control",
            Self::UnknownConditionBinding { .. } => "live_view_condition.unknown_binding",
            Self::UnsupportedCondition { .. } => "live_view_condition.unsupported_condition",
            Self::UnsupportedParticipation { .. } => {
                "live_view_condition.unsupported_participation"
            }
        }
    }
}

impl WorthUiLiveViewConditionalProjectionAdmissionReport {
    pub(crate) fn denied(denials: Vec<WorthUiLiveViewConditionalProjectionDenial>) -> Self {
        let denial_set_digest = digest_parts(denials.iter().map(|denial| denial.code()));
        Self {
            denials,
            denial_set_digest,
        }
    }

    pub fn denials(&self) -> &[WorthUiLiveViewConditionalProjectionDenial] {
        &self.denials
    }

    pub fn denial_set_digest(&self) -> u64 {
        self.denial_set_digest
    }
}
