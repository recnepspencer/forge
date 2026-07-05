use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarBooleanOverlapRegionCanonicalWindingSourceKind {
    AdmittedRegion,
    BoundaryOnlyOutcome,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionCanonicalWindingRow {
    canonical_winding_identity: String,
    source_kind: PlanarBooleanOverlapRegionCanonicalWindingSourceKind,
    source_identity: String,
    island_identity: String,
    neighborhood_identity: String,
    area_overlap_component_identity: Option<String>,
    canonical_operand_side: Option<PlanarBooleanCommonPlaneOperandSide>,
    canonical_winding_sign: Option<i8>,
    boundary_component_identities: Vec<String>,
    canonical_boundary_segment_identities: Vec<String>,
    canonical_source_loop_identities: Vec<String>,
    chain_identities: Vec<String>,
    fragment_identities: Vec<String>,
    lineage_identities: Vec<String>,
    source_edge_identities: Vec<String>,
    boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
    propagated_persistent_name_identities: Vec<String>,
}

impl PlanarBooleanOverlapRegionCanonicalWindingRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        canonical_winding_identity: String,
        source_kind: PlanarBooleanOverlapRegionCanonicalWindingSourceKind,
        source_identity: String,
        island_identity: String,
        neighborhood_identity: String,
        area_overlap_component_identity: Option<String>,
        canonical_operand_side: Option<PlanarBooleanCommonPlaneOperandSide>,
        canonical_winding_sign: Option<i8>,
        boundary_component_identities: Vec<String>,
        canonical_boundary_segment_identities: Vec<String>,
        canonical_source_loop_identities: Vec<String>,
        chain_identities: Vec<String>,
        fragment_identities: Vec<String>,
        lineage_identities: Vec<String>,
        source_edge_identities: Vec<String>,
        boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
        propagated_persistent_name_identities: Vec<String>,
    ) -> Self {
        Self {
            canonical_winding_identity,
            source_kind,
            source_identity,
            island_identity,
            neighborhood_identity,
            area_overlap_component_identity,
            canonical_operand_side,
            canonical_winding_sign,
            boundary_component_identities,
            canonical_boundary_segment_identities,
            canonical_source_loop_identities,
            chain_identities,
            fragment_identities,
            lineage_identities,
            source_edge_identities,
            boundary_roles,
            propagated_persistent_name_identities,
        }
    }

    pub fn canonical_winding_identity(&self) -> &str {
        &self.canonical_winding_identity
    }

    pub fn source_kind(&self) -> PlanarBooleanOverlapRegionCanonicalWindingSourceKind {
        self.source_kind
    }

    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }

    pub fn island_identity(&self) -> &str {
        &self.island_identity
    }

    pub fn neighborhood_identity(&self) -> &str {
        &self.neighborhood_identity
    }

    pub fn area_overlap_component_identity(&self) -> Option<&str> {
        self.area_overlap_component_identity.as_deref()
    }

    pub fn canonical_operand_side(&self) -> Option<PlanarBooleanCommonPlaneOperandSide> {
        self.canonical_operand_side
    }

    pub fn canonical_winding_sign(&self) -> Option<i8> {
        self.canonical_winding_sign
    }

    pub fn boundary_component_identities(&self) -> &[String] {
        &self.boundary_component_identities
    }

    pub fn canonical_boundary_segment_identities(&self) -> &[String] {
        &self.canonical_boundary_segment_identities
    }

    pub fn canonical_source_loop_identities(&self) -> &[String] {
        &self.canonical_source_loop_identities
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
