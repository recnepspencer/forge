use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOppositeSenseOverlapNormalizationRow {
    normalization_identity: String,
    shared_area_admission_outcome_identity: String,
    island_identity: String,
    neighborhood_identity: String,
    area_overlap_component_identity: String,
    canonical_operand_side: PlanarBooleanCommonPlaneOperandSide,
    canonical_winding_sign: i8,
    chain_identities: Vec<String>,
    fragment_identities: Vec<String>,
    lineage_identities: Vec<String>,
    source_edge_identities: Vec<String>,
    source_loop_identities: Vec<String>,
    boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
    propagated_persistent_name_identities: Vec<String>,
}

impl PlanarBooleanOppositeSenseOverlapNormalizationRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        normalization_identity: String,
        shared_area_admission_outcome_identity: String,
        island_identity: String,
        neighborhood_identity: String,
        area_overlap_component_identity: String,
        canonical_operand_side: PlanarBooleanCommonPlaneOperandSide,
        canonical_winding_sign: i8,
        chain_identities: Vec<String>,
        fragment_identities: Vec<String>,
        lineage_identities: Vec<String>,
        source_edge_identities: Vec<String>,
        source_loop_identities: Vec<String>,
        boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
        propagated_persistent_name_identities: Vec<String>,
    ) -> Self {
        Self {
            normalization_identity,
            shared_area_admission_outcome_identity,
            island_identity,
            neighborhood_identity,
            area_overlap_component_identity,
            canonical_operand_side,
            canonical_winding_sign,
            chain_identities,
            fragment_identities,
            lineage_identities,
            source_edge_identities,
            source_loop_identities,
            boundary_roles,
            propagated_persistent_name_identities,
        }
    }

    pub fn normalization_identity(&self) -> &str {
        &self.normalization_identity
    }

    pub fn shared_area_admission_outcome_identity(&self) -> &str {
        &self.shared_area_admission_outcome_identity
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

    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }

    pub fn boundary_roles(&self) -> &[PlanarBooleanOverlapChainBoundaryRole] {
        &self.boundary_roles
    }

    pub fn propagated_persistent_name_identities(&self) -> &[String] {
        &self.propagated_persistent_name_identities
    }
}
