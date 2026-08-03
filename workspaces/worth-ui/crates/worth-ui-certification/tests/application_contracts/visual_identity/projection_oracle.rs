use worth_ui_runtime::facade::mounted::{
    UiMountedAllocationProjection, UiMountedClipProjection, UiMountedHitTestProjection,
    UiMountedLayerProjection, UiMountedNodeProjectionView, UiMountedPaintPrimitiveKind,
    UiMountedPaintProjection, UiMountedParticipationStatus, UiMountedProjectionView,
};

#[derive(Clone, Copy)]
struct ExpectedMechanics {
    bounds: Option<[f32; 4]>,
    paint_order: Option<u32>,
    hit_order: Option<u32>,
    paint_participation: UiMountedParticipationStatus,
    hit_participation: UiMountedParticipationStatus,
}

pub(super) fn assert_four_way_projection(projection: &UiMountedProjectionView) {
    assert_eq!(projection.nodes().len(), 4);
    assert!(projection.clips().rows().is_empty());
    assert_eq!(projection.filled_rects().rows().len(), 2);
    assert_eq!(projection.hit_tests().rows().len(), 2);
    for expected in expected_mechanics() {
        let node = node_with_bounds(projection, expected.bounds);
        assert_node_mechanics(projection, node, expected);
    }
    assert_ordinary_lane_tables(projection);
    assert_order_sets(projection);
}

fn expected_mechanics() -> [ExpectedMechanics; 4] {
    [
        ExpectedMechanics {
            bounds: Some([0.0, 0.0, 160.0, 96.0]),
            paint_order: Some(7),
            hit_order: None,
            paint_participation: UiMountedParticipationStatus::Admitted,
            hit_participation: UiMountedParticipationStatus::Deferred,
        },
        ExpectedMechanics {
            bounds: Some([8.0, 8.0, 144.0, 80.0]),
            paint_order: None,
            hit_order: Some(1),
            paint_participation: UiMountedParticipationStatus::Deferred,
            hit_participation: UiMountedParticipationStatus::Admitted,
        },
        ExpectedMechanics {
            bounds: Some([16.0, 12.0, 128.0, 72.0]),
            paint_order: Some(3),
            hit_order: Some(0),
            paint_participation: UiMountedParticipationStatus::Admitted,
            hit_participation: UiMountedParticipationStatus::Admitted,
        },
        ExpectedMechanics {
            bounds: None,
            paint_order: None,
            hit_order: None,
            paint_participation: UiMountedParticipationStatus::Deferred,
            hit_participation: UiMountedParticipationStatus::Deferred,
        },
    ]
}

fn assert_order_sets(projection: &UiMountedProjectionView) {
    let mut paint_orders = projection
        .filled_rects()
        .rows()
        .iter()
        .map(|row| row.layer_semantic_order())
        .collect::<Vec<_>>();
    let mut hit_orders = projection
        .hit_tests()
        .rows()
        .iter()
        .map(|row| row.order().rank())
        .collect::<Vec<_>>();
    paint_orders.sort_unstable();
    hit_orders.sort_unstable();
    assert_eq!(paint_orders, vec![3, 7]);
    assert_eq!(hit_orders, vec![0, 1]);
}

fn assert_ordinary_lane_tables(projection: &UiMountedProjectionView) {
    assert_eq!(projection.layers().rows().len(), 1);
    let layer = projection.layers().rows()[0];
    assert_eq!(layer.semantic_order(), 0);
    assert_eq!(layer.clip(), UiMountedClipProjection::Unclipped);
    assert_eq!(projection.paint_batches().rows().len(), 1);
    let batch = projection.paint_batches().rows()[0];
    assert_eq!(
        batch.primitive_count(),
        5,
        "ordinary root-shell work covers four authored components plus the authored surface"
    );
    assert_eq!(batch.resource(), None);
    assert_eq!(
        batch.primitive_kind(),
        UiMountedPaintPrimitiveKind::OrdinaryLaneSummary
    );
    assert!(matches!(
        batch.layer(),
        UiMountedLayerProjection::Layer(reference) if reference.index() == 0
    ));
}

fn node_with_bounds(
    projection: &UiMountedProjectionView,
    expected: Option<[f32; 4]>,
) -> &UiMountedNodeProjectionView {
    let matches = projection
        .nodes()
        .iter()
        .filter(|node| allocation_bounds(node) == expected)
        .collect::<Vec<_>>();
    assert_eq!(
        matches.len(),
        1,
        "each independent geometry oracle must identify exactly one node"
    );
    matches[0]
}

fn allocation_bounds(node: &UiMountedNodeProjectionView) -> Option<[f32; 4]> {
    match node.allocation() {
        UiMountedAllocationProjection::Known { bounds, .. } => {
            Some([bounds.x(), bounds.y(), bounds.width(), bounds.height()])
        }
        UiMountedAllocationProjection::Omitted(_) => None,
        UiMountedAllocationProjection::PortalAnchorObservation { .. } => {
            panic!("the ordinary filesystem world cannot produce portal geometry")
        }
    }
}

fn assert_node_mechanics(
    projection: &UiMountedProjectionView,
    node: &UiMountedNodeProjectionView,
    expected: ExpectedMechanics,
) {
    assert_eq!(
        node.participation().paint().status(),
        expected.paint_participation
    );
    assert_eq!(
        node.participation().hit_test().status(),
        expected.hit_participation
    );
    match (node.paint(), expected.paint_order) {
        (UiMountedPaintProjection::FilledRect(reference), Some(order)) => {
            let mechanic = projection.filled_rects().resolve(reference).unwrap();
            assert_eq!(mechanic.node_receipt(), node.node_receipt());
            assert_eq!(mechanic.layer_semantic_order(), order);
            assert_eq!(allocation_bounds(node), Some(box_values(mechanic.bounds())));
        }
        (UiMountedPaintProjection::Omitted(_), None) => {}
        (actual, expected) => panic!("paint mechanic mismatch: {actual:?} vs {expected:?}"),
    }
    match (node.hit_test(), expected.hit_order) {
        (UiMountedHitTestProjection::Region(reference), Some(order)) => {
            let mechanic = projection.hit_tests().resolve(reference).unwrap();
            assert_eq!(mechanic.node_receipt(), node.node_receipt());
            assert_eq!(mechanic.order().rank(), order);
            assert_eq!(allocation_bounds(node), Some(box_values(mechanic.bounds())));
        }
        (UiMountedHitTestProjection::Omitted(_), None) => {}
        (actual, expected) => panic!("hit mechanic mismatch: {actual:?} vs {expected:?}"),
    }
}

fn box_values(bounds: worth_ui_runtime::facade::mounted::UiMountedCanonicalBox) -> [f32; 4] {
    [bounds.x(), bounds.y(), bounds.width(), bounds.height()]
}
