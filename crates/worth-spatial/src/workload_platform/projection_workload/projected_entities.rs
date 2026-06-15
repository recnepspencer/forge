use crate::workload_platform::{
    geometry_binding::PlanarLoopBoundaryGeometry,
    surface_support::{CertifiedSurfaceSupport, SurfaceSupportCarrierRow},
};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct ProjectedTopologyEntities {
    faces: Vec<ProjectedFace>,
    edges: Vec<ProjectedEdge>,
    loops: Vec<ProjectedLoop>,
}

impl ProjectedTopologyEntities {
    pub(crate) fn from_certified_surface_support(
        surface_support: &CertifiedSurfaceSupport,
        surface_support_identity: &str,
        local_basis_identity: &str,
    ) -> Self {
        let faces = surface_support
            .geometry_snapshot()
            .face_rows()
            .iter()
            .map(|row| {
                ProjectedFace::from_carrier(row, surface_support_identity, local_basis_identity)
            })
            .collect();
        let edges = surface_support
            .geometry_snapshot()
            .edge_rows()
            .iter()
            .map(|row| {
                ProjectedEdge::from_carrier(row, surface_support_identity, local_basis_identity)
            })
            .collect();
        let loops = surface_support
            .geometry_snapshot()
            .loop_rows()
            .iter()
            .map(|row| {
                ProjectedLoop::from_carrier(row, surface_support_identity, local_basis_identity)
            })
            .collect();
        Self {
            faces,
            edges,
            loops,
        }
    }

    pub(crate) fn face_count(&self) -> usize {
        self.faces.len()
    }

    pub(crate) fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub(crate) fn loop_count(&self) -> usize {
        self.loops.len()
    }

    pub(crate) fn into_parts(self) -> (Vec<ProjectedFace>, Vec<ProjectedEdge>, Vec<ProjectedLoop>) {
        (self.faces, self.edges, self.loops)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectedEntityIdentity {
    topology_entity_identity: String,
    geometry_carrier_identity: String,
    surface_support_identity: String,
    local_basis_identity: String,
    projected_fact_identity: String,
}

impl ProjectedEntityIdentity {
    fn from_carrier(
        carrier: &SurfaceSupportCarrierRow,
        surface_support_identity: &str,
        local_basis_identity: &str,
    ) -> Self {
        let projected_fact_identity = format!(
            "projected-entity:{}:{}:{}:{}",
            carrier.topology_entity_identity(),
            carrier.geometry_carrier_identity(),
            surface_support_identity,
            local_basis_identity
        );
        Self {
            topology_entity_identity: carrier.topology_entity_identity().to_string(),
            geometry_carrier_identity: carrier.geometry_carrier_identity().to_string(),
            surface_support_identity: surface_support_identity.to_string(),
            local_basis_identity: local_basis_identity.to_string(),
            projected_fact_identity,
        }
    }

    pub fn topology_entity_identity(&self) -> &str {
        &self.topology_entity_identity
    }

    pub fn geometry_carrier_identity(&self) -> &str {
        &self.geometry_carrier_identity
    }

    pub fn surface_support_identity(&self) -> &str {
        &self.surface_support_identity
    }

    pub fn local_basis_identity(&self) -> &str {
        &self.local_basis_identity
    }

    pub fn projected_fact_identity(&self) -> &str {
        &self.projected_fact_identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectedFace {
    identity: ProjectedEntityIdentity,
}

impl ProjectedFace {
    pub(crate) fn from_carrier(
        carrier: &SurfaceSupportCarrierRow,
        surface_support_identity: &str,
        local_basis_identity: &str,
    ) -> Self {
        Self {
            identity: ProjectedEntityIdentity::from_carrier(
                carrier,
                surface_support_identity,
                local_basis_identity,
            ),
        }
    }

    pub fn identity(&self) -> &ProjectedEntityIdentity {
        &self.identity
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectedLoop {
    identity: ProjectedEntityIdentity,
    boundary: Option<PlanarLoopBoundaryGeometry>,
}

impl ProjectedLoop {
    pub(crate) fn from_carrier(
        carrier: &SurfaceSupportCarrierRow,
        surface_support_identity: &str,
        local_basis_identity: &str,
    ) -> Self {
        Self {
            identity: ProjectedEntityIdentity::from_carrier(
                carrier,
                surface_support_identity,
                local_basis_identity,
            ),
            boundary: carrier.loop_boundary().cloned(),
        }
    }

    pub fn identity(&self) -> &ProjectedEntityIdentity {
        &self.identity
    }

    pub fn boundary(&self) -> Option<&PlanarLoopBoundaryGeometry> {
        self.boundary.as_ref()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectedEdge {
    identity: ProjectedEntityIdentity,
}

impl ProjectedEdge {
    pub(crate) fn from_carrier(
        carrier: &SurfaceSupportCarrierRow,
        surface_support_identity: &str,
        local_basis_identity: &str,
    ) -> Self {
        Self {
            identity: ProjectedEntityIdentity::from_carrier(
                carrier,
                surface_support_identity,
                local_basis_identity,
            ),
        }
    }

    pub fn identity(&self) -> &ProjectedEntityIdentity {
        &self.identity
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectedEdgeSet {
    edges: Vec<ProjectedEdge>,
}

impl ProjectedEdgeSet {
    pub(crate) fn new(edges: Vec<ProjectedEdge>) -> Self {
        Self { edges }
    }

    pub fn edges(&self) -> &[ProjectedEdge] {
        &self.edges
    }
}
