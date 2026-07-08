use crate::declaration::{UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementOwnershipPosture};

use crate::evidence::measurement::{
    UiMeasurementEvidenceCategory, UiMeasurementGenerationCompatibility,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMeasurementEvidenceSlot {
    QueryProjectionFactReceipt,
    HostCapabilityReport,
    HostTextIntrinsicSize,
    HostFontMetrics,
    HostNativeControlIntrinsicSize,
    ViewportExtent,
    PortalAnchorRect,
    ScrollContainerViewport,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiMeasurementBasisDenial {
    GenerationIncompatible {
        compatibility: UiMeasurementGenerationCompatibility,
    },
    MissingEvidence {
        slot: UiMeasurementEvidenceSlot,
    },
    MissingBasisSourceEvidence {
        basis_source: UiDeclaredMeasurementBasisSource,
        slot: UiMeasurementEvidenceSlot,
    },
    MissingOwnershipEvidence {
        ownership_posture: UiDeclaredMeasurementOwnershipPosture,
        slot: UiMeasurementEvidenceSlot,
    },
    MissingRequiredMeasurementEvidence {
        category: UiMeasurementEvidenceCategory,
        slot: UiMeasurementEvidenceSlot,
    },
    ConflictingEvidenceInputs {
        slot: UiMeasurementEvidenceSlot,
    },
}
