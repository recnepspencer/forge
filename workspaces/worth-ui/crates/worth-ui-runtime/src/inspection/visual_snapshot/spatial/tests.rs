use super::record::UiVisibleOpacity;
use super::{validate_and_index, UiSpatialValidationDenial};

#[path = "tests/cost_bounds.rs"]
mod cost_bounds;

struct SpatialWorld {
    frame: worth_ui_host_contract::UiMountedFrameIdentity,
    surface: worth_ui_host_contract::UiSemanticSurfaceIdentity,
    binding: worth_ui_host_contract::UiSurfaceBindingGeneration,
    instance: worth_ui_host_contract::UiMountedInstanceIdentity,
    receipt: worth_ui_host_contract::UiMountedNodeReceiptIdentity,
}

#[test]
fn validation_preserves_disjoint_typed_mechanics_and_half_open_edges() {
    let world = SpatialWorld::new();
    let first = paint(&world, bounds(0.0, 0.0, 8.0, 8.0), 1, u8::MAX);
    let second = paint(&world, bounds(16.0, 0.0, 8.0, 8.0), 2, 128);
    let hit = hit_test(&world, bounds(0.0, 0.0, 24.0, 8.0), 7);
    let basis = crate::mounting::UiMountedVisualRegionBasis::new(
        vec![first, second].into_boxed_slice(),
        vec![hit].into_boxed_slice(),
    );
    let observed = [
        observed_hit(hit, 7),
        observed_paint(second, 2),
        observed_paint(first, 1),
    ];

    let indexed = validate_and_index(41, &basis, &observed, transform([32, 16]))
        .expect("shuffled exact rows validate against their typed mechanics");
    let (visible, hit_test, cost) = indexed.into_parts();
    assert_eq!(visible.len(), 2);
    assert_eq!(hit_test.len(), 1);
    assert_eq!(cost.region_records_examined(), 3);
    assert!(cost.retained_structural_bytes() > 0);

    let (first_candidates, first_probes, first_exhausted) =
        visible.point_candidates(point(0, 0), 16).into_parts();
    assert!(!first_exhausted);
    assert_eq!(first_candidates.len(), 1);
    assert_first_visible_record(*first_candidates[0], &world, first);
    assert!(first_probes > 0);

    assert!(visible
        .point_candidates(point(8, 0), 16)
        .into_parts()
        .0
        .is_empty());
    let second_candidates = visible.point_candidates(point(16, 0), 16).into_parts().0;
    assert_eq!(second_candidates.len(), 1);
    assert_eq!(
        second_candidates[0].opacity(),
        UiVisibleOpacity::Composited(128)
    );
    let hit_candidates = hit_test.point_candidates(point(23, 7), 16).into_parts().0;
    assert_eq!(hit_candidates.len(), 1);
    assert_eq!(hit_candidates[0].node_receipt(), world.receipt);
    assert_eq!(hit_candidates[0].total_order().rank(), 7);
    assert_eq!(
        hit_candidates[0].source_projection_digest(),
        hit.semantic_digest()
    );
}

#[test]
fn validation_rejects_missing_duplicate_cross_participation_and_wrong_order_rows() {
    let world = SpatialWorld::new();
    let paint = paint(&world, bounds(0.0, 0.0, 8.0, 8.0), 3, u8::MAX);
    let hit = hit_test(&world, bounds(0.0, 0.0, 8.0, 8.0), 5);
    let basis = crate::mounting::UiMountedVisualRegionBasis::new(
        vec![paint].into_boxed_slice(),
        vec![hit].into_boxed_slice(),
    );
    let exact_paint = observed_paint(paint, 3);
    let exact_hit = observed_hit(hit, 5);
    let wrong_hit_order = observed_hit(hit, 6);
    let paint_claimed_as_hit = observed(
        paint.node_receipt(),
        realized_geometry(paint.bounds(), paint.clip_bounds()),
        realized_ordering(
            3,
            worth_ui_host_contract::UiHostRealizedRegionParticipation::HitTest,
        ),
    );
    let wrong_bounds = bounds(1.0, 0.0, 8.0, 8.0);
    let wrong_geometry = observed(
        paint.node_receipt(),
        realized_geometry(wrong_bounds, wrong_bounds),
        realized_ordering(
            3,
            worth_ui_host_contract::UiHostRealizedRegionParticipation::Paint,
        ),
    );
    let foreign_instance =
        worth_ui_host_contract::UiMountedInstanceIdentity::mint_unbound().unwrap();
    let foreign_receipt = worth_ui_host_contract::UiMountedNodeReceiptIssuer::mint_for(world.frame)
        .unwrap()
        .receipt_for(foreign_instance);
    let foreign_identity = observed(
        foreign_receipt,
        realized_geometry(paint.bounds(), paint.clip_bounds()),
        realized_ordering(
            3,
            worth_ui_host_contract::UiHostRealizedRegionParticipation::Paint,
        ),
    );

    for rows in [
        vec![exact_paint],
        vec![exact_paint, exact_paint],
        vec![exact_paint, wrong_hit_order],
        vec![paint_claimed_as_hit, exact_hit],
        vec![wrong_geometry, exact_hit],
        vec![foreign_identity, exact_hit],
    ] {
        assert_eq!(
            validate_and_index(7, &basis, &rows, transform([16, 16])).err(),
            Some(UiSpatialValidationDenial::ProtocolMismatch)
        );
    }
}

#[test]
fn viewport_clipping_excludes_offscreen_and_right_edge_pixels() {
    let world = SpatialWorld::new();
    let partially_offscreen = paint(&world, bounds(-4.0, 0.0, 8.0, 8.0), 1, u8::MAX);
    let basis = crate::mounting::UiMountedVisualRegionBasis::new(
        vec![partially_offscreen].into_boxed_slice(),
        Box::new([]),
    );
    let indexed = validate_and_index(
        11,
        &basis,
        &[observed_paint(partially_offscreen, 1)],
        transform([16, 16]),
    )
    .expect("a partially offscreen canonical row clips to the client viewport");
    let (visible, _, _) = indexed.into_parts();
    let clipped = visible.point_candidates(point(0, 0), 16).into_parts().0;
    assert_eq!(clipped.len(), 1);
    assert_eq!(
        clipped[0].clip_lineage().canonical(),
        partially_offscreen.clip_bounds()
    );
    assert_eq!(
        clipped[0].clip_lineage().realized(),
        observed_paint(partially_offscreen, 1).clip()
    );
    assert_eq!(
        visible
            .point_candidates(point(3, 7), 16)
            .into_parts()
            .0
            .len(),
        1
    );
    assert!(visible
        .point_candidates(point(4, 0), 16)
        .into_parts()
        .0
        .is_empty());
}

#[test]
fn sparse_1024_record_index_uses_bounded_point_probes() {
    let world = SpatialWorld::new();
    let paint = (0..1_024)
        .map(|index| {
            paint(
                &world,
                bounds((index * 2) as f32, 0.0, 1.0, 1.0),
                index,
                u8::MAX,
            )
        })
        .collect::<Vec<_>>();
    let observed = paint
        .iter()
        .map(|row| observed_paint(*row, row.layer_semantic_order()))
        .collect::<Vec<_>>();
    let basis =
        crate::mounting::UiMountedVisualRegionBasis::new(paint.into_boxed_slice(), Box::new([]));
    let indexed = validate_and_index(13, &basis, &observed, transform([2_048, 2]))
        .expect("the admitted sparse maximum validates and indexes");
    let (visible, _, cost) = indexed.into_parts();
    let (candidates, probes, exhausted) =
        visible.point_candidates(point(1_022, 0), 16).into_parts();
    assert_eq!(candidates.len(), 1);
    assert!(!exhausted);
    assert!(probes <= 24, "balanced sparse lookup took {probes} probes");
    assert_eq!(cost.region_records_examined(), 1_024);
}

#[test]
fn overlapping_point_candidates_stop_at_the_explicit_budget() {
    let world = SpatialWorld::new();
    let paint = (0..64)
        .map(|order| paint(&world, bounds(0.0, 0.0, 8.0, 8.0), order, u8::MAX))
        .collect::<Vec<_>>();
    let observed = paint
        .iter()
        .map(|row| observed_paint(*row, row.layer_semantic_order()))
        .collect::<Vec<_>>();
    let basis =
        crate::mounting::UiMountedVisualRegionBasis::new(paint.into_boxed_slice(), Box::new([]));
    let indexed = validate_and_index(17, &basis, &observed, transform([16, 16]))
        .expect("overlapping admitted rows validate and index");
    let (visible, _, _) = indexed.into_parts();

    let (candidates, probes, exhausted) = visible.point_candidates(point(4, 4), 7).into_parts();

    assert_eq!(candidates.len(), 7);
    assert!(exhausted);
    assert!(probes <= 15, "budgeted lookup took {probes} probes");
}

fn assert_first_visible_record(
    record: super::record::UiVisibleRegionRecord,
    world: &SpatialWorld,
    mechanic: worth_ui_host_contract::UiMountedFilledRectMechanic,
) {
    assert_eq!(record.node_receipt(), world.receipt);
    assert_eq!(record.layer_order(), 1);
    assert_eq!(record.paint_order(), 1);
    assert_eq!(record.opacity(), UiVisibleOpacity::Opaque);
    assert_eq!(record.clip_lineage().canonical(), mechanic.clip_bounds());
    assert_eq!(
        record.clip_lineage().realized(),
        observed_paint(mechanic, 1).clip()
    );
    assert_eq!(
        record.source_projection_digest(),
        mechanic.semantic_digest()
    );
}

impl SpatialWorld {
    fn new() -> Self {
        let frame = worth_ui_host_contract::UiMountedFrameIdentity::mint_unbound().unwrap();
        let instance = worth_ui_host_contract::UiMountedInstanceIdentity::mint_unbound().unwrap();
        let receipt = worth_ui_host_contract::UiMountedNodeReceiptIssuer::mint_for(frame)
            .unwrap()
            .receipt_for(instance);
        Self {
            frame,
            surface: worth_ui_host_contract::UiSemanticSurfaceIdentity::mint_unbound().unwrap(),
            binding: worth_ui_host_contract::UiSurfaceBindingGeneration::mint_unbound().unwrap(),
            instance,
            receipt,
        }
    }
}

fn paint(
    world: &SpatialWorld,
    bounds: worth_ui_host_contract::UiMountedCanonicalBox,
    order: u32,
    alpha: u8,
) -> worth_ui_host_contract::UiMountedFilledRectMechanic {
    worth_ui_host_contract::UiMountedFilledRectMechanic::complete_from_runtime_mounting(
        worth_ui_host_contract::UiMountedFilledRectCompletionInput {
            frame: world.frame,
            surface: world.surface,
            binding: world.binding,
            mounted_instance: world.instance,
            node_receipt: world.receipt,
            allocation_basis: worth_ui_host_contract::UiMountedAllocationBasis::new(
                1,
                1,
                1,
                worth_ui_host_contract::UiMountedTransformProjection::Identity,
            ),
            bounds,
            color: worth_ui_host_contract::UiMountedRgba8::new(1, 2, 3, alpha),
            layer_semantic_order: order,
            clip_bounds: bounds,
        },
    )
    .unwrap()
}

fn hit_test(
    world: &SpatialWorld,
    bounds: worth_ui_host_contract::UiMountedCanonicalBox,
    order: u32,
) -> worth_ui_host_contract::UiMountedHitTestMechanic {
    worth_ui_host_contract::UiMountedHitTestMechanic::complete_from_runtime_mounting(
        worth_ui_host_contract::UiMountedHitTestCompletionInput {
            frame: world.frame,
            surface: world.surface,
            binding: world.binding,
            mounted_instance: world.instance,
            node_receipt: world.receipt,
            bounds,
            clip_bounds: bounds,
            order: worth_ui_host_contract::UiMountedHitTestOrder::from_runtime_plan(order),
        },
    )
    .unwrap()
}

fn observed_paint(
    row: worth_ui_host_contract::UiMountedFilledRectMechanic,
    order: u32,
) -> worth_ui_host_contract::UiHostRealizedRegion {
    observed(
        row.node_receipt(),
        realized_geometry(row.bounds(), row.clip_bounds()),
        realized_ordering(
            order,
            worth_ui_host_contract::UiHostRealizedRegionParticipation::Paint,
        ),
    )
}

fn observed_hit(
    row: worth_ui_host_contract::UiMountedHitTestMechanic,
    order: u32,
) -> worth_ui_host_contract::UiHostRealizedRegion {
    observed(
        row.node_receipt(),
        realized_geometry(row.bounds(), row.clip_bounds()),
        realized_ordering(
            order,
            worth_ui_host_contract::UiHostRealizedRegionParticipation::HitTest,
        ),
    )
}

fn observed(
    receipt: worth_ui_host_contract::UiMountedNodeReceiptIdentity,
    geometry: worth_ui_host_contract::UiHostRealizedGeometry,
    ordering: worth_ui_host_contract::UiHostRealizedOrdering,
) -> worth_ui_host_contract::UiHostRealizedRegion {
    worth_ui_host_contract::UiHostRealizedRegion::observed_by_host(receipt, geometry, ordering)
}

fn realized_geometry(
    bounds: worth_ui_host_contract::UiMountedCanonicalBox,
    clip: worth_ui_host_contract::UiMountedCanonicalBox,
) -> worth_ui_host_contract::UiHostRealizedGeometry {
    worth_ui_host_contract::UiHostRealizedGeometry::observed_by_host(bounds, clip)
}

fn realized_ordering(
    order: u32,
    participation: worth_ui_host_contract::UiHostRealizedRegionParticipation,
) -> worth_ui_host_contract::UiHostRealizedOrdering {
    worth_ui_host_contract::UiHostRealizedOrdering::observed_by_host(order, participation)
}

fn bounds(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> worth_ui_host_contract::UiMountedCanonicalBox {
    worth_ui_host_contract::UiMountedCanonicalBox::canonicalize(
        worth_ui_host_contract::UiMountedCanonicalBoxInput {
            x,
            y,
            width,
            height,
            coordinate_space: worth_ui_host_contract::UiMountedCoordinateSpace::Viewport,
        },
    )
    .unwrap()
}

fn transform(dimensions: [u32; 2]) -> worth_ui_host_contract::UiHostCoordinateTransform {
    worth_ui_host_contract::UiHostCoordinateTransform::observed_by_host(
        worth_ui_host_contract::UiHostClientAreaObservation::observed_by_host([0, 0], dimensions),
        worth_ui_host_contract::UiHostViewportTransformObservation::observed_by_host(
            [dimensions[0] as f32, dimensions[1] as f32],
            [1.0, 1.0],
            [0.0, 0.0],
        ),
        worth_ui_host_contract::UiHostCoordinatePosture::observed_by_host(
            worth_ui_host_contract::UiHostCoordinateOrientation::TopLeftOrigin,
            worth_ui_host_contract::UiHostCoordinateRounding::FloorEdges,
        ),
    )
}

fn point(x: i64, y: i64) -> worth_ui_inspection::UiClientPhysicalPixel {
    worth_ui_inspection::UiClientPhysicalPixel::new(x, y).unwrap()
}
