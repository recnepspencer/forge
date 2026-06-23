use super::phase_stop::PlanarBooleanEdgeSplitPhaseStop;
use super::query_declaration::PlanarBooleanSplitDecisionLogDeclaration;
use crate::workload_platform::planar_boolean_edge_splitting::{
    PlanarBooleanEdgeSplitRequest, PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    PlanarBooleanSplitChainValidationReceipt, PlanarBooleanSplitEdgeFragmentSet,
    PlanarBooleanSplitPersistentNamingReceipt, PlanarBooleanSplitVertexIdentitySet,
};

pub struct PlanarBooleanSplitDecisionLogInput<'a> {
    declaration: PlanarBooleanSplitDecisionLogDeclaration,
    split_request: Option<&'a PlanarBooleanEdgeSplitRequest>,
    endpoint_boundary_schedules: &'a PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
    interval_subdivision_schedules: &'a PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
    split_vertices: &'a PlanarBooleanSplitVertexIdentitySet,
    split_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
    split_chain_validation: &'a PlanarBooleanSplitChainValidationReceipt,
    split_persistent_names: &'a PlanarBooleanSplitPersistentNamingReceipt,
    phase_stops: Vec<PlanarBooleanEdgeSplitPhaseStop>,
}

impl<'a> PlanarBooleanSplitDecisionLogInput<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn from_certified_products(
        declaration: PlanarBooleanSplitDecisionLogDeclaration,
        split_request: &'a PlanarBooleanEdgeSplitRequest,
        endpoint_boundary_schedules: &'a PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
        interval_subdivision_schedules: &'a PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
        split_vertices: &'a PlanarBooleanSplitVertexIdentitySet,
        split_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
        split_chain_validation: &'a PlanarBooleanSplitChainValidationReceipt,
        split_persistent_names: &'a PlanarBooleanSplitPersistentNamingReceipt,
    ) -> Self {
        Self {
            declaration,
            split_request: Some(split_request),
            endpoint_boundary_schedules,
            interval_subdivision_schedules,
            split_vertices,
            split_fragments,
            split_chain_validation,
            split_persistent_names,
            phase_stops: Vec::new(),
        }
    }

    #[cfg(test)]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_certified_product_identities_for_tests(
        declaration: PlanarBooleanSplitDecisionLogDeclaration,
        endpoint_boundary_schedules: &'a PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet,
        interval_subdivision_schedules: &'a PlanarBooleanIntervalSubdivisionNormalizedScheduleSet,
        split_vertices: &'a PlanarBooleanSplitVertexIdentitySet,
        split_fragments: &'a PlanarBooleanSplitEdgeFragmentSet,
        split_chain_validation: &'a PlanarBooleanSplitChainValidationReceipt,
        split_persistent_names: &'a PlanarBooleanSplitPersistentNamingReceipt,
    ) -> Self {
        Self {
            declaration,
            split_request: None,
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

    pub(crate) fn declaration(&self) -> &PlanarBooleanSplitDecisionLogDeclaration {
        &self.declaration
    }
    pub(crate) fn split_request(&self) -> Option<&PlanarBooleanEdgeSplitRequest> {
        self.split_request
    }
    pub(crate) fn endpoint_boundary_schedules(
        &self,
    ) -> &PlanarBooleanEndpointBoundaryNormalizedSplitScheduleSet {
        self.endpoint_boundary_schedules
    }
    pub(crate) fn interval_subdivision_schedules(
        &self,
    ) -> &PlanarBooleanIntervalSubdivisionNormalizedScheduleSet {
        self.interval_subdivision_schedules
    }
    pub(crate) fn split_vertices(&self) -> &PlanarBooleanSplitVertexIdentitySet {
        self.split_vertices
    }
    pub(crate) fn split_fragments(&self) -> &PlanarBooleanSplitEdgeFragmentSet {
        self.split_fragments
    }
    pub(crate) fn split_chain_validation(&self) -> &PlanarBooleanSplitChainValidationReceipt {
        self.split_chain_validation
    }
    pub(crate) fn split_persistent_names(&self) -> &PlanarBooleanSplitPersistentNamingReceipt {
        self.split_persistent_names
    }
    pub(crate) fn phase_stops(&self) -> &[PlanarBooleanEdgeSplitPhaseStop] {
        &self.phase_stops
    }
}
