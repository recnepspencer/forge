use worth_ui_inspection::{
    UiInspectionMeasurementBasisPosture, UiInspectionMeasurementDenialPosture,
    UiInspectionMeasurementDependencyLineageEntry, UiInspectionMeasurementEvidenceView,
    UiInspectionMeasurementFailureSource, UiInspectionMeasurementGenerationCompatibility,
    UiInspectionSupportReport,
};

use crate::evidence::measurement::basis::{UiMeasurementBasis, UiMeasurementBasisPosture};

use super::project_basis_input::project_basis_input;
use super::project_evidence_maps::{project_lineage_kind, project_neighborhood_class_hint};

pub(crate) fn measurement_view_from_parts(
    support_report: UiInspectionSupportReport,
    basis: Option<&UiMeasurementBasis>,
    denial_posture: Option<UiInspectionMeasurementDenialPosture>,
    generation_compatibility: Option<UiInspectionMeasurementGenerationCompatibility>,
    failure_source: Option<UiInspectionMeasurementFailureSource>,
) -> UiInspectionMeasurementEvidenceView {
    let basis_posture = basis.map(|basis| match basis.basis_posture() {
        UiMeasurementBasisPosture::QueryOnly => UiInspectionMeasurementBasisPosture::QueryOnly,
        UiMeasurementBasisPosture::HostOnly => UiInspectionMeasurementBasisPosture::HostOnly,
        UiMeasurementBasisPosture::QueryAndHost => {
            UiInspectionMeasurementBasisPosture::QueryAndHost
        }
    });
    let basis_inputs = basis
        .map(|basis| {
            basis
                .evidence_inputs()
                .iter()
                .map(project_basis_input)
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
        .unwrap_or_else(|| Box::new([]));
    let dependency_lineage = basis
        .map(|basis| {
            basis
                .dependency_lineage()
                .entries()
                .iter()
                .map(|entry| {
                    UiInspectionMeasurementDependencyLineageEntry::new(
                        project_lineage_kind(entry.kind()),
                        entry.identity_digest(),
                        entry.generation_digest(),
                    )
                })
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
        .unwrap_or_else(|| Box::new([]));
    let neighborhood_class_hint =
        basis.map(|basis| project_neighborhood_class_hint(basis.neighborhood_class_hint()));

    UiInspectionMeasurementEvidenceView::new(
        support_report,
        basis_posture,
        denial_posture,
        basis_inputs,
        dependency_lineage,
        generation_compatibility,
        neighborhood_class_hint,
        failure_source,
    )
}
