//! Measurement inspection receipt projection — orchestration of named project/classify/assemble steps.

mod assembly;
mod classifier;
mod project_basis_input;
mod project_compatibility;
mod project_denial;
mod project_evidence_maps;

use worth_ui_inspection::{
    UiInspectionMeasurementDenialPosture, UiInspectionMeasurementEvidenceView,
    UiInspectionMeasurementFailureSource, UiInspectionMeasurementGenerationCompatibility,
    UiInspectionSupportReport,
};

use crate::evidence::measurement::basis::UiMeasurementBasis;

use assembly::measurement_view_from_parts;
use classifier::classify_failure_source;
use project_compatibility::project_generation_compatibility;
use project_denial::project_denial;

pub(crate) fn project_measurement_inspection_view(
    support_report: UiInspectionSupportReport,
    basis: Option<&UiMeasurementBasis>,
) -> UiInspectionMeasurementEvidenceView {
    measurement_view_from_parts(
        support_report,
        basis,
        basis.and_then(|basis| basis.denial_posture().map(project_denial)),
        basis.map(|basis| project_generation_compatibility(basis.generation_compatibility())),
        classify_failure_source(&support_report, basis),
    )
}

pub(crate) fn project_measurement_inspection_denial_view(
    support_report: UiInspectionSupportReport,
    denial_posture: UiInspectionMeasurementDenialPosture,
    failure_source: Option<UiInspectionMeasurementFailureSource>,
) -> UiInspectionMeasurementEvidenceView {
    measurement_view_from_parts(
        support_report,
        None,
        Some(denial_posture),
        None,
        failure_source,
    )
}

pub(crate) fn project_measurement_inspection_compatibility_view(
    support_report: UiInspectionSupportReport,
    compatibility: UiInspectionMeasurementGenerationCompatibility,
) -> UiInspectionMeasurementEvidenceView {
    measurement_view_from_parts(
        support_report,
        None,
        Some(UiInspectionMeasurementDenialPosture::GenerationIncompatible {
            compatibility: compatibility.clone(),
        }),
        Some(compatibility),
        Some(UiInspectionMeasurementFailureSource::CompatibilityMismatch),
    )
}
