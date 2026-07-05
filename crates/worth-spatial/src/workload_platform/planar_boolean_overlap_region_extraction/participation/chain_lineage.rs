use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapChainRegionLineageRow {
    lineage_row_identity: String,
    lineage_identity: String,
    chain_identity: String,
    fragment_identities: Vec<String>,
    source_loop_identities: Vec<String>,
    source_loop_operand_sides: Vec<PlanarBooleanCommonPlaneOperandSide>,
    source_loop_winding_signs: Vec<i8>,
    source_edge_identities: Vec<String>,
    boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
    participating_loop_identities: Vec<String>,
    participating_island_identities: Vec<String>,
    propagated_persistent_name_identities: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapChainRegionLineageMap {
    map_identity: String,
    request_identity: String,
    rows: Vec<PlanarBooleanOverlapChainRegionLineageRow>,
}

impl PlanarBooleanOverlapChainRegionLineageRow {
    pub(crate) fn new(
        lineage_row_identity: String,
        lineage_identity: String,
        chain_identity: String,
        fragment_identities: Vec<String>,
        source_loop_identities: Vec<String>,
        source_loop_operand_sides: Vec<PlanarBooleanCommonPlaneOperandSide>,
        source_loop_winding_signs: Vec<i8>,
        source_edge_identities: Vec<String>,
        boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
        participating_loop_identities: Vec<String>,
        participating_island_identities: Vec<String>,
        propagated_persistent_name_identities: Vec<String>,
    ) -> Self {
        Self {
            lineage_row_identity,
            lineage_identity,
            chain_identity,
            fragment_identities,
            source_loop_identities,
            source_loop_operand_sides,
            source_loop_winding_signs,
            source_edge_identities,
            boundary_roles,
            participating_loop_identities,
            participating_island_identities,
            propagated_persistent_name_identities,
        }
    }

    pub fn lineage_row_identity(&self) -> &str {
        &self.lineage_row_identity
    }

    pub fn lineage_identity(&self) -> &str {
        &self.lineage_identity
    }

    pub fn chain_identity(&self) -> &str {
        &self.chain_identity
    }

    pub fn fragment_identities(&self) -> &[String] {
        &self.fragment_identities
    }

    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }

    pub fn source_loop_operand_sides(&self) -> &[PlanarBooleanCommonPlaneOperandSide] {
        &self.source_loop_operand_sides
    }

    pub fn source_loop_winding_signs(&self) -> &[i8] {
        &self.source_loop_winding_signs
    }

    pub fn source_edge_identities(&self) -> &[String] {
        &self.source_edge_identities
    }

    pub fn boundary_roles(&self) -> &[PlanarBooleanOverlapChainBoundaryRole] {
        &self.boundary_roles
    }

    pub fn participating_loop_identities(&self) -> &[String] {
        &self.participating_loop_identities
    }

    pub fn participating_island_identities(&self) -> &[String] {
        &self.participating_island_identities
    }

    pub fn propagated_persistent_name_identities(&self) -> &[String] {
        &self.propagated_persistent_name_identities
    }
}

impl PlanarBooleanOverlapChainRegionLineageMap {
    pub(crate) fn new(
        map_identity: String,
        request_identity: String,
        rows: Vec<PlanarBooleanOverlapChainRegionLineageRow>,
    ) -> Self {
        Self {
            map_identity,
            request_identity,
            rows,
        }
    }

    pub fn map_identity(&self) -> &str {
        &self.map_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn rows(&self) -> &[PlanarBooleanOverlapChainRegionLineageRow] {
        &self.rows
    }
}
