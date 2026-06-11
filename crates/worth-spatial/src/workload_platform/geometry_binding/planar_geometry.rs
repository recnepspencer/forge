use super::{GeometryCarrierFamily, GeometryCarrierIdentity};
use crate::bindings::authority::{
    CoedgeBindingSite, CoedgePCurveBindingSpec, EdgeBindingSite, EdgeCurveBindingSpec,
    FaceBindingSite, FaceSurfaceBindingSpec, SpatialBindingCompleteness,
};
use crate::bindings::identity::{
    coedge_pcurve_basis, edge_curve_basis, face_surface_basis, SpatialBindingIdentity,
};
use topology::facade::{TopologySeedKind, TopologySeedReceipt};
use worth_primitives::{
    PrimitiveConstructionBirthSynopsisContract, PrimitiveConstructionFamilyContractRegistry,
    PrimitiveGeometryIdentityBundle, PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity,
    PrimitiveWitnessDescriptor,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundPlanarFaceGeometry {
    topology_face_identity: String,
    carrier_identity: GeometryCarrierIdentity,
    binding_spec: FaceSurfaceBindingSpec,
    completeness: SpatialBindingCompleteness,
}

impl BoundPlanarFaceGeometry {
    fn new(
        topology_face_identity: String,
        binding_spec: FaceSurfaceBindingSpec,
        binding_identity: SpatialBindingIdentity,
    ) -> Self {
        Self {
            carrier_identity: GeometryCarrierIdentity::from_spatial_binding(
                GeometryCarrierFamily::PlanarFace,
                topology_face_identity.clone(),
                &binding_identity,
            ),
            topology_face_identity,
            completeness: crate::bindings::authority::evaluate_face_surface_completeness(
                binding_spec.geometry_identity(),
            ),
            binding_spec,
        }
    }

    pub fn topology_face_identity(&self) -> &str {
        &self.topology_face_identity
    }

    pub fn carrier_identity(&self) -> &GeometryCarrierIdentity {
        &self.carrier_identity
    }

    pub fn completeness(&self) -> SpatialBindingCompleteness {
        self.completeness
    }

    pub fn binding_spec(&self) -> &FaceSurfaceBindingSpec {
        &self.binding_spec
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundPlanarEdgeGeometry {
    topology_edge_identity: String,
    carrier_identity: GeometryCarrierIdentity,
    binding_spec: EdgeCurveBindingSpec,
    completeness: SpatialBindingCompleteness,
}

impl BoundPlanarEdgeGeometry {
    fn new(
        topology_edge_identity: String,
        binding_spec: EdgeCurveBindingSpec,
        binding_identity: SpatialBindingIdentity,
    ) -> Self {
        Self {
            carrier_identity: GeometryCarrierIdentity::from_spatial_binding(
                GeometryCarrierFamily::PlanarEdge,
                topology_edge_identity.clone(),
                &binding_identity,
            ),
            topology_edge_identity,
            completeness: crate::bindings::authority::evaluate_edge_curve_completeness(
                binding_spec.geometry_identity(),
            ),
            binding_spec,
        }
    }

    pub fn topology_edge_identity(&self) -> &str {
        &self.topology_edge_identity
    }

    pub fn carrier_identity(&self) -> &GeometryCarrierIdentity {
        &self.carrier_identity
    }

    pub fn completeness(&self) -> SpatialBindingCompleteness {
        self.completeness
    }

    pub fn binding_spec(&self) -> &EdgeCurveBindingSpec {
        &self.binding_spec
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundPlanarLoopGeometry {
    topology_loop_identity: String,
    carrier_identity: GeometryCarrierIdentity,
    binding_spec: CoedgePCurveBindingSpec,
    completeness: SpatialBindingCompleteness,
}

impl BoundPlanarLoopGeometry {
    fn new(
        topology_loop_identity: String,
        binding_spec: CoedgePCurveBindingSpec,
        binding_identity: SpatialBindingIdentity,
    ) -> Self {
        Self {
            carrier_identity: GeometryCarrierIdentity::from_spatial_binding(
                GeometryCarrierFamily::PlanarLoop,
                topology_loop_identity.clone(),
                &binding_identity,
            ),
            topology_loop_identity,
            completeness: crate::bindings::authority::evaluate_coedge_pcurve_completeness(
                binding_spec.geometry_identity(),
            ),
            binding_spec,
        }
    }

    pub fn topology_loop_identity(&self) -> &str {
        &self.topology_loop_identity
    }

    pub fn carrier_identity(&self) -> &GeometryCarrierIdentity {
        &self.carrier_identity
    }

    pub fn completeness(&self) -> SpatialBindingCompleteness {
        self.completeness
    }

    pub fn binding_spec(&self) -> &CoedgePCurveBindingSpec {
        &self.binding_spec
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarFaceCarrierSet {
    topology_receipt_identity: String,
    faces: Vec<BoundPlanarFaceGeometry>,
}

impl PlanarFaceCarrierSet {
    pub fn for_seed_faces(seed: &TopologySeedReceipt) -> Self {
        let contract = birth_contract_for_seed(seed);
        let topology_receipt_identity = topology_receipt_identity(seed);
        let faces = seed
            .entity_identities()
            .face_identity_tokens()
            .into_iter()
            .enumerate()
            .map(|(index, topology_identity)| {
                let geometry = planar_geometry_identity(index);
                let spec = FaceSurfaceBindingSpec::new(
                    FaceBindingSite::new(topology_identity.clone()),
                    contract,
                    geometry,
                );
                let binding_identity = SpatialBindingIdentity::from_basis(face_surface_basis(
                    spec.site().topology_face_identity(),
                    spec.birth_contract(),
                    spec.geometry_identity(),
                ));
                BoundPlanarFaceGeometry::new(topology_identity, spec, binding_identity)
            })
            .collect();
        Self {
            topology_receipt_identity,
            faces,
        }
    }

    pub fn topology_receipt_identity(&self) -> &str {
        &self.topology_receipt_identity
    }

    pub fn faces(&self) -> &[BoundPlanarFaceGeometry] {
        &self.faces
    }

    pub(crate) fn into_faces(self) -> Vec<BoundPlanarFaceGeometry> {
        self.faces
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarEdgeCarrierSet {
    topology_receipt_identity: String,
    edges: Vec<BoundPlanarEdgeGeometry>,
}

impl PlanarEdgeCarrierSet {
    pub fn for_seed_edges(seed: &TopologySeedReceipt) -> Self {
        let contract = birth_contract_for_seed(seed);
        let topology_receipt_identity = topology_receipt_identity(seed);
        let edges = seed
            .entity_identities()
            .edge_identity_tokens()
            .into_iter()
            .enumerate()
            .map(|(index, topology_identity)| {
                let geometry = curve_geometry_identity(index);
                let spec = EdgeCurveBindingSpec::new(
                    EdgeBindingSite::new(topology_identity.clone()),
                    contract,
                    geometry,
                );
                let binding_identity = SpatialBindingIdentity::from_basis(edge_curve_basis(
                    spec.site().topology_edge_identity(),
                    spec.birth_contract(),
                    spec.geometry_identity(),
                ));
                BoundPlanarEdgeGeometry::new(topology_identity, spec, binding_identity)
            })
            .collect();
        Self {
            topology_receipt_identity,
            edges,
        }
    }

    pub fn topology_receipt_identity(&self) -> &str {
        &self.topology_receipt_identity
    }

    pub fn edges(&self) -> &[BoundPlanarEdgeGeometry] {
        &self.edges
    }

    pub(crate) fn into_edges(self) -> Vec<BoundPlanarEdgeGeometry> {
        self.edges
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarLoopCarrierSet {
    topology_receipt_identity: String,
    loops: Vec<BoundPlanarLoopGeometry>,
}

impl PlanarLoopCarrierSet {
    pub fn for_seed_loops(seed: &TopologySeedReceipt) -> Self {
        let contract = birth_contract_for_seed(seed);
        let topology_receipt_identity = topology_receipt_identity(seed);
        let loops = seed
            .entity_identities()
            .loop_identity_tokens()
            .into_iter()
            .enumerate()
            .map(|(index, topology_identity)| {
                let geometry = planar_geometry_identity(index);
                let spec = CoedgePCurveBindingSpec::new(
                    CoedgeBindingSite::new(topology_identity.clone()),
                    contract,
                    geometry,
                );
                let binding_identity = SpatialBindingIdentity::from_basis(coedge_pcurve_basis(
                    spec.site().topology_coedge_identity(),
                    spec.birth_contract(),
                    spec.geometry_identity(),
                ));
                BoundPlanarLoopGeometry::new(topology_identity, spec, binding_identity)
            })
            .collect();
        Self {
            topology_receipt_identity,
            loops,
        }
    }

    pub fn topology_receipt_identity(&self) -> &str {
        &self.topology_receipt_identity
    }

    pub fn loops(&self) -> &[BoundPlanarLoopGeometry] {
        &self.loops
    }

    pub(crate) fn into_loops(self) -> Vec<BoundPlanarLoopGeometry> {
        self.loops
    }
}

fn topology_receipt_identity(seed: &TopologySeedReceipt) -> String {
    seed.query_receipts()
        .declaration_receipt()
        .identity()
        .name()
        .to_string()
}

fn birth_contract_for_seed(
    seed: &TopologySeedReceipt,
) -> PrimitiveConstructionBirthSynopsisContract {
    let descriptor = match seed.kind() {
        TopologySeedKind::Cube => PrimitiveWitnessDescriptor::Orthotope,
        TopologySeedKind::Tetrahedron => PrimitiveWitnessDescriptor::SimplexSolid,
        TopologySeedKind::SingleFaceLoop => PrimitiveWitnessDescriptor::ShellWithHole {
            outer_loop_edge_count: seed.counters().edge_count().max(3) as u32,
            hole_loop_edge_counts: vec![],
        },
        TopologySeedKind::OpenWire | TopologySeedKind::NonManifoldWire => {
            PrimitiveWitnessDescriptor::WireBody {
                edge_count: seed.counters().edge_count().max(1) as u32,
            }
        }
        _ => PrimitiveWitnessDescriptor::RegularPrism {
            side_count: seed.counters().face_count().saturating_sub(2).max(3) as u32,
        },
    };
    PrimitiveConstructionFamilyContractRegistry::contract_for(&descriptor)
}

fn planar_geometry_identity(index: usize) -> PrimitiveGeometryIdentityBundle {
    PrimitiveGeometryIdentityBundle::new(vec![support_plane(index)], vertices(index))
}

fn curve_geometry_identity(index: usize) -> PrimitiveGeometryIdentityBundle {
    PrimitiveGeometryIdentityBundle::new(vec![], vertices(index))
}

fn support_plane(index: usize) -> PrimitiveSupportPlaneIdentity {
    PrimitiveSupportPlaneIdentity::new(
        "0".to_string(),
        "0".to_string(),
        "1".to_string(),
        format!("-{index}"),
    )
}

fn vertices(index: usize) -> Vec<PrimitiveVertexIdentity> {
    let base = index as f64;
    vec![
        PrimitiveVertexIdentity::from_position([base, 0.0, 0.0]),
        PrimitiveVertexIdentity::from_position([base, 1.0, 0.0]),
    ]
}
