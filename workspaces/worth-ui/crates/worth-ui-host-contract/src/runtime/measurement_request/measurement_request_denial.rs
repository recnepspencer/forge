use super::{UiMeasurementEvidenceFamily, UiMeasurementRequestFamily};
use crate::runtime::WorthUiHostCapability;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiForbiddenHostAuthorityAsk {
    FinalLayoutSize,
    OverflowDecision,
    ScrollExtentAuthority,
    PortalPositionDecision,
    AllocationBox,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiMeasurementRequestDenial {
    ForbiddenAuthorityAsk {
        ask: UiForbiddenHostAuthorityAsk,
    },
    IncompatibleEvidenceFamily {
        family: UiMeasurementRequestFamily,
        evidence_family: UiMeasurementEvidenceFamily,
    },
    MissingCapability {
        required_capabilities: Box<[WorthUiHostCapability]>,
    },
    AmbiguousCapability {
        required_capabilities: Box<[WorthUiHostCapability]>,
    },
    DiagnosticOnlyCapability {
        required_capabilities: Box<[WorthUiHostCapability]>,
    },
}
