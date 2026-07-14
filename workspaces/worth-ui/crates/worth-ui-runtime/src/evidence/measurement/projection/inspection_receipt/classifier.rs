use worth_ui_inspection::{
    UiInspectionMeasurementFailureSource, UiInspectionSupportPosture, UiInspectionSupportReport,
};

use crate::evidence::measurement::basis::UiMeasurementEvidenceSlot;
use crate::evidence::measurement::basis::{UiMeasurementBasis, UiMeasurementBasisDenial};

pub(crate) fn classify_failure_source(
    support_report: &UiInspectionSupportReport,
    basis: Option<&UiMeasurementBasis>,
) -> Option<UiInspectionMeasurementFailureSource> {
    if !matches!(
        support_report.posture(),
        UiInspectionSupportPosture::Supported
    ) {
        return Some(UiInspectionMeasurementFailureSource::DeclarationPosture);
    }

    let basis = basis?;
    match basis.denial_posture() {
        Some(UiMeasurementBasisDenial::GenerationIncompatible { .. }) => {
            Some(UiInspectionMeasurementFailureSource::CompatibilityMismatch)
        }
        Some(UiMeasurementBasisDenial::MissingEvidence { slot })
        | Some(UiMeasurementBasisDenial::ConflictingEvidenceInputs { slot }) => {
            Some(classify_slot_source(*slot))
        }
        Some(UiMeasurementBasisDenial::MissingBasisSourceEvidence { .. })
        | Some(UiMeasurementBasisDenial::MissingOwnershipEvidence { .. })
        | Some(UiMeasurementBasisDenial::MissingRequiredMeasurementEvidence { .. }) => {
            Some(UiInspectionMeasurementFailureSource::HostEvidence)
        }
        None => (!basis.generation_compatibility().is_compatible())
            .then_some(UiInspectionMeasurementFailureSource::CompatibilityMismatch),
    }
}

pub(crate) fn classify_slot_source(
    slot: UiMeasurementEvidenceSlot,
) -> UiInspectionMeasurementFailureSource {
    match slot {
        UiMeasurementEvidenceSlot::QueryProjectionFactReceipt => {
            UiInspectionMeasurementFailureSource::QueryFacts
        }
        UiMeasurementEvidenceSlot::HostCapabilityReport
        | UiMeasurementEvidenceSlot::HostTextIntrinsicSize
        | UiMeasurementEvidenceSlot::HostFontMetrics
        | UiMeasurementEvidenceSlot::HostNativeControlIntrinsicSize
        | UiMeasurementEvidenceSlot::ViewportExtent
        | UiMeasurementEvidenceSlot::PortalAnchorRect
        | UiMeasurementEvidenceSlot::ScrollContainerViewport => {
            UiInspectionMeasurementFailureSource::HostEvidence
        }
    }
}
