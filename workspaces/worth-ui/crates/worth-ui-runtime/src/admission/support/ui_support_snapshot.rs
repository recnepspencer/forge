use worth_ui_inspection::UiInspectionSupportPosture;

use crate::admission::{UiAdmissionTarget, UiSupportPosture};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiSupportSnapshot {
    target: UiAdmissionTarget,
    posture: UiSupportPosture,
}

impl UiSupportSnapshot {
    pub(crate) fn new(target: UiAdmissionTarget, posture: UiSupportPosture) -> Self {
        Self { target, posture }
    }

    pub fn target(&self) -> &UiAdmissionTarget {
        &self.target
    }

    pub fn posture(&self) -> &UiSupportPosture {
        &self.posture
    }

    pub fn inspection_posture(&self) -> UiInspectionSupportPosture {
        match self.posture() {
            UiSupportPosture::Supported { .. } => UiInspectionSupportPosture::Supported,
            UiSupportPosture::DiagnosticOnly { .. } => UiInspectionSupportPosture::DiagnosticOnly,
            UiSupportPosture::Unsupported { .. } => UiInspectionSupportPosture::Unsupported,
            UiSupportPosture::WrongWorld { .. } => UiInspectionSupportPosture::WrongWorld,
            UiSupportPosture::Deferred { .. } => UiInspectionSupportPosture::Deferred,
        }
    }
}
