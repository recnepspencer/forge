use crate::declaration::{UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementOwnershipPosture};

use super::{UiMeasurementEvidenceCategory, UiMeasurementGenerationCompatibility};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMeasurementEvidenceSlot {
    QueryProjectionFactReceipt,
    HostCapabilityReport,
    HostFontMetrics,
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
