use topology::facade::TopologySeed;
use worth_spatial::facade::workload_binding::{
    GeometryBindingWorkload, PlanarEdgeCarrierSet, PlanarFaceCarrierSet, PlanarLoopCarrierSet,
    UnsupportedGeometryBinding, UnsupportedGeometryBindingReasonCode,
    UnsupportedGeometryCarrierFamily,
};

#[test]
fn unsupported_geometry_binding_is_typed_and_non_consumable() {
    let topology = TopologySeed::cube()
        .build()
        .expect("cube topology seed should be admitted");

    for unsupported_family in [
        UnsupportedGeometryCarrierFamily::NonPlanarFace,
        UnsupportedGeometryCarrierFamily::FreeformSurface,
        UnsupportedGeometryCarrierFamily::VolumetricFeature,
        UnsupportedGeometryCarrierFamily::Unknown,
    ] {
        let denial = GeometryBindingWorkload::for_topology_seed(&topology)
            .declared("try unsupported carrier family")
            .with_planar_faces(PlanarFaceCarrierSet::for_seed_faces(&topology))
            .with_planar_edges(PlanarEdgeCarrierSet::for_seed_edges(&topology))
            .with_planar_loops(PlanarLoopCarrierSet::for_seed_loops(&topology))
            .with_unsupported_family(unsupported_family)
            .admit()
            .expect_err("unsupported carrier families cannot enter admitted binding");

        assert_eq!(
            denial.reason_code(),
            UnsupportedGeometryBindingReasonCode::UnsupportedCarrierFamily
        );
        assert_eq!(
            denial.human_reason(),
            format!(
                "{} is not admitted for geometry binding in this workload phase.",
                unsupported_family.human_label()
            )
        );
        assert_eq!(
            denial.requested_unsupported_family(),
            Some(unsupported_family)
        );
        assert_eq!(
            denial.topology_receipt_identity(),
            Some(
                topology
                    .query_receipts()
                    .declaration_receipt()
                    .identity()
                    .name()
            )
        );
        assert_eq!(
            denial.topology_query_surface(),
            Some(topology.query_receipts().query_surface())
        );
        assert!(!denial.can_enter_surface_support());
    }
}

#[test]
fn dirty_topology_clean_fail_binding_denial_is_human_readable() {
    let clean_fail = TopologySeed::self_intersecting_loop()
        .build()
        .expect_err("dirty topology must clean-fail before spatial binding");

    let denial = UnsupportedGeometryBinding::from_topology_clean_fail(&clean_fail);

    assert_eq!(
        denial.reason_code(),
        UnsupportedGeometryBindingReasonCode::DirtyTopology
    );
    assert_eq!(
        denial.human_reason(),
        "Topology clean-fail receipt explicitly denies spatial binding."
    );
    assert!(!denial.can_enter_surface_support());
}

#[test]
fn partial_geometry_binding_cannot_skip_topology_targets() {
    let topology = TopologySeed::cube()
        .build()
        .expect("cube topology seed should be admitted");

    let denial = GeometryBindingWorkload::for_topology_seed(&topology)
        .declared("try face-only binding")
        .with_planar_faces(PlanarFaceCarrierSet::for_seed_faces(&topology))
        .admit()
        .expect_err("face-only binding cannot stand in for full topology binding");

    assert_eq!(
        denial.reason_code(),
        UnsupportedGeometryBindingReasonCode::MismatchedCarrierTarget
    );
    assert_eq!(
        denial.human_reason(),
        "Planar edge carriers must exactly match the topology seed edge targets."
    );
    assert!(!denial.can_enter_surface_support());
}
