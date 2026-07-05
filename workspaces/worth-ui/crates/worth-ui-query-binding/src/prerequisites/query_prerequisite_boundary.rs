use forge_query::facade::{ResolvedSnapshotBasis, SnapshotResolutionReport};

use super::{
    WorthUiQueryBasisPosture, WorthUiQueryCausalExplanationLane, WorthUiQueryInspectionLane,
    WorthUiQueryMeasurementFactEligibility, WorthUiQueryMeasurementFactEligibilityError,
    WorthUiQueryMeasurementFactReceipt, WorthUiQueryMeasurementFactReceiptError,
    WorthUiQueryPrerequisiteEvidence, WorthUiQueryPrerequisiteEvidenceError,
    WorthUiQueryProjectionConsumptionLane,
};
#[cfg(feature = "certification-construction")]
use super::WorthUiQueryMeasurementFactFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiQueryPrerequisiteBoundary {
    _sealed: (),
}

impl WorthUiQueryPrerequisiteBoundary {
    pub(crate) const fn new() -> Self {
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
        WorthUiQueryPrerequisiteEvidence::new(
            basis,
            resolution_report,
            basis_posture,
            projection_consumption_lane,
            inspection_lane,
            causal_explanation_lane,
            None,
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
            WorthUiQueryInspectionLane::NotRequested,
            WorthUiQueryCausalExplanationLane::NotRequested,
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

    pub fn bind_projection_consumption(
        self,
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        consumption: &forge_query::facade::ProjectionFactConsumptionAttempt,
    ) -> Result<WorthUiQueryPrerequisiteEvidence, WorthUiQueryMeasurementFactEligibilityError> {
        let _ = self;
        WorthUiQueryMeasurementFactEligibility::bind_projection_consumption_attempt(
            prerequisites,
            consumption,
        )
    }

    pub fn measurement_fact_eligibility_from_projection_consumption(
        self,
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        consumption: &forge_query::facade::ProjectionFactConsumptionAttempt,
    ) -> Result<WorthUiQueryMeasurementFactEligibility, WorthUiQueryMeasurementFactEligibilityError>
    {
        let _ = self;
        WorthUiQueryMeasurementFactEligibility::from_projection_consumption_attempt(
            prerequisites,
            consumption,
        )
    }

    pub fn measurement_fact_receipt_from_projection_consumption(
        self,
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        consumption: &forge_query::facade::ProjectionFactConsumptionAttempt,
    ) -> Result<WorthUiQueryMeasurementFactReceipt, WorthUiQueryMeasurementFactReceiptError> {
        let _ = self;
        WorthUiQueryMeasurementFactReceipt::from_projection_consumption_attempt(
            prerequisites,
            consumption,
        )
    }

    #[cfg(feature = "certification-construction")]
    pub fn bind_projection_contract_for_certification(
        self,
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        projection_contract_digest: impl AsRef<str>,
    ) -> WorthUiQueryPrerequisiteEvidence {
        let _ = self;
        prerequisites.bound_to_projection_contract(projection_contract_digest.as_ref())
    }

    #[cfg(feature = "certification-construction")]
    pub fn measurement_fact_eligibility_for_certification(
        self,
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        projection_contract_digest: impl Into<Box<str>>,
        available_families: Vec<WorthUiQueryMeasurementFactFamily>,
    ) -> WorthUiQueryMeasurementFactEligibility {
        let _ = self;
        WorthUiQueryMeasurementFactEligibility::for_certification(
            prerequisites,
            projection_contract_digest,
            available_families,
        )
    }

    #[cfg(feature = "certification-construction")]
    pub fn measurement_fact_receipt_for_certification(
        self,
        prerequisites: WorthUiQueryPrerequisiteEvidence,
        projection_contract_digest: impl Into<Box<str>>,
        projection_consumption_declaration_digest: impl Into<Box<str>>,
        projection_consumption_receipt_digest: impl Into<Box<str>>,
        projection_fact_set_digest: impl Into<Box<str>>,
        projection_source_identity: impl Into<Box<str>>,
        consumed_families: Vec<WorthUiQueryMeasurementFactFamily>,
    ) -> WorthUiQueryMeasurementFactReceipt {
        let _ = self;
        WorthUiQueryMeasurementFactReceipt::for_certification(
            prerequisites,
            projection_contract_digest,
            projection_consumption_declaration_digest,
            projection_consumption_receipt_digest,
            projection_fact_set_digest,
            projection_source_identity,
            consumed_families,
        )
    }
}
