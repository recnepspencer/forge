use crate::runtime::live_view::digest::digest_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewControlProjectionDenial {
    InvalidControlId {
        control_id: String,
    },
    DuplicateControlId {
        control_id: String,
    },
    UnknownBinding {
        control_id: String,
        binding_id: String,
    },
    UnsupportedProjectionKind {
        control_id: String,
        projection_kind: String,
    },
    UnregisteredComponent {
        control_id: String,
        component_id: String,
    },
    MissingOptions {
        control_id: String,
    },
    UnsupportedOptionSource {
        control_id: String,
        option_source: String,
    },
    PrimitiveFlowLayout {
        control_id: String,
        prop_key: String,
        raw_value: String,
        expected: String,
        denial_digest: u64,
    },
    PrimitiveAppearanceState {
        control_id: String,
        prop_key: String,
        raw_value: String,
        expected: String,
        denial_digest: u64,
    },
    PrimitiveEventGeometry {
        control_id: String,
        prop_key: String,
        raw_value: String,
        expected: String,
        denial_digest: u64,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewControlProjectionAdmissionReport {
    denials: Vec<WorthUiLiveViewControlProjectionDenial>,
    denial_set_digest: u64,
}

impl WorthUiLiveViewControlProjectionDenial {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidControlId { .. } => "live_view_control.invalid_id",
            Self::DuplicateControlId { .. } => "live_view_control.duplicate_id",
            Self::UnknownBinding { .. } => "live_view_control.unknown_binding",
            Self::UnsupportedProjectionKind { .. } => "live_view_control.unsupported_kind",
            Self::UnregisteredComponent { .. } => "live_view_control.unregistered_component",
            Self::MissingOptions { .. } => "live_view_control.missing_options",
            Self::UnsupportedOptionSource { .. } => "live_view_control.unsupported_option_source",
            Self::PrimitiveFlowLayout { .. } => "live_view_control.primitive_flow_layout_denied",
            Self::PrimitiveAppearanceState { .. } => {
                "live_view_control.primitive_appearance_state_denied"
            }
            Self::PrimitiveEventGeometry { .. } => {
                "live_view_control.primitive_event_geometry_denied"
            }
        }
    }
}

impl WorthUiLiveViewControlProjectionAdmissionReport {
    pub(crate) fn denied(denials: Vec<WorthUiLiveViewControlProjectionDenial>) -> Self {
        let denial_set_digest = digest_parts(denials.iter().map(|denial| denial.code()));
        Self {
            denials,
            denial_set_digest,
        }
    }

    pub fn denials(&self) -> &[WorthUiLiveViewControlProjectionDenial] {
        &self.denials
    }

    pub fn denial_set_digest(&self) -> u64 {
        self.denial_set_digest
    }
}
