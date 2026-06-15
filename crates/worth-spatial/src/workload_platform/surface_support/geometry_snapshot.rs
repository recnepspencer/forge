use crate::workload_platform::geometry_binding::{
    BoundGeometryWorkload, GeometryCarrierFamily, PlanarLoopBoundaryGeometry,
};
use worth_primitives::{truth_digest_parts, PrimitiveSupportPlaneIdentity, TruthDigestScope};

#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceSupportCarrierRow {
    topology_entity_identity: String,
    geometry_carrier_identity: String,
    carrier_family: GeometryCarrierFamily,
    support_plane_identity_digests: Vec<String>,
    loop_boundary: Option<PlanarLoopBoundaryGeometry>,
}

impl SurfaceSupportCarrierRow {
    fn new(
        topology_entity_identity: impl Into<String>,
        geometry_carrier_identity: impl Into<String>,
        carrier_family: GeometryCarrierFamily,
        support_plane_identity_digests: Vec<String>,
    ) -> Self {
        Self {
            topology_entity_identity: topology_entity_identity.into(),
            geometry_carrier_identity: geometry_carrier_identity.into(),
            carrier_family,
            support_plane_identity_digests,
            loop_boundary: None,
        }
    }

    fn with_loop_boundary(mut self, boundary: PlanarLoopBoundaryGeometry) -> Self {
        self.loop_boundary = Some(boundary);
        self
    }

    pub fn topology_entity_identity(&self) -> &str {
        &self.topology_entity_identity
    }

    pub fn geometry_carrier_identity(&self) -> &str {
        &self.geometry_carrier_identity
    }

    pub fn carrier_family(&self) -> GeometryCarrierFamily {
        self.carrier_family
    }

    pub(crate) fn support_plane_identity_digests(&self) -> &[String] {
        &self.support_plane_identity_digests
    }

    pub fn loop_boundary(&self) -> Option<&PlanarLoopBoundaryGeometry> {
        self.loop_boundary.as_ref()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SurfaceSupportGeometrySnapshot {
    face_rows: Vec<SurfaceSupportCarrierRow>,
    edge_rows: Vec<SurfaceSupportCarrierRow>,
    loop_rows: Vec<SurfaceSupportCarrierRow>,
}

impl SurfaceSupportGeometrySnapshot {
    pub(crate) fn from_bound_geometry(bound_geometry: &BoundGeometryWorkload) -> Self {
        let face_rows = bound_geometry
            .planar_faces()
            .iter()
            .map(|face| {
                SurfaceSupportCarrierRow::new(
                    face.topology_face_identity(),
                    face.carrier_identity().carrier_identity(),
                    face.carrier_identity().family(),
                    support_plane_identity_digests(
                        face.binding_spec().geometry_identity().support_planes(),
                    ),
                )
            })
            .collect();
        let edge_rows = bound_geometry
            .planar_edges()
            .iter()
            .map(|edge| {
                SurfaceSupportCarrierRow::new(
                    edge.topology_edge_identity(),
                    edge.carrier_identity().carrier_identity(),
                    edge.carrier_identity().family(),
                    support_plane_identity_digests(
                        edge.binding_spec().geometry_identity().support_planes(),
                    ),
                )
            })
            .collect();
        let loop_rows = bound_geometry
            .planar_loops()
            .iter()
            .map(|loop_geometry| {
                SurfaceSupportCarrierRow::new(
                    loop_geometry.topology_loop_identity(),
                    loop_geometry.carrier_identity().carrier_identity(),
                    loop_geometry.carrier_identity().family(),
                    support_plane_identity_digests(
                        loop_geometry
                            .binding_spec()
                            .geometry_identity()
                            .support_planes(),
                    ),
                )
                .with_loop_boundary(loop_geometry.boundary().clone())
            })
            .collect();
        Self {
            face_rows,
            edge_rows,
            loop_rows,
        }
    }

    pub fn face_rows(&self) -> &[SurfaceSupportCarrierRow] {
        &self.face_rows
    }

    pub fn edge_rows(&self) -> &[SurfaceSupportCarrierRow] {
        &self.edge_rows
    }

    pub fn loop_rows(&self) -> &[SurfaceSupportCarrierRow] {
        &self.loop_rows
    }
}

fn support_plane_identity_digests(planes: &[PrimitiveSupportPlaneIdentity]) -> Vec<String> {
    planes.iter().map(support_plane_identity_digest).collect()
}

fn support_plane_identity_digest(plane: &PrimitiveSupportPlaneIdentity) -> String {
    truth_digest_parts(
        TruthDigestScope::GeometryIdentity,
        &plane
            .digest_parts()
            .into_iter()
            .enumerate()
            .map(|(index, value)| format!("support-plane:{index}:{value}"))
            .collect::<Vec<_>>(),
    )
}
