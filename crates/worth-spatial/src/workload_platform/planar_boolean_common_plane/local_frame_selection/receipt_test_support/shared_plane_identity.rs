use topology::facade::TopologySeed;

use crate::facade::planar_boolean_common_plane::{
    PlanarBooleanCommonPlaneAgreementWorkload, PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt,
};
use crate::facade::surface_support::{
    CertifiedSurfaceSupport, SurfaceFamily, SurfaceSupportWorkload,
};
use crate::facade::workload_binding::{
    BoundGeometryWorkload, GeometryBindingWorkload, PlanarEdgeCarrierSet, PlanarFaceCarrierSet,
    PlanarLoopCarrierSet,
};

pub(crate) fn shared_plane_identity_receipt(
    declaration: &str,
) -> PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt {
    let left = certified_surface_support(declaration);
    let right = certified_surface_support(declaration);
    let agreement =
        PlanarBooleanCommonPlaneAgreementWorkload::for_surface_support_pair(left, right)
            .declared(declaration)
            .certify()
            .expect("equivalent supports should certify plane agreement");
    PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt::from_plane_agreement(&agreement)
}

fn certified_surface_support(declaration: &str) -> CertifiedSurfaceSupport {
    let topology = TopologySeed::single_face_loop(4)
        .with_declaration(format!("{declaration} topology"))
        .build()
        .expect("single-face topology should build");
    let bound_geometry: BoundGeometryWorkload =
        GeometryBindingWorkload::for_topology_seed(&topology)
            .declared(format!("bind {declaration}"))
            .with_planar_faces(PlanarFaceCarrierSet::for_seed_faces(&topology))
            .with_planar_edges(PlanarEdgeCarrierSet::for_seed_edges(&topology))
            .with_planar_loops(PlanarLoopCarrierSet::for_seed_loops(&topology))
            .admit()
            .expect("geometry binding should admit");

    SurfaceSupportWorkload::for_bound_geometry(bound_geometry)
        .declared(format!("support {declaration}"))
        .with_surface_family(SurfaceFamily::Plane)
        .certify()
        .expect("surface support should certify")
}
