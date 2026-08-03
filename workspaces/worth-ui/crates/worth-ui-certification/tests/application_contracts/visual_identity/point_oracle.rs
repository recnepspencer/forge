use worth_ui::facade::inspection::{
    UiClientPhysicalPixel, UiGeometryOnly, UiVisualHitTestOutcome, UiVisualSnapshotDenial,
    UiVisualSnapshotReceipt, UiVisualVisibleOutcome,
};

pub(super) fn assert_four_way_point_adjudication(
    receipt: &UiVisualSnapshotReceipt<UiGeometryOnly>,
) {
    assert_eq!(
        receipt.coordinates().client_physical_dimensions(),
        [200, 120]
    );
    assert_eq!(
        receipt.coordinates().viewport_logical_dimensions(),
        [160.0, 96.0]
    );
    assert_eq!(receipt.coordinates().scale(), [1.25, 1.25]);
    assert_eq!(receipt.coordinates().translation(), [0.0, 0.0]);
    receipt.with_coordinate_scope(|scope| {
        let paint_only = adjudicate(&scope, 195, 115);
        assert_visible_declaration(&paint_only, 0);
        assert!(matches!(
            paint_only.hit_test(),
            UiVisualHitTestOutcome::None
        ));

        let hit_only = adjudicate(&scope, 15, 100);
        assert_visible_declaration(&hit_only, 0);
        assert_hit_declaration(&hit_only, 1, 1);

        let paint_and_hit = adjudicate(&scope, 100, 50);
        assert_visible_declaration(&paint_and_hit, 0);
        assert_hit_declaration(&paint_and_hit, 2, 0);
        assert_opaque_occlusion(&paint_and_hit);

        let outside_extent =
            UiClientPhysicalPixel::new(201, 121).expect("boundary oracle is nonnegative");
        assert!(matches!(
            scope.client_pixel(outside_extent),
            Err(UiVisualSnapshotDenial::OutsideCapturedPixelExtent)
        ));

        let inside_half_open_edge = adjudicate(&scope, 189, 109);
        assert_hit_declaration(&inside_half_open_edge, 1, 1);
        let outside_half_open_edge = adjudicate(&scope, 190, 109);
        assert!(matches!(
            outside_half_open_edge.hit_test(),
            UiVisualHitTestOutcome::None
        ));
    });
}

fn adjudicate<'snapshot>(
    scope: &worth_ui::facade::inspection::UiVisualCoordinateScope<'snapshot>,
    x: i64,
    y: i64,
) -> worth_ui::facade::inspection::UiVisualPointAdjudication {
    let point = UiClientPhysicalPixel::new(x, y).expect("the oracle uses nonnegative pixels");
    scope
        .adjudicate_point(scope.client_pixel(point).unwrap())
        .expect("a sealed exact snapshot supports point adjudication")
}

fn assert_visible_declaration(
    adjudication: &worth_ui::facade::inspection::UiVisualPointAdjudication,
    declaration_index: usize,
) {
    let UiVisualVisibleOutcome::Contributors(stack) = adjudication.visible() else {
        panic!("the named point must have exact visible contributors");
    };
    let frontmost = stack
        .frontmost()
        .expect("a contributor outcome has a frontmost row");
    assert_eq!(
        frontmost
            .identity_trace()
            .authored_provenance()
            .declaration_index(),
        declaration_index
    );
    assert_trace_is_complete(frontmost.identity_trace());
    assert_query_cost_is_explicit(adjudication);
}

fn assert_hit_declaration(
    adjudication: &worth_ui::facade::inspection::UiVisualPointAdjudication,
    declaration_index: usize,
    total_order: u32,
) {
    let UiVisualHitTestOutcome::Target(target) = adjudication.hit_test() else {
        panic!("the named point must have one exact hit target");
    };
    assert_eq!(target.total_order(), total_order);
    assert_eq!(
        target
            .identity_trace()
            .authored_provenance()
            .declaration_index(),
        declaration_index
    );
    assert_trace_is_complete(target.identity_trace());
}

fn assert_opaque_occlusion(adjudication: &worth_ui::facade::inspection::UiVisualPointAdjudication) {
    let UiVisualVisibleOutcome::Contributors(stack) = adjudication.visible() else {
        panic!("the overlap point must have visible evidence");
    };
    assert_eq!(
        stack.contributors().len(),
        1,
        "frontmost opaque paint excludes the covered contributor"
    );
    let UiVisualHitTestOutcome::Target(target) = adjudication.hit_test() else {
        panic!("the overlap point must also have a hit target");
    };
    assert_ne!(
        stack.frontmost().unwrap().identity_trace().mounted_node(),
        target.identity_trace().mounted_node(),
        "paint and hit selection must remain independent"
    );
}

fn assert_trace_is_complete(trace: &worth_ui::facade::inspection::UiVisualIdentityTrace) {
    let mounted = trace.mounted_node();
    assert_ne!(mounted.node_receipt(), 0);
    assert_ne!(mounted.mounted_instance(), 0);
    assert_ne!(mounted.incarnation(), 0);
    assert_ne!(trace.graph_node().diagnostic_value(), 0);
    assert_ne!(trace.declaration().diagnostic_value(), 0);
    assert!(!trace.declaration().authored_semantic_name().is_empty());
    assert!(!trace.evidence().is_empty());
    assert!(
        trace.evidence().windows(2).all(|pair| pair[0] < pair[1]),
        "trace evidence is ordered and deduplicated"
    );
}

fn assert_query_cost_is_explicit(
    adjudication: &worth_ui::facade::inspection::UiVisualPointAdjudication,
) {
    assert_eq!(adjudication.budget().maximum_results(), 32);
    assert_eq!(adjudication.budget().maximum_candidates(), 4_096);
    assert!(adjudication.cost().spatial_index_probes() > 0);
    assert!(adjudication.cost().candidates_considered() > 0);
    assert!(adjudication.cost().trace_index_probes() > 0);
}
