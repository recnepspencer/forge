//! Evidence types and certification suites — consumption through this lane, not facade root.

mod retained_expansion;

pub(crate) use retained_expansion::{
    expand_retained_allocation_planning_ref, expand_retained_obligation_ref,
};

pub use crate::evidence::{
    admit_measurement_basis, certify_allocation_planning_determinism,
    certify_allocation_planning_suite, certify_measurement_basis_determinism,
    certify_measurement_basis_determinism_for_active_host,
    certify_measurement_basis_determinism_for_scenarios,
    consume_declared_measurement_projection_facts, consume_settled_query_measurement_fact,
    MeasurementEvidenceInput, UiAllocationPlanningCertificationReport,
    UiAllocationPlanningCertificationSuiteKind, UiAllocationPlanningCostClass,
    UiAllocationPlanningCostReceipt, UiAllocationPlanningDeniedBroadeningReason,
    UiAllocationPlanningDeterminismPosture, UiAllocationReceiptDenialInspectionReceipt,
    UiAllocationReceiptInspectionReceipt, UiAllocationSolveConvergencePosture,
    UiAllocationSolvePass, UiAllocationSolveRemainderPolicy, UiAllocationSolveTrace,
    UiAllocationSourceGatewayEvidence, UiAllocationStreamPolicyDenialEvidenceReceipt,
    UiAllocationStreamPolicyEvidenceOutcome, UiAllocationStreamPolicyEvidenceReceipt,
    UiAllocationStreamPolicyPayloadCounters, UiCurrentMeasurementResult, UiEvidenceExpansion,
    UiEvidenceFamilySummary, UiEvidenceHandle, UiEvidenceIdentity, UiEvidenceMaterializedDetail,
    UiEvidenceRef, UiEvidenceSlice, UiEvidenceSliceRef, UiInspectionCostReceipt,
    UiInspectionObligationEvidenceReceipt, UiInspectionObligationReasonProjection,
    UiMeasurementBasis, UiMeasurementBasisCertificationHostRequest,
    UiMeasurementBasisCertificationOutcome, UiMeasurementBasisCertificationReport,
    UiMeasurementBasisCertificationScenario, UiMeasurementBasisCertificationScenarioError,
    UiMeasurementBasisDenial, UiMeasurementBasisDeterminismPosture, UiMeasurementBasisGeneration,
    UiMeasurementBasisPosture, UiMeasurementCoordinateSpace, UiMeasurementDependencyLineage,
    UiMeasurementDependencyLineageEntry, UiMeasurementDependencyLineageKind,
    UiMeasurementEvidenceCategory, UiMeasurementEvidenceSlot, UiMeasurementGenerationCompatibility,
    UiMeasurementNeighborhoodClassHint, UiMeasurementResult, UiMeasurementRoundingPosture,
    UiMeasurementUnitPosture, UiMeasurementValue, UiSettledQueryFactReceipt,
};
pub use crate::runtime::{
    WorthUiFrameReportMaterializationBoundary, WorthUiSteadyFrameReportPlanner,
};
