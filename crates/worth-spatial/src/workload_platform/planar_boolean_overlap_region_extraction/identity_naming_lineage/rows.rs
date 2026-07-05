use crate::workload_platform::planar_boolean_common_plane::PlanarBooleanCommonPlaneOperandSide;
use crate::workload_platform::planar_boolean_edge_splitting::PlanarBooleanOverlapChainBoundaryRole;
use crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionCanonicalWindingSourceKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionIdentityRow {
    region_identity: String,
    canonical_winding_identity: String,
    source_kind: PlanarBooleanOverlapRegionCanonicalWindingSourceKind,
    source_identity: String,
    island_identity: String,
    neighborhood_identity: String,
    area_overlap_component_identity: Option<String>,
    canonical_operand_side: Option<PlanarBooleanCommonPlaneOperandSide>,
    canonical_winding_sign: Option<i8>,
    canonical_boundary_segment_identities: Vec<String>,
    canonical_source_loop_identities: Vec<String>,
    lineage_identities: Vec<String>,
    source_edge_identities: Vec<String>,
    boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionPersistentNamePropagationRow {
    propagation_identity: String,
    region_identity: String,
    canonical_winding_identity: String,
    persistent_name_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanOverlapRegionSubshapeSignatureRow {
    signature_identity: String,
    region_identity: String,
    canonical_winding_identity: String,
    signature_basis_identity: String,
    correspondence_only: bool,
}

impl PlanarBooleanOverlapRegionIdentityRow {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        region_identity: String,
        canonical_winding_identity: String,
        source_kind: PlanarBooleanOverlapRegionCanonicalWindingSourceKind,
        source_identity: String,
        island_identity: String,
        neighborhood_identity: String,
        area_overlap_component_identity: Option<String>,
        canonical_operand_side: Option<PlanarBooleanCommonPlaneOperandSide>,
        canonical_winding_sign: Option<i8>,
        canonical_boundary_segment_identities: Vec<String>,
        canonical_source_loop_identities: Vec<String>,
        lineage_identities: Vec<String>,
        source_edge_identities: Vec<String>,
        boundary_roles: Vec<PlanarBooleanOverlapChainBoundaryRole>,
    ) -> Self {
        Self {
            region_identity,
            canonical_winding_identity,
            source_kind,
            source_identity,
            island_identity,
            neighborhood_identity,
            area_overlap_component_identity,
            canonical_operand_side,
            canonical_winding_sign,
            canonical_boundary_segment_identities,
            canonical_source_loop_identities,
            lineage_identities,
            source_edge_identities,
            boundary_roles,
        }
    }

    pub fn region_identity(&self) -> &str { &self.region_identity }
    pub fn canonical_winding_identity(&self) -> &str { &self.canonical_winding_identity }
    pub fn source_kind(&self) -> PlanarBooleanOverlapRegionCanonicalWindingSourceKind { self.source_kind }
    pub fn source_identity(&self) -> &str { &self.source_identity }
    pub fn island_identity(&self) -> &str { &self.island_identity }
    pub fn neighborhood_identity(&self) -> &str { &self.neighborhood_identity }
    pub fn area_overlap_component_identity(&self) -> Option<&str> { self.area_overlap_component_identity.as_deref() }
    pub fn canonical_operand_side(&self) -> Option<PlanarBooleanCommonPlaneOperandSide> { self.canonical_operand_side }
    pub fn canonical_winding_sign(&self) -> Option<i8> { self.canonical_winding_sign }
    pub fn canonical_boundary_segment_identities(&self) -> &[String] { &self.canonical_boundary_segment_identities }
    pub fn canonical_source_loop_identities(&self) -> &[String] { &self.canonical_source_loop_identities }
    pub fn lineage_identities(&self) -> &[String] { &self.lineage_identities }
    pub fn source_edge_identities(&self) -> &[String] { &self.source_edge_identities }
    pub fn boundary_roles(&self) -> &[PlanarBooleanOverlapChainBoundaryRole] { &self.boundary_roles }
}

impl PlanarBooleanOverlapRegionPersistentNamePropagationRow {
    pub(crate) fn new(
        propagation_identity: String,
        region_identity: String,
        canonical_winding_identity: String,
        persistent_name_identity: String,
    ) -> Self {
        Self {
            propagation_identity,
            region_identity,
            canonical_winding_identity,
            persistent_name_identity,
        }
    }

    pub fn propagation_identity(&self) -> &str { &self.propagation_identity }
    pub fn region_identity(&self) -> &str { &self.region_identity }
    pub fn canonical_winding_identity(&self) -> &str { &self.canonical_winding_identity }
    pub fn persistent_name_identity(&self) -> &str { &self.persistent_name_identity }
}

impl PlanarBooleanOverlapRegionSubshapeSignatureRow {
    pub(crate) fn new(
        signature_identity: String,
        region_identity: String,
        canonical_winding_identity: String,
        signature_basis_identity: String,
        correspondence_only: bool,
    ) -> Self {
        Self {
            signature_identity,
            region_identity,
            canonical_winding_identity,
            signature_basis_identity,
            correspondence_only,
        }
    }

    pub fn signature_identity(&self) -> &str { &self.signature_identity }
    pub fn region_identity(&self) -> &str { &self.region_identity }
    pub fn canonical_winding_identity(&self) -> &str { &self.canonical_winding_identity }
    pub fn signature_basis_identity(&self) -> &str { &self.signature_basis_identity }
    pub fn correspondence_only(&self) -> bool { self.correspondence_only }
}
