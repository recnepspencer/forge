use topology::facade::TopologySeed;
use worth_spatial::facade::workload_binding::{
    GeometryBindingWorkload, GeometryCarrierFamily, PlanarEdgeCarrierSet, PlanarFaceCarrierSet,
    PlanarLoopCarrierSet,
};

#[test]
fn geometry_binding_workload_consumes_topology_seed_receipts() {
    let topology = TopologySeed::cube()
        .build()
        .expect("cube topology seed should be admitted");

    let bound_geometry = GeometryBindingWorkload::for_topology_seed(&topology)
        .declared("bind cube topology to planar carriers")
        .with_planar_faces(PlanarFaceCarrierSet::for_seed_faces(&topology))
        .with_planar_edges(PlanarEdgeCarrierSet::for_seed_edges(&topology))
        .with_planar_loops(PlanarLoopCarrierSet::for_seed_loops(&topology))
        .admit()
        .expect("complete planar carriers should bind");

    let receipts = bound_geometry.receipts();
    let counters = topology.counters();

    assert_eq!(
        receipts.topology_identity(),
        topology
            .query_receipts()
            .declaration_receipt()
            .identity()
            .name()
    );
    assert_eq!(
        receipts.topology_query_surface(),
        topology.query_receipts().query_surface()
    );
    assert!(receipts.has_binding_declaration_receipt());
    assert!(receipts.has_geometry_carrier_receipts());
    assert!(bound_geometry.can_enter_surface_support());

    assert_eq!(receipts.counters().face_bindings(), counters.face_count());
    assert_eq!(receipts.counters().edge_bindings(), counters.edge_count());
    assert_eq!(receipts.counters().loop_bindings(), counters.loop_count());
    assert_eq!(
        receipts.counters().topology_targets(),
        counters.face_count() + counters.edge_count() + counters.loop_count()
    );
    assert_eq!(
        receipts.counters().geometry_carriers(),
        bound_geometry.planar_faces().len()
            + bound_geometry.planar_edges().len()
            + bound_geometry.planar_loops().len()
    );

    let mut topology_entity_identities = topology.entity_identities().face_identity_tokens();
    topology_entity_identities.extend(topology.entity_identities().edge_identity_tokens());
    topology_entity_identities.extend(topology.entity_identities().loop_identity_tokens());

    for carrier in receipts.carrier_receipts() {
        assert!(carrier.is_distinct_from_topology_identity());
        assert!(matches!(
            carrier.family(),
            GeometryCarrierFamily::PlanarFace
                | GeometryCarrierFamily::PlanarEdge
                | GeometryCarrierFamily::PlanarLoop
        ));
        assert!(
            topology
                .query_receipts()
                .declaration_receipt()
                .identity()
                .name()
                != carrier.target_topology_identity()
        );
        assert!(topology_entity_identities
            .iter()
            .any(|identity| identity == carrier.target_topology_identity()));
    }
}

#[test]
fn geometry_binding_workload_rejects_missing_binding_receipts() {
    let topology = TopologySeed::cube()
        .build()
        .expect("cube topology seed should be admitted");

    let denial = GeometryBindingWorkload::for_topology_seed(&topology)
        .declared("")
        .with_planar_faces(PlanarFaceCarrierSet::for_seed_faces(&topology))
        .with_planar_edges(PlanarEdgeCarrierSet::for_seed_edges(&topology))
        .with_planar_loops(PlanarLoopCarrierSet::for_seed_loops(&topology))
        .admit()
        .expect_err("blank binding declarations cannot produce receipts");

    assert_eq!(
        denial.reason_code(),
        worth_spatial::facade::workload_binding::UnsupportedGeometryBindingReasonCode::MissingBindingDeclaration
    );
    assert!(denial.human_reason().contains("human-readable declaration"));
    assert!(!denial.can_enter_surface_support());
}

#[test]
fn geometry_binding_rejects_carriers_from_another_topology_receipt() {
    let topology = TopologySeed::cube()
        .with_declaration("authoritative cube topology")
        .build()
        .expect("cube topology seed should be admitted");
    let same_shape_other_receipt = TopologySeed::cube()
        .with_declaration("different cube topology receipt")
        .build()
        .expect("second cube topology seed should be admitted");

    assert_ne!(
        topology
            .query_receipts()
            .declaration_receipt()
            .identity()
            .name(),
        same_shape_other_receipt
            .query_receipts()
            .declaration_receipt()
            .identity()
            .name()
    );

    let denial = GeometryBindingWorkload::for_topology_seed(&topology)
        .declared("try cross-topology carrier substitution")
        .with_planar_faces(PlanarFaceCarrierSet::for_seed_faces(
            &same_shape_other_receipt,
        ))
        .with_planar_edges(PlanarEdgeCarrierSet::for_seed_edges(
            &same_shape_other_receipt,
        ))
        .with_planar_loops(PlanarLoopCarrierSet::for_seed_loops(
            &same_shape_other_receipt,
        ))
        .admit()
        .expect_err("carrier origin receipt must match target topology receipt");

    assert_eq!(
        denial.reason_code(),
        worth_spatial::facade::workload_binding::UnsupportedGeometryBindingReasonCode::MismatchedCarrierTarget
    );
    assert_eq!(
        denial.human_reason(),
        "Planar face carriers must originate from the same topology receipt as the binding workload."
    );
    assert!(!denial.can_enter_surface_support());
}
