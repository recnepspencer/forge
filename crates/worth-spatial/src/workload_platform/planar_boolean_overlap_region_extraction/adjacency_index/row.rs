use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole;
use crate::workload_platform::planar_boolean_events::PlanarBooleanLoopRole;
use crate::workload_platform::planar_boolean_loop_reconstruction::PlanarBooleanLoopIslandKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapAdjacencyRow {
    adjacency_identity: String,
    neighborhood_identity: String,
    chain_identities: Vec<String>,
    lineage_identities: Vec<String>,
    loop_participation_identities: Vec<String>,
    participating_loop_identities: Vec<String>,
    loop_roles: Vec<PlanarBooleanLoopRole>,
    island_participation_identities: Vec<String>,
    participating_island_identities: Vec<String>,
    island_origin_loop_identities: Vec<String>,
    island_kinds: Vec<PlanarBooleanLoopIslandKind>,
    island_member_source_loop_identities: Vec<Vec<String>>,
    island_member_source_loop_operand_sides: Vec<Vec<PlanarBooleanCommonPlaneOperandSide>>,
    island_member_source_loop_winding_signs: Vec<Vec<i8>>,
    source_loop_identities: Vec<String>,
    source_loop_operand_sides: Vec<PlanarBooleanCommonPlaneOperandSide>,
    source_loop_winding_signs: Vec<i8>,
    source_edge_identities: Vec<String>,
    fragment_identities: Vec<String>,
    boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
    propagated_persistent_name_identities: Vec<String>,
}

impl PlanarBooleanOverlapAdjacencyRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        adjacency_identity: String,
        neighborhood_identity: String,
        chain_identities: Vec<String>,
        lineage_identities: Vec<String>,
        loop_participation_identities: Vec<String>,
        participating_loop_identities: Vec<String>,
        loop_roles: Vec<PlanarBooleanLoopRole>,
        island_participation_identities: Vec<String>,
        participating_island_identities: Vec<String>,
        island_origin_loop_identities: Vec<String>,
        island_kinds: Vec<PlanarBooleanLoopIslandKind>,
        island_member_source_loop_identities: Vec<Vec<String>>,
        island_member_source_loop_operand_sides: Vec<Vec<PlanarBooleanCommonPlaneOperandSide>>,
        island_member_source_loop_winding_signs: Vec<Vec<i8>>,
        source_loop_identities: Vec<String>,
        source_loop_operand_sides: Vec<PlanarBooleanCommonPlaneOperandSide>,
        source_loop_winding_signs: Vec<i8>,
        source_edge_identities: Vec<String>,
        fragment_identities: Vec<String>,
        boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
        propagated_persistent_name_identities: Vec<String>,
    ) -> Self {
        Self {
            adjacency_identity,
            neighborhood_identity,
            chain_identities,
            lineage_identities,
            loop_participation_identities,
            participating_loop_identities,
            loop_roles,
            island_participation_identities,
            participating_island_identities,
            island_origin_loop_identities,
            island_kinds,
            island_member_source_loop_identities,
            island_member_source_loop_operand_sides,
            island_member_source_loop_winding_signs,
            source_loop_identities,
            source_loop_operand_sides,
            source_loop_winding_signs,
            source_edge_identities,
            fragment_identities,
            boundary_roles,
            propagated_persistent_name_identities,
        }
    }

    pub fn adjacency_identity(&self) -> &str {
        &self.adjacency_identity
    }

    pub fn neighborhood_identity(&self) -> &str {
        &self.neighborhood_identity
    }

    pub fn chain_identities(&self) -> &[String] {
        &self.chain_identities
    }

    pub fn lineage_identities(&self) -> &[String] {
        &self.lineage_identities
    }

    pub fn loop_participation_identities(&self) -> &[String] {
        &self.loop_participation_identities
    }

    pub fn participating_loop_identities(&self) -> &[String] {
        &self.participating_loop_identities
    }

    pub fn loop_roles(&self) -> &[PlanarBooleanLoopRole] {
        &self.loop_roles
    }

    pub fn island_participation_identities(&self) -> &[String] {
        &self.island_participation_identities
    }

    pub fn participating_island_identities(&self) -> &[String] {
        &self.participating_island_identities
    }

    pub fn island_origin_loop_identities(&self) -> &[String] {
        &self.island_origin_loop_identities
    }

    pub fn island_kinds(&self) -> &[PlanarBooleanLoopIslandKind] {
        &self.island_kinds
    }

    pub fn island_member_source_loop_identities(&self) -> &[Vec<String>] {
        &self.island_member_source_loop_identities
    }

    pub fn island_member_source_loop_operand_sides(
        &self,
    ) -> &[Vec<PlanarBooleanCommonPlaneOperandSide>] {
        &self.island_member_source_loop_operand_sides
    }

    pub fn island_member_source_loop_winding_signs(&self) -> &[Vec<i8>] {
        &self.island_member_source_loop_winding_signs
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

    pub fn fragment_identities(&self) -> &[String] {
        &self.fragment_identities
    }

    pub fn boundary_roles(&self) -> &[PlanarBooleanOverlapChainBoundaryRole] {
        &self.boundary_roles
    }

    pub fn propagated_persistent_name_identities(&self) -> &[String] {
        &self.propagated_persistent_name_identities
    }
}
