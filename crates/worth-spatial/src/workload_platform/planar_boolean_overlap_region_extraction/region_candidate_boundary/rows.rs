use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole;

use super::denial::PlanarBooleanDeniedOverlapRegionCandidateKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionCandidateRow {
    candidate_identity: String,
    shared_area_admission_outcome_identity: String,
    normalization_identity: String,
    island_identity: String,
    neighborhood_identity: String,
    area_overlap_component_identity: String,
    cell_identities: Vec<String>,
    boundary_component_identities: Vec<String>,
    boundary_segment_identities: Vec<String>,
    source_loop_identities: Vec<String>,
    canonical_operand_side: PlanarBooleanCommonPlaneOperandSide,
    canonical_winding_sign: i8,
    chain_identities: Vec<String>,
    fragment_identities: Vec<String>,
    lineage_identities: Vec<String>,
    source_edge_identities: Vec<String>,
    boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
    propagated_persistent_name_identities: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanDeniedOverlapRegionCandidateRow {
    denial_identity: String,
    island_identity: String,
    neighborhood_identity: String,
    area_overlap_component_identities: Vec<String>,
    boundary_contact_component_identities: Vec<String>,
    cell_identities: Vec<String>,
    denial_kind: PlanarBooleanDeniedOverlapRegionCandidateKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanAdmittedOverlapRegionRow {
    admitted_region_identity: String,
    candidate_identity: String,
    shared_area_admission_outcome_identity: String,
    normalization_identity: String,
    island_identity: String,
    neighborhood_identity: String,
    area_overlap_component_identity: String,
    cell_identities: Vec<String>,
    boundary_component_identities: Vec<String>,
    boundary_segment_identities: Vec<String>,
    source_loop_identities: Vec<String>,
    canonical_boundary_segment_witness: Vec<String>,
    canonical_source_loop_witness: Vec<String>,
    canonical_operand_side: PlanarBooleanCommonPlaneOperandSide,
    canonical_winding_sign: i8,
    chain_identities: Vec<String>,
    fragment_identities: Vec<String>,
    lineage_identities: Vec<String>,
    source_edge_identities: Vec<String>,
    boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
    propagated_persistent_name_identities: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanBoundaryOnlyOverlapOutcomeRow {
    outcome_identity: String,
    pure_boundary_only_outcome_identity: String,
    island_identity: String,
    neighborhood_identity: String,
    boundary_contact_component_identities: Vec<String>,
    cell_identities: Vec<String>,
    boundary_component_identities: Vec<String>,
    boundary_segment_identities: Vec<String>,
    source_loop_identities: Vec<String>,
    canonical_boundary_segment_witness: Vec<String>,
    canonical_source_loop_witness: Vec<String>,
}
impl PlanarBooleanOverlapRegionCandidateRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        candidate_identity: String,
        shared_area_admission_outcome_identity: String,
        normalization_identity: String,
        island_identity: String,
        neighborhood_identity: String,
        area_overlap_component_identity: String,
        cell_identities: Vec<String>,
        boundary_component_identities: Vec<String>,
        boundary_segment_identities: Vec<String>,
        source_loop_identities: Vec<String>,
        canonical_operand_side: PlanarBooleanCommonPlaneOperandSide,
        canonical_winding_sign: i8,
        chain_identities: Vec<String>,
        fragment_identities: Vec<String>,
        lineage_identities: Vec<String>,
        source_edge_identities: Vec<String>,
        boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
        propagated_persistent_name_identities: Vec<String>,
    ) -> Self {
        Self {
            candidate_identity,
            shared_area_admission_outcome_identity,
            normalization_identity,
            island_identity,
            neighborhood_identity,
            area_overlap_component_identity,
            cell_identities,
            boundary_component_identities,
            boundary_segment_identities,
            source_loop_identities,
            canonical_operand_side,
            canonical_winding_sign,
            chain_identities,
            fragment_identities,
            lineage_identities,
            source_edge_identities,
            boundary_roles,
            propagated_persistent_name_identities,
        }
    }

    pub fn candidate_identity(&self) -> &str {
        &self.candidate_identity
    }
    pub fn shared_area_admission_outcome_identity(&self) -> &str {
        &self.shared_area_admission_outcome_identity
    }
    pub fn normalization_identity(&self) -> &str {
        &self.normalization_identity
    }
    pub fn island_identity(&self) -> &str {
        &self.island_identity
    }
    pub fn neighborhood_identity(&self) -> &str {
        &self.neighborhood_identity
    }
    pub fn area_overlap_component_identity(&self) -> &str {
        &self.area_overlap_component_identity
    }
    pub fn cell_identities(&self) -> &[String] {
        &self.cell_identities
    }
    pub fn boundary_component_identities(&self) -> &[String] {
        &self.boundary_component_identities
    }
    pub fn boundary_segment_identities(&self) -> &[String] {
        &self.boundary_segment_identities
    }
    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }
    pub fn canonical_operand_side(&self) -> PlanarBooleanCommonPlaneOperandSide {
        self.canonical_operand_side
    }
    pub fn canonical_winding_sign(&self) -> i8 {
        self.canonical_winding_sign
    }
    pub fn chain_identities(&self) -> &[String] {
        &self.chain_identities
    }
    pub fn fragment_identities(&self) -> &[String] {
        &self.fragment_identities
    }
    pub fn lineage_identities(&self) -> &[String] {
        &self.lineage_identities
    }
    pub fn source_edge_identities(&self) -> &[String] {
        &self.source_edge_identities
    }
    pub fn boundary_roles(&self) -> &[PlanarBooleanOverlapChainBoundaryRole] {
        &self.boundary_roles
    }
    pub fn propagated_persistent_name_identities(&self) -> &[String] {
        &self.propagated_persistent_name_identities
    }
}

impl PlanarBooleanDeniedOverlapRegionCandidateRow {
    pub(crate) fn new(
        denial_identity: String,
        island_identity: String,
        neighborhood_identity: String,
        area_overlap_component_identities: Vec<String>,
        boundary_contact_component_identities: Vec<String>,
        cell_identities: Vec<String>,
        denial_kind: PlanarBooleanDeniedOverlapRegionCandidateKind,
    ) -> Self {
        Self {
            denial_identity,
            island_identity,
            neighborhood_identity,
            area_overlap_component_identities,
            boundary_contact_component_identities,
            cell_identities,
            denial_kind,
        }
    }

    pub fn denial_identity(&self) -> &str {
        &self.denial_identity
    }
    pub fn island_identity(&self) -> &str {
        &self.island_identity
    }
    pub fn neighborhood_identity(&self) -> &str {
        &self.neighborhood_identity
    }
    pub fn area_overlap_component_identities(&self) -> &[String] {
        &self.area_overlap_component_identities
    }
    pub fn boundary_contact_component_identities(&self) -> &[String] {
        &self.boundary_contact_component_identities
    }
    pub fn cell_identities(&self) -> &[String] {
        &self.cell_identities
    }
    pub fn denial_kind(&self) -> PlanarBooleanDeniedOverlapRegionCandidateKind {
        self.denial_kind
    }
}

impl PlanarBooleanAdmittedOverlapRegionRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        admitted_region_identity: String,
        candidate_identity: String,
        shared_area_admission_outcome_identity: String,
        normalization_identity: String,
        island_identity: String,
        neighborhood_identity: String,
        area_overlap_component_identity: String,
        cell_identities: Vec<String>,
        boundary_component_identities: Vec<String>,
        boundary_segment_identities: Vec<String>,
        source_loop_identities: Vec<String>,
        canonical_boundary_segment_witness: Vec<String>,
        canonical_source_loop_witness: Vec<String>,
        canonical_operand_side: PlanarBooleanCommonPlaneOperandSide,
        canonical_winding_sign: i8,
        chain_identities: Vec<String>,
        fragment_identities: Vec<String>,
        lineage_identities: Vec<String>,
        source_edge_identities: Vec<String>,
        boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
        propagated_persistent_name_identities: Vec<String>,
    ) -> Self {
        Self {
            admitted_region_identity,
            candidate_identity,
            shared_area_admission_outcome_identity,
            normalization_identity,
            island_identity,
            neighborhood_identity,
            area_overlap_component_identity,
            cell_identities,
            boundary_component_identities,
            boundary_segment_identities,
            source_loop_identities,
            canonical_boundary_segment_witness,
            canonical_source_loop_witness,
            canonical_operand_side,
            canonical_winding_sign,
            chain_identities,
            fragment_identities,
            lineage_identities,
            source_edge_identities,
            boundary_roles,
            propagated_persistent_name_identities,
        }
    }

    pub fn admitted_region_identity(&self) -> &str {
        &self.admitted_region_identity
    }
    pub fn candidate_identity(&self) -> &str {
        &self.candidate_identity
    }
    pub fn shared_area_admission_outcome_identity(&self) -> &str {
        &self.shared_area_admission_outcome_identity
    }
    pub fn normalization_identity(&self) -> &str {
        &self.normalization_identity
    }
    pub fn island_identity(&self) -> &str {
        &self.island_identity
    }
    pub fn neighborhood_identity(&self) -> &str {
        &self.neighborhood_identity
    }
    pub fn area_overlap_component_identity(&self) -> &str {
        &self.area_overlap_component_identity
    }
    pub fn cell_identities(&self) -> &[String] {
        &self.cell_identities
    }
    pub fn boundary_component_identities(&self) -> &[String] {
        &self.boundary_component_identities
    }
    pub fn boundary_segment_identities(&self) -> &[String] {
        &self.boundary_segment_identities
    }
    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }
    pub(crate) fn canonical_boundary_segment_witness(&self) -> &[String] {
        &self.canonical_boundary_segment_witness
    }
    pub(crate) fn canonical_source_loop_witness(&self) -> &[String] {
        &self.canonical_source_loop_witness
    }
    pub fn canonical_operand_side(&self) -> PlanarBooleanCommonPlaneOperandSide {
        self.canonical_operand_side
    }
    pub fn canonical_winding_sign(&self) -> i8 {
        self.canonical_winding_sign
    }
    pub fn chain_identities(&self) -> &[String] {
        &self.chain_identities
    }
    pub fn fragment_identities(&self) -> &[String] {
        &self.fragment_identities
    }
    pub fn lineage_identities(&self) -> &[String] {
        &self.lineage_identities
    }
    pub fn source_edge_identities(&self) -> &[String] {
        &self.source_edge_identities
    }
    pub fn boundary_roles(&self) -> &[PlanarBooleanOverlapChainBoundaryRole] {
        &self.boundary_roles
    }
    pub fn propagated_persistent_name_identities(&self) -> &[String] {
        &self.propagated_persistent_name_identities
    }
}

impl PlanarBooleanBoundaryOnlyOverlapOutcomeRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        outcome_identity: String,
        pure_boundary_only_outcome_identity: String,
        island_identity: String,
        neighborhood_identity: String,
        boundary_contact_component_identities: Vec<String>,
        cell_identities: Vec<String>,
        boundary_component_identities: Vec<String>,
        boundary_segment_identities: Vec<String>,
        source_loop_identities: Vec<String>,
        canonical_boundary_segment_witness: Vec<String>,
        canonical_source_loop_witness: Vec<String>,
    ) -> Self {
        Self {
            outcome_identity,
            pure_boundary_only_outcome_identity,
            island_identity,
            neighborhood_identity,
            boundary_contact_component_identities,
            cell_identities,
            boundary_component_identities,
            boundary_segment_identities,
            source_loop_identities,
            canonical_boundary_segment_witness,
            canonical_source_loop_witness,
        }
    }

    pub fn outcome_identity(&self) -> &str {
        &self.outcome_identity
    }
    pub fn pure_boundary_only_outcome_identity(&self) -> &str {
        &self.pure_boundary_only_outcome_identity
    }
    pub fn island_identity(&self) -> &str {
        &self.island_identity
    }
    pub fn neighborhood_identity(&self) -> &str {
        &self.neighborhood_identity
    }
    pub fn boundary_contact_component_identities(&self) -> &[String] {
        &self.boundary_contact_component_identities
    }
    pub fn cell_identities(&self) -> &[String] {
        &self.cell_identities
    }
    pub fn boundary_component_identities(&self) -> &[String] {
        &self.boundary_component_identities
    }
    pub fn boundary_segment_identities(&self) -> &[String] {
        &self.boundary_segment_identities
    }
    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }
    pub(crate) fn canonical_boundary_segment_witness(&self) -> &[String] {
        &self.canonical_boundary_segment_witness
    }
    pub(crate) fn canonical_source_loop_witness(&self) -> &[String] {
        &self.canonical_source_loop_witness
    }
}
