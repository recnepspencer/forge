use topology::facade::TopologySeed;
use worth_spatial::facade::planar_boolean_common_plane::{
    PlanarBooleanCommonPlaneAgreementWorkload, PlanarBooleanCommonPlaneLocalFrameSelectionReceipt,
    PlanarBooleanCommonPlaneSharedPlaneIdentityReceipt,
};
use worth_spatial::facade::planar_contract_bundle::{
    PlanarM7ReadinessBundle, PlanarM7ReadinessReceipt, PlanarM7ReadinessSupportPosture,
};
use worth_spatial::facade::surface_support::{
    CertifiedSurfaceSupport, SurfaceFamily, SurfaceSupportWorkload,
};
use worth_spatial::facade::workload_binding::{
    BoundGeometryWorkload, GeometryBindingWorkload, PlanarEdgeCarrierSet, PlanarFaceCarrierSet,
    PlanarLoopCarrierSet,
};

use crate::public_api_planar_contract_bundle::m7_readiness::fixture::{
    bundle_contracts, m7_readiness_parts,
};

#[test]
fn planar_boolean_common_plane_local_frame_selection_replays_to_one_identity() {
    run_with_large_stack(|| {
        let shared_plane = shared_plane_identity_receipt("phase7.1 local frame parity");
        let readiness = m7_readiness_receipt("phase7.1 local frame parity");

        let first = PlanarBooleanCommonPlaneLocalFrameSelectionReceipt::from_shared_plane_identity_and_m7_readiness(
            &shared_plane,
            &readiness,
        )
        .expect("local-frame selection should certify");
        let replayed = PlanarBooleanCommonPlaneLocalFrameSelectionReceipt::from_shared_plane_identity_and_m7_readiness(
            &shared_plane,
            &readiness,
        )
        .expect("replayed local-frame selection should preserve the same identity");

        assert_eq!(first, replayed);
        assert_eq!(
            first.local_frame_fact_digest(),
            readiness.local_frame_receipt().fact_digest()
        );
        assert_eq!(
            first.frame_identity(),
            readiness.local_frame_receipt().basis().frame_identity()
        );
        assert_eq!(
            first.topology_basis_identity(),
            readiness.topology_basis_identity()
        );
        assert_eq!(
            first.movement_rotation_posture_identity(),
            readiness.movement_rotation_posture_identity()
        );
    });
}

fn m7_readiness_receipt(world: &'static str) -> PlanarM7ReadinessReceipt {
    let parts = m7_readiness_parts(world);
    PlanarM7ReadinessBundle::from_certified_planar_bundle(parts.readiness)
        .with_structural_identity(parts.structural)
        .with_motion_posture(parts.motion)
        .with_retained_planar_facts(parts.retained)
        .with_projection_consumed_facts(parts.projected)
        .with_recovery_posture(parts.recovery)
        .with_diagnostics(parts.diagnostics)
        .with_support_posture(PlanarM7ReadinessSupportPosture::support_gated(
            "M7 boolean split/classify/assemble is support-gated until Milestone 7",
        ))
        .compile(&bundle_contracts(world))
        .expect("M7 readiness plan")
        .certify()
        .expect("M7 readiness receipt")
}

fn shared_plane_identity_receipt(
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

fn run_with_large_stack(body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new()
        .name("planar-boolean-common-plane-local-frame-selection".to_string())
        .stack_size(16 * 1024 * 1024)
        .spawn(body)
        .expect("local-frame selection contract thread should spawn")
        .join()
        .expect("local-frame selection contract thread should finish");
}
