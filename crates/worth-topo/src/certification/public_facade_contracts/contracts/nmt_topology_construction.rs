fn _nmt_topology_construction_public_facade_contract(
) -> Result<NmtTopologyConstructionReceipt, topology::facade::NmtTopologyConstructionDenial> {
    let _: fn(OpenWireChainSpec) -> NmtTopologyConstruction =
        NmtTopologyConstruction::open_wire_chain;
    let _: fn(OpenSheetPatchSpec) -> NmtTopologyConstruction =
        NmtTopologyConstruction::open_sheet_patch;
    let _: fn(OpenRadialFanSpec) -> NmtTopologyConstruction =
        NmtTopologyConstruction::open_radial_fan;
    let _: fn(OpenLayerStackSpec) -> NmtTopologyConstruction =
        NmtTopologyConstruction::open_layer_stack;

    let wire = NmtTopologyConstruction::open_wire_chain(OpenWireChainSpec::new().edges(9))
        .declared("public API open wire construction")
        .construct()?;
    assert_eq!(wire.counters().edge_count(), 9);
    assert_eq!(wire.topology_seed_receipt().counters().edge_count(), 9);
    assert_eq!(
        wire.open_boundary().boundary_half_edge_count(),
        wire.counters().boundary_half_edge_count()
    );
    assert!(wire.query_surface().contains("open-wire-chain"));
    assert_eq!(
        wire.query_declaration_identity(),
        "public API open wire construction"
    );
    assert_eq!(
        wire.topology_posture().posture(),
        NmtTopologyPosture::OpenWire
    );

    let sheet = NmtTopologyConstruction::open_sheet_patch(OpenSheetPatchSpec::new().strips(12))
        .declared("public API open sheet construction")
        .construct()?;
    assert_eq!(sheet.counters().face_count(), 12);
    assert_eq!(sheet.topology_seed_receipt().counters().face_count(), 12);
    assert!(sheet.open_boundary().boundary_half_edge_count() > 0);
    assert!(!sheet.open_boundary().boundary_digest().is_empty());
    assert_ne!(
        sheet.open_boundary().boundary_digest(),
        sheet.pattern_identity().identity_digest()
    );
    assert_eq!(
        sheet.topology_posture().posture(),
        NmtTopologyPosture::OpenSheet
    );

    let fan =
        NmtTopologyConstruction::open_radial_fan(OpenRadialFanSpec::new().incident_faces(11))
            .declared("public API open radial fan construction")
            .construct()?;
    assert_eq!(fan.radial_adjacency().non_manifold_edge_count(), 1);
    assert_eq!(
        fan.radial_adjacency().non_manifold_edge_count(),
        fan.counters().non_manifold_edge_count()
    );
    assert!(!fan.radial_adjacency().radial_digest().is_empty());
    assert_ne!(
        fan.radial_adjacency().radial_digest(),
        fan.pattern_identity().identity_digest()
    );
    assert_eq!(
        fan.topology_posture().posture(),
        NmtTopologyPosture::OpenNonManifold
    );

    let stack = NmtTopologyConstruction::open_layer_stack(
        OpenLayerStackSpec::new()
            .layers(5)
            .layer_pattern(OpenLayerPattern::sheet_patch(
                OpenSheetPatchSpec::new().strips(8),
            ))
            .with_layer_identity()
            .with_open_boundary_receipts()
            .with_radial_adjacency_receipts(),
    )
    .declared("public API layered open construction")
    .construct()?;
    assert_eq!(stack.counters().layer_count(), 5);
    assert_eq!(stack.counters().face_count(), 40);
    assert_eq!(stack.pattern_identity().layer_count(), 5);
    assert_eq!(stack.topology_seed_receipt().counters().face_count(), 40);
    assert_eq!(
        stack.open_boundary().boundary_half_edge_count(),
        stack.counters().boundary_half_edge_count()
    );
    assert_eq!(
        stack.topology_posture().posture(),
        NmtTopologyPosture::LayeredOpen
    );

    let missing_layer_identity = NmtTopologyConstruction::open_layer_stack(
        OpenLayerStackSpec::new()
            .layers(3)
            .layer_pattern(OpenLayerPattern::sheet_patch(
                OpenSheetPatchSpec::new().strips(4),
            )),
    )
    .declared("missing layer authority")
    .construct()
    .expect_err("layer stack must request evidence receipts");
    assert_eq!(
        missing_layer_identity.class(),
        NmtTopologyConstructionDenialClass::MissingRequiredEvidence
    );
    assert!(missing_layer_identity.reason().contains("layer identity"));

    let missing_open_boundary = NmtTopologyConstruction::open_layer_stack(
        OpenLayerStackSpec::new()
            .layers(3)
            .with_layer_identity()
            .layer_pattern(OpenLayerPattern::sheet_patch(
                OpenSheetPatchSpec::new().strips(4),
            )),
    )
    .declared("missing boundary authority")
    .construct()
    .expect_err("layer stack must request open-boundary receipts");
    assert_eq!(
        missing_open_boundary.class(),
        NmtTopologyConstructionDenialClass::MissingRequiredEvidence
    );
    assert!(missing_open_boundary.reason().contains("open-boundary"));

    let missing_radial_adjacency = NmtTopologyConstruction::open_layer_stack(
        OpenLayerStackSpec::new()
            .layers(3)
            .with_layer_identity()
            .with_open_boundary_receipts()
            .layer_pattern(OpenLayerPattern::sheet_patch(
                OpenSheetPatchSpec::new().strips(4),
            )),
    )
    .declared("missing radial authority")
    .construct()
    .expect_err("layer stack must request radial-adjacency receipts");
    assert_eq!(
        missing_radial_adjacency.class(),
        NmtTopologyConstructionDenialClass::MissingRequiredEvidence
    );
    assert!(missing_radial_adjacency.reason().contains("radial-adjacency"));

    let invalid_layer_count = NmtTopologyConstruction::open_layer_stack(
        OpenLayerStackSpec::new()
            .layers(1)
            .with_layer_identity()
            .with_open_boundary_receipts()
            .with_radial_adjacency_receipts(),
    )
    .declared("invalid layer count")
    .construct()
    .expect_err("layer stack layer count must be bounded");
    assert_eq!(
        invalid_layer_count.class(),
        NmtTopologyConstructionDenialClass::UnsupportedCardinality
    );
    assert!(invalid_layer_count.reason().contains("2 through 16"));

    Ok(stack)
}
