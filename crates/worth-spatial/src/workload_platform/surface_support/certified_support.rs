use super::{SurfaceFamily, SurfaceSupportGeometrySnapshot, SurfaceSupportReceiptSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertifiedPlaneSupport {
    family: SurfaceFamily,
    upstream_geometry_binding_identity: String,
    topology_query_surface: String,
}

impl CertifiedPlaneSupport {
    pub(crate) fn new(
        upstream_geometry_binding_identity: impl Into<String>,
        topology_query_surface: impl Into<String>,
    ) -> Self {
        Self {
            family: SurfaceFamily::Plane,
            upstream_geometry_binding_identity: upstream_geometry_binding_identity.into(),
            topology_query_surface: topology_query_surface.into(),
        }
    }

    pub fn family(&self) -> SurfaceFamily {
        self.family
    }

    pub fn upstream_geometry_binding_identity(&self) -> &str {
        &self.upstream_geometry_binding_identity
    }

    pub fn topology_query_surface(&self) -> &str {
        &self.topology_query_surface
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CertifiedSurfaceSupport {
    plane_support: CertifiedPlaneSupport,
    receipts: SurfaceSupportReceiptSet,
    geometry_snapshot: SurfaceSupportGeometrySnapshot,
}

impl CertifiedSurfaceSupport {
    pub(crate) fn new(
        plane_support: CertifiedPlaneSupport,
        receipts: SurfaceSupportReceiptSet,
        geometry_snapshot: SurfaceSupportGeometrySnapshot,
    ) -> Self {
        Self {
            plane_support,
            receipts,
            geometry_snapshot,
        }
    }

    pub fn certified_plane_support(&self) -> &CertifiedPlaneSupport {
        &self.plane_support
    }

    pub fn receipts(&self) -> &SurfaceSupportReceiptSet {
        &self.receipts
    }

    pub fn geometry_snapshot(&self) -> &SurfaceSupportGeometrySnapshot {
        &self.geometry_snapshot
    }

    pub fn can_enter_local_frame_workload(&self) -> bool {
        self.plane_support.family() == SurfaceFamily::Plane
    }

    pub fn can_enter_projection_workload(&self) -> bool {
        self.plane_support.family() == SurfaceFamily::Plane
    }

    pub fn can_enter_operator_execution(&self) -> bool {
        false
    }
}
