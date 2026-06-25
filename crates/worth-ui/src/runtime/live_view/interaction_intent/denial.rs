#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewInteractionIntentDenial {
    InvalidInteractionId {
        interaction_id: String,
    },
    UnsupportedKind {
        interaction_id: String,
        kind: String,
    },
    UnsupportedEffect {
        interaction_id: String,
        effect: String,
    },
    UnknownReadiness {
        interaction_id: String,
        readiness_id: String,
    },
    UnknownPayload {
        interaction_id: String,
        payload_id: String,
    },
    PrimitiveFlowLayout {
        interaction_id: String,
        prop_key: String,
        raw_value: String,
        expected: String,
        denial_digest: u64,
    },
    PrimitiveAppearanceState {
        interaction_id: String,
        prop_key: String,
        raw_value: String,
        expected: String,
        denial_digest: u64,
    },
    PrimitiveEventGeometry {
        interaction_id: String,
        prop_key: String,
        raw_value: String,
        expected: String,
        denial_digest: u64,
    },
}

impl WorthUiLiveViewInteractionIntentDenial {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidInteractionId { .. } => "live_view_interaction.invalid_id",
            Self::UnsupportedKind { .. } => "live_view_interaction.unsupported_kind",
            Self::UnsupportedEffect { .. } => "live_view_interaction.unsupported_effect",
            Self::UnknownReadiness { .. } => "live_view_interaction.unknown_readiness",
            Self::UnknownPayload { .. } => "live_view_interaction.unknown_payload",
            Self::PrimitiveFlowLayout { .. } => {
                "live_view_interaction.primitive_flow_layout_denied"
            }
            Self::PrimitiveAppearanceState { .. } => {
                "live_view_interaction.primitive_appearance_state_denied"
            }
            Self::PrimitiveEventGeometry { .. } => {
                "live_view_interaction.primitive_event_geometry_denied"
            }
        }
    }
}
