use super::coverage_manifest::{
    PlanarBooleanSplitDecisionCoverageManifest, PlanarBooleanSplitDecisionCoverageReceipt,
};
use super::denial::PlanarBooleanSplitDecisionLogDenial;
use super::input::PlanarBooleanSplitDecisionLogInput;
use super::phase_stop::PlanarBooleanEdgeSplitPhaseStop;
use super::query_declaration::PlanarBooleanSplitDecisionLogDeclaration;
use super::receipt::PlanarBooleanSplitDecisionLogReceipt;
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanEdgeSplitRequest, PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    PlanarBooleanSplitChainValidationReceipt, PlanarBooleanSplitEdgeFragmentSet,
    PlanarBooleanSplitPersistentNamingReceipt, PlanarBooleanSplitVertexIdentitySet,
};

pub struct PlanarBooleanSplitDecisionLogQueryDomain;

pub struct PlanarBooleanSplitDecisionLogQueryInput<'a> {
    split_request: &'a PlanarBooleanEdgeSplitRequest,
    endpoint_boundary_schedules: &'a PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    interval_subdivision_schedules: &'a PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    split_vertices: &'a PlanarBooleanSplitVertexIdentitySet,
    split_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
    split_chain_validation: &'a PlanarBooleanSplitChainValidationReceipt,
    split_persistent_names: &'a PlanarBooleanSplitPersistentNamingReceipt,
    phase_stops: Vec<PlanarBooleanEdgeSplitPhaseStop>,
}

pub struct PlanarBooleanSplitDecisionLogLoweredPlan<'a> {
    declaration: PlanarBooleanSplitDecisionLogDeclaration,
    input: PlanarBooleanSplitDecisionLogQueryInput<'a>,
    coverage_manifest: PlanarBooleanSplitDecisionCoverageManifest,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitDecisionLogQueryResult {
    receipt: PlanarBooleanSplitDecisionLogReceipt,
    coverage: PlanarBooleanSplitDecisionCoverageReceipt,
}

impl<'a> PlanarBooleanSplitDecisionLogQueryInput<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        split_request: &'a PlanarBooleanEdgeSplitRequest,
        endpoint_boundary_schedules: &'a PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
        interval_subdivision_schedules: &'a PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
        split_vertices: &'a PlanarBooleanSplitVertexIdentitySet,
        split_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
        split_chain_validation: &'a PlanarBooleanSplitChainValidationReceipt,
        split_persistent_names: &'a PlanarBooleanSplitPersistentNamingReceipt,
    ) -> Self {
        Self {
            split_request,
            endpoint_boundary_schedules,
            interval_subdivision_schedules,
            split_vertices,
            split_fragments,
            split_chain_validation,
            split_persistent_names,
            phase_stops: Vec::new(),
        }
    }

    pub fn with_phase_stop(mut self, stop: PlanarBooleanEdgeSplitPhaseStop) -> Self {
        self.phase_stops.push(stop);
        self
    }
}

impl PlanarBooleanSplitDecisionLogQueryDomain {
    pub fn declare<'a>(
        input: PlanarBooleanSplitDecisionLogQueryInput<'a>,
    ) -> Result<PlanarBooleanSplitDecisionLogLoweredPlan<'a>, PlanarBooleanSplitDecisionLogDenial>
    {
        let declaration = PlanarBooleanSplitDecisionLogDeclaration::for_split_products(
            input.split_request,
            input.split_chain_validation,
            input.split_persistent_names,
        )?;
        let receipt_input = query_input_to_receipt_input(&declaration, &input);
        let coverage_manifest =
            PlanarBooleanSplitDecisionCoverageManifest::from_input(&receipt_input);
        Ok(PlanarBooleanSplitDecisionLogLoweredPlan {
            declaration,
            input,
            coverage_manifest,
        })
    }
}

impl PlanarBooleanSplitDecisionLogLoweredPlan<'_> {
    pub fn declaration(&self) -> &PlanarBooleanSplitDecisionLogDeclaration {
        &self.declaration
    }
    pub fn coverage_manifest(&self) -> &PlanarBooleanSplitDecisionCoverageManifest {
        &self.coverage_manifest
    }

    pub fn execute(
        self,
    ) -> Result<PlanarBooleanSplitDecisionLogQueryResult, PlanarBooleanSplitDecisionLogDenial> {
        let receipt_input = query_input_to_receipt_input(&self.declaration, &self.input);
        let receipt = PlanarBooleanSplitDecisionLogReceipt::record_decisions(receipt_input)?;
        let coverage = self
            .coverage_manifest
            .validate_rows(receipt.receipt_identity(), receipt.decision_rows())?;
        Ok(PlanarBooleanSplitDecisionLogQueryResult { receipt, coverage })
    }
}

impl PlanarBooleanSplitDecisionLogQueryResult {
    pub fn receipt(&self) -> &PlanarBooleanSplitDecisionLogReceipt {
        &self.receipt
    }
    pub fn into_receipt(self) -> PlanarBooleanSplitDecisionLogReceipt {
        self.receipt
    }
    pub fn coverage(&self) -> &PlanarBooleanSplitDecisionCoverageReceipt {
        &self.coverage
    }
    pub fn certifies_query_owned_decision_log(&self) -> bool {
        self.receipt.certifies_query_native_split_decision_log()
            && self.coverage.expected_rows() == self.coverage.observed_rows()
            && self.coverage.observed_rows() == self.receipt.decision_rows().len()
    }
}

fn query_input_to_receipt_input<'a>(
    declaration: &PlanarBooleanSplitDecisionLogDeclaration,
    input: &PlanarBooleanSplitDecisionLogQueryInput<'a>,
) -> PlanarBooleanSplitDecisionLogInput<'a> {
    let mut receipt_input = PlanarBooleanSplitDecisionLogInput::from_certified_products(
        declaration.clone(),
        input.split_request,
        input.endpoint_boundary_schedules,
        input.interval_subdivision_schedules,
        input.split_vertices,
        input.split_fragments,
        input.split_chain_validation,
        input.split_persistent_names,
    );
    for stop in &input.phase_stops {
        receipt_input = receipt_input.with_phase_stop(stop.clone());
    }
    receipt_input
}
