use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapArrangementBoundarySegmentRow {
    segment_identity: String,
    neighborhood_identity: String,
    source_loop_identity: String,
    operand_side: PlanarBooleanCommonPlaneOperandSide,
    source_loop_winding_sign: i8,
    source_edge_identity: String,
    fragment_identity: String,
    boundary_role: PlanarBooleanOverlapChainBoundaryRole,
    ordinal: usize,
}

impl PlanarBooleanOverlapArrangementBoundarySegmentRow {
    pub(crate) fn new(
        segment_identity: String,
        neighborhood_identity: String,
        source_loop_identity: String,
        operand_side: PlanarBooleanCommonPlaneOperandSide,
        source_loop_winding_sign: i8,
        source_edge_identity: String,
        fragment_identity: String,
        boundary_role: PlanarBooleanOverlapChainBoundaryRole,
        ordinal: usize,
    ) -> Self {
        Self {
            segment_identity,
            neighborhood_identity,
            source_loop_identity,
            operand_side,
            source_loop_winding_sign,
            source_edge_identity,
            fragment_identity,
            boundary_role,
            ordinal,
        }
    }

    pub fn segment_identity(&self) -> &str {
        &self.segment_identity
    }

    pub fn neighborhood_identity(&self) -> &str {
        &self.neighborhood_identity
    }

    pub fn source_loop_identity(&self) -> &str {
        &self.source_loop_identity
    }

    pub fn operand_side(&self) -> PlanarBooleanCommonPlaneOperandSide {
        self.operand_side
    }

    pub fn source_loop_winding_sign(&self) -> i8 {
        self.source_loop_winding_sign
    }

    pub fn source_edge_identity(&self) -> &str {
        &self.source_edge_identity
    }

    pub fn fragment_identity(&self) -> &str {
        &self.fragment_identity
    }

    pub fn boundary_role(&self) -> PlanarBooleanOverlapChainBoundaryRole {
        self.boundary_role
    }

    pub fn ordinal(&self) -> usize {
        self.ordinal
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapArrangementBoundaryComponentRow {
    boundary_component_identity: String,
    neighborhood_identity: String,
    source_loop_identities: Vec<String>,
    boundary_cycle_identities: Vec<String>,
}

impl PlanarBooleanOverlapArrangementBoundaryComponentRow {
    pub(crate) fn new(
        boundary_component_identity: String,
        neighborhood_identity: String,
        source_loop_identities: Vec<String>,
        boundary_cycle_identities: Vec<String>,
    ) -> Self {
        Self {
            boundary_component_identity,
            neighborhood_identity,
            source_loop_identities,
            boundary_cycle_identities,
        }
    }

    pub fn boundary_component_identity(&self) -> &str {
        &self.boundary_component_identity
    }

    pub fn neighborhood_identity(&self) -> &str {
        &self.neighborhood_identity
    }

    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }

    pub fn boundary_cycle_identities(&self) -> &[String] {
        &self.boundary_cycle_identities
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanCoplanarOverlapArrangementNeighborhoodRow {
    arrangement_identity: String,
    neighborhood_identity: String,
    chain_identities: Vec<String>,
    lineage_identities: Vec<String>,
    participating_loop_identities: Vec<String>,
    participating_island_identities: Vec<String>,
    boundary_component_identities: Vec<String>,
    source_loop_identities: Vec<String>,
    source_edge_identities: Vec<String>,
    fragment_identities: Vec<String>,
    boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
    propagated_persistent_name_identities: Vec<String>,
    cell_identities: Vec<String>,
}

impl PlanarBooleanCoplanarOverlapArrangementNeighborhoodRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        arrangement_identity: String,
        neighborhood_identity: String,
        chain_identities: Vec<String>,
        lineage_identities: Vec<String>,
        participating_loop_identities: Vec<String>,
        participating_island_identities: Vec<String>,
        boundary_component_identities: Vec<String>,
        source_loop_identities: Vec<String>,
        source_edge_identities: Vec<String>,
        fragment_identities: Vec<String>,
        boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
        propagated_persistent_name_identities: Vec<String>,
        cell_identities: Vec<String>,
    ) -> Self {
        Self {
            arrangement_identity,
            neighborhood_identity,
            chain_identities,
            lineage_identities,
            participating_loop_identities,
            participating_island_identities,
            boundary_component_identities,
            source_loop_identities,
            source_edge_identities,
            fragment_identities,
            boundary_roles,
            propagated_persistent_name_identities,
            cell_identities,
        }
    }

    pub fn arrangement_identity(&self) -> &str {
        &self.arrangement_identity
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

    pub fn participating_loop_identities(&self) -> &[String] {
        &self.participating_loop_identities
    }

    pub fn participating_island_identities(&self) -> &[String] {
        &self.participating_island_identities
    }

    pub fn boundary_component_identities(&self) -> &[String] {
        &self.boundary_component_identities
    }

    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
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

    pub fn cell_identities(&self) -> &[String] {
        &self.cell_identities
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapArrangementCellRow {
    cell_identity: String,
    arrangement_identity: String,
    neighborhood_identity: String,
    source_loop_identities: Vec<String>,
    supporting_island_identity: Option<String>,
    supporting_island_member_source_loop_identities: Vec<String>,
    supporting_island_member_source_loop_operand_sides: Vec<PlanarBooleanCommonPlaneOperandSide>,
    supporting_island_member_source_loop_winding_signs: Vec<i8>,
    chain_identities: Vec<String>,
    lineage_identities: Vec<String>,
    participating_loop_identities: Vec<String>,
    participating_island_identities: Vec<String>,
    boundary_component_identities: Vec<String>,
    boundary_segment_identities: Vec<String>,
    propagated_persistent_name_identities: Vec<String>,
}

impl PlanarBooleanOverlapArrangementCellRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        cell_identity: String,
        arrangement_identity: String,
        neighborhood_identity: String,
        source_loop_identities: Vec<String>,
        supporting_island_identity: Option<String>,
        supporting_island_member_source_loop_identities: Vec<String>,
        supporting_island_member_source_loop_operand_sides: Vec<
            PlanarBooleanCommonPlaneOperandSide,
        >,
        supporting_island_member_source_loop_winding_signs: Vec<i8>,
        chain_identities: Vec<String>,
        lineage_identities: Vec<String>,
        participating_loop_identities: Vec<String>,
        participating_island_identities: Vec<String>,
        boundary_component_identities: Vec<String>,
        boundary_segment_identities: Vec<String>,
        propagated_persistent_name_identities: Vec<String>,
    ) -> Self {
        Self {
            cell_identity,
            arrangement_identity,
            neighborhood_identity,
            source_loop_identities,
            supporting_island_identity,
            supporting_island_member_source_loop_identities,
            supporting_island_member_source_loop_operand_sides,
            supporting_island_member_source_loop_winding_signs,
            chain_identities,
            lineage_identities,
            participating_loop_identities,
            participating_island_identities,
            boundary_component_identities,
            boundary_segment_identities,
            propagated_persistent_name_identities,
        }
    }

    pub fn cell_identity(&self) -> &str {
        &self.cell_identity
    }

    pub fn arrangement_identity(&self) -> &str {
        &self.arrangement_identity
    }

    pub fn neighborhood_identity(&self) -> &str {
        &self.neighborhood_identity
    }

    pub fn source_loop_identities(&self) -> &[String] {
        &self.source_loop_identities
    }

    pub fn supporting_island_identity(&self) -> Option<&str> {
        self.supporting_island_identity.as_deref()
    }

    pub fn supporting_island_member_source_loop_identities(&self) -> &[String] {
        &self.supporting_island_member_source_loop_identities
    }

    pub fn supporting_island_member_source_loop_operand_sides(
        &self,
    ) -> &[PlanarBooleanCommonPlaneOperandSide] {
        &self.supporting_island_member_source_loop_operand_sides
    }

    pub fn supporting_island_member_source_loop_winding_signs(&self) -> &[i8] {
        &self.supporting_island_member_source_loop_winding_signs
    }

    pub fn chain_identities(&self) -> &[String] {
        &self.chain_identities
    }

    pub fn lineage_identities(&self) -> &[String] {
        &self.lineage_identities
    }

    pub fn participating_loop_identities(&self) -> &[String] {
        &self.participating_loop_identities
    }

    pub fn participating_island_identities(&self) -> &[String] {
        &self.participating_island_identities
    }

    pub fn boundary_component_identities(&self) -> &[String] {
        &self.boundary_component_identities
    }

    pub fn boundary_segment_identities(&self) -> &[String] {
        &self.boundary_segment_identities
    }

    pub fn propagated_persistent_name_identities(&self) -> &[String] {
        &self.propagated_persistent_name_identities
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapArrangementCellSet {
    cell_set_identity: String,
    request_identity: String,
    adjacency_index_identity: String,
    ordering_basis_identity: String,
    cells: Vec<PlanarBooleanOverlapArrangementCellRow>,
}

impl PlanarBooleanOverlapArrangementCellSet {
    pub(crate) fn new(
        cell_set_identity: String,
        request_identity: String,
        adjacency_index_identity: String,
        ordering_basis_identity: String,
        cells: Vec<PlanarBooleanOverlapArrangementCellRow>,
    ) -> Self {
        Self {
            cell_set_identity,
            request_identity,
            adjacency_index_identity,
            ordering_basis_identity,
            cells,
        }
    }

    pub fn cell_set_identity(&self) -> &str {
        &self.cell_set_identity
    }

    pub fn request_identity(&self) -> &str {
        &self.request_identity
    }

    pub fn adjacency_index_identity(&self) -> &str {
        &self.adjacency_index_identity
    }

    pub fn ordering_basis_identity(&self) -> &str {
        &self.ordering_basis_identity
    }

    pub fn cells(&self) -> &[PlanarBooleanOverlapArrangementCellRow] {
        &self.cells
    }
}
