pub(crate) mod basis;
pub(crate) mod dependency;
mod evidence_category;
mod coordinate_space;
mod generation_compatibility;
#[cfg(test)]
mod host_request_shape_digest;
pub(crate) mod inputs;
pub(crate) mod projection;
mod result;
#[cfg(test)]
mod result_identity_digest;
mod rounding_posture;
mod unit_posture;

pub use basis::{
    admit_measurement_basis, certify_measurement_basis_determinism,
    certify_measurement_basis_determinism_for_scenarios, UiMeasurementBasis,
    UiMeasurementBasisCertificationHostRequest, UiMeasurementBasisCertificationOutcome,
    UiMeasurementBasisCertificationReport, UiMeasurementBasisCertificationScenario,
    UiMeasurementBasisCertificationScenarioError, UiMeasurementBasisDenial,
    UiMeasurementBasisDeterminismPosture, UiMeasurementBasisGeneration, UiMeasurementBasisPosture,
    UiMeasurementEvidenceSlot,
};
pub use dependency::{
    UiMeasurementDependencyLineage, UiMeasurementDependencyLineageEntry,
    UiMeasurementDependencyLineageKind, UiMeasurementDependencyMap,
    UiMeasurementDependencyMapEntry, UiMeasurementNeighborhoodClassHint,
};
pub(crate) use dependency::{
    derive_measurement_dependency_map, derive_measurement_neighborhood_class_hint,
};
pub use evidence_category::UiMeasurementEvidenceCategory;
pub use coordinate_space::UiMeasurementCoordinateSpace;
pub use generation_compatibility::UiMeasurementGenerationCompatibility;
pub use inputs::{
    MeasurementEvidenceInput, UiChildIntrinsicMeasurementEvidence,
    UiChildIntrinsicMeasurementSource, UiMeasurementSiblingResizeSupport,
    UiMeasurementSiblingResizeSupportSource,
};
pub use result::{UiCurrentMeasurementResult, UiMeasurementResult, UiMeasurementValue};
pub use projection::{
    consume_declared_measurement_projection_facts, UiProjectionFactObservation, UiProjectionFactReceipt,
    UiProjectionFactReceiptDenial,
};
pub(crate) use projection::{
    admit_declared_measurement_projection_fact_receipt,
    project_measurement_inspection_compatibility_view, project_measurement_inspection_denial_view,
    project_measurement_inspection_view,
};
pub use rounding_posture::UiMeasurementRoundingPosture;
pub use unit_posture::UiMeasurementUnitPosture;
pub(crate) use crate::evidence::shared::query_measurement_fact_family_digest::query_measurement_fact_family_set_digest;
#[cfg(test)]
pub(crate) use result_identity_digest::measurement_result_identity_digest;
#[cfg(test)]
pub(crate) use host_request_shape_digest::host_measurement_request_shape_digest;