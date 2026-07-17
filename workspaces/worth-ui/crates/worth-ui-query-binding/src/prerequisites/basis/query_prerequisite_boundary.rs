use worth_query::facade::foundation::{ResolvedSnapshotBasis, SnapshotResolutionReport};

use super::prerequisite_assembly::construct_prerequisite_evidence;
use super::{
    WorthUiQueryAuthorityHandle, WorthUiQueryBasisPosture, WorthUiQueryCausalExplanationLane,
    WorthUiQueryInspectionLane, WorthUiQueryMeasurementFactEligibility,
    WorthUiQueryMeasurementFactEligibilityError, WorthUiQueryMeasurementFactObservation,
    WorthUiQueryMeasurementFactObservationError, WorthUiQueryMeasurementFactReceipt,
    WorthUiQueryMeasurementFactReceiptError, WorthUiQueryMeasurementRefinementCounters,
    WorthUiQueryPrerequisiteEvidence, WorthUiQueryPrerequisiteEvidenceError,
    WorthUiQueryProjectionConsumptionLane,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiQueryPrerequisiteBoundary {
    _sealed: (),
}

impl WorthUiQueryPrerequisiteBoundary {
    #[doc(hidden)]
    pub const fn new() -> Self {
        Self { _sealed: () }
    }

    pub fn assemble(
        self,
        basis: ResolvedSnapshotBasis,
        resolution_report: SnapshotResolutionReport,
        basis_posture: WorthUiQueryBasisPosture,
        projection_consumption_lane: WorthUiQueryProjectionConsumptionLane,
        inspection_lane: WorthUiQueryInspectionLane,
        causal_explanation_lane: WorthUiQueryCausalExplanationLane,
    ) -> Result<WorthUiQueryPrerequisiteEvidence, WorthUiQueryPrerequisiteEvidenceError> {
        let _ = self;
        construct_prerequisite_evidence(
            basis,
            resolution_report,
            basis_posture,
            projection_consumption_lane,
            inspection_lane,
            causal_explanation_lane,
        )
    }

    pub fn graph_aligned(
        self,
        basis: ResolvedSnapshotBasis,
        resolution_report: SnapshotResolutionReport,
    ) -> Result<WorthUiQueryPrerequisiteEvidence, WorthUiQueryPrerequisiteEvidenceError> {
        self.assemble(
            basis,
            resolution_report,
            WorthUiQueryBasisPosture::GraphAligned,
            WorthUiQueryProjectionConsumptionLane::ConsumeProjectionFacts,
            WorthUiQueryInspectionLane::WorkspaceInspect,
            WorthUiQueryCausalExplanationLane::AdmitAndRequestCausalInspection,
        )
    }

    pub fn wrong_world_projection(
        self,
        basis: ResolvedSnapshotBasis,
        resolution_report: SnapshotResolutionReport,
    ) -> Result<WorthUiQueryPrerequisiteEvidence, WorthUiQueryPrerequisiteEvidenceError> {
        self.assemble(
            basis,
            resolution_report,
            WorthUiQueryBasisPosture::WrongWorldProjection,
            WorthUiQueryProjectionConsumptionLane::ConsumeProjectionFacts,
            WorthUiQueryInspectionLane::NotRequested,
            WorthUiQueryCausalExplanationLane::NotRequested,
        )
    }

    pub fn rebind_required(
        self,
        basis: ResolvedSnapshotBasis,
        resolution_report: SnapshotResolutionReport,
    ) -> Result<WorthUiQueryPrerequisiteEvidence, WorthUiQueryPrerequisiteEvidenceError> {
        self.assemble(
            basis,
            resolution_report,
            WorthUiQueryBasisPosture::RebindRequired,
            WorthUiQueryProjectionConsumptionLane::ConsumeProjectionFacts,
            WorthUiQueryInspectionLane::NotRequested,
            WorthUiQueryCausalExplanationLane::NotRequested,
        )
    }

    pub fn stale_receipt(
        self,
        basis: ResolvedSnapshotBasis,
        resolution_report: SnapshotResolutionReport,
    ) -> Result<WorthUiQueryPrerequisiteEvidence, WorthUiQueryPrerequisiteEvidenceError> {
        self.assemble(
            basis,
            resolution_report,
            WorthUiQueryBasisPosture::StaleReceipt,
            WorthUiQueryProjectionConsumptionLane::ConsumeProjectionFacts,
            WorthUiQueryInspectionLane::NotRequested,
            WorthUiQueryCausalExplanationLane::NotRequested,
        )
    }

    pub fn ambiguous_sources(
        self,
        basis: ResolvedSnapshotBasis,
        resolution_report: SnapshotResolutionReport,
    ) -> Result<WorthUiQueryPrerequisiteEvidence, WorthUiQueryPrerequisiteEvidenceError> {
        self.assemble(
            basis,
            resolution_report,
            WorthUiQueryBasisPosture::AmbiguousSources,
            WorthUiQueryProjectionConsumptionLane::ConsumeProjectionFacts,
            WorthUiQueryInspectionLane::NotRequested,
            WorthUiQueryCausalExplanationLane::NotRequested,
        )
    }

    pub fn bind_query_authority(
        self,
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        authority: &worth_query::facade::foundation::WorthQueryConsumedProjectionAuthority,
    ) -> Result<WorthUiQueryPrerequisiteEvidence, WorthUiQueryMeasurementFactEligibilityError> {
        let _ = self;
        WorthUiQueryMeasurementFactEligibility::bind_query_authority(prerequisites, authority)
    }

    pub fn measurement_fact_eligibility_from_query_authority(
        self,
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        authority: &worth_query::facade::foundation::WorthQueryConsumedProjectionAuthority,
    ) -> Result<WorthUiQueryMeasurementFactEligibility, WorthUiQueryMeasurementFactEligibilityError>
    {
        let _ = self;
        WorthUiQueryMeasurementFactEligibility::from_query_authority(prerequisites, authority)
    }

    pub fn measurement_fact_receipt_from_query_authority(
        self,
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        query_authority: WorthUiQueryAuthorityHandle,
    ) -> Result<WorthUiQueryMeasurementFactReceipt, WorthUiQueryMeasurementFactReceiptError> {
        let _ = self;
        WorthUiQueryMeasurementFactReceipt::from_query_authority(
            prerequisites,
            query_authority,
            false,
        )
    }

    pub fn measurement_fact_observation_from_query_authority(
        self,
        _prerequisites: WorthUiQueryPrerequisiteEvidence,
        authority: &worth_query::facade::foundation::WorthQueryConsumedProjectionAuthority,
    ) -> Result<
        (
            Box<[WorthUiQueryMeasurementFactObservation]>,
            WorthUiQueryMeasurementRefinementCounters,
        ),
        WorthUiQueryMeasurementFactObservationError,
    > {
        let _ = self;
        WorthUiQueryMeasurementFactObservation::from_query_authority(authority)
    }
}
