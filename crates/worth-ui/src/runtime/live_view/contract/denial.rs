use super::WorthUiLiveViewStateValueKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewDenial {
    InvalidLiveViewId {
        live_view_id: String,
    },
    EmptyStateBindings {
        live_view_id: String,
    },
    InvalidBindingId {
        binding_id: String,
    },
    DuplicateBindingId {
        binding_id: String,
    },
    InvalidStateFact {
        binding_id: String,
        state_fact: String,
    },
    StaleTargetBinding {
        slot_name: String,
        surface_id: String,
        expected_component_id: String,
        actual_component_id: Option<String>,
    },
    UnsupportedValueKind {
        binding_id: String,
        value_kind: String,
    },
    UnsupportedWritePosture {
        binding_id: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewStateEditDenial {
    StaleTargetBinding {
        binding_id: String,
        slot_name: String,
        surface_id: String,
        expected_component_id: String,
        actual_component_id: Option<String>,
    },
    ValueKindMismatch {
        binding_id: String,
        expected: WorthUiLiveViewStateValueKind,
        actual: WorthUiLiveViewStateValueKind,
    },
    ReadOnlyBinding {
        binding_id: String,
    },
}

impl WorthUiLiveViewDenial {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidLiveViewId { .. } => "live_view.invalid_id",
            Self::EmptyStateBindings { .. } => "live_view.empty_state_bindings",
            Self::InvalidBindingId { .. } => "live_view.invalid_binding_id",
            Self::DuplicateBindingId { .. } => "live_view.duplicate_binding_id",
            Self::InvalidStateFact { .. } => "live_view.invalid_state_fact",
            Self::StaleTargetBinding { .. } => "live_view.stale_target_binding",
            Self::UnsupportedValueKind { .. } => "live_view.unsupported_value_kind",
            Self::UnsupportedWritePosture { .. } => "live_view.unsupported_write_posture",
        }
    }
}
