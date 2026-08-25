use super::{
    UiMountedFilledRectCompletionDenial, UiMountedFilledRectCompletionInput,
    UiMountedFilledRectMechanic, UiMountedFilledRectTable, UiMountedFilledRectTableDenial,
    UiMountedRgba8,
};
use crate::{
    UiMountedAllocationBasis, UiMountedCanonicalBox, UiMountedCanonicalBoxInput,
    UiMountedCoordinateSpace, UiMountedFrameIdentity, UiMountedInstanceIdentity,
    UiMountedNodeReceiptIssuer, UiMountedTransformProjection, UiSemanticSurfaceIdentity,
    UiSurfaceBindingGeneration,
};

#[test]
fn complete_rectangle_preserves_every_runtime_owned_field() {
    let input = fixture();
    let row = complete(input);

    assert_eq!(row.frame(), input.frame);
    assert_eq!(row.surface(), input.surface);
    assert_eq!(row.binding(), input.binding);
    assert_eq!(row.mounted_instance(), input.mounted_instance);
    assert_eq!(row.node_receipt(), input.node_receipt);
    assert_eq!(row.allocation_basis(), input.allocation_basis);
    assert_eq!(row.bounds(), input.bounds);
    assert_eq!(row.color(), UiMountedRgba8::new(47, 129, 247, 255));
    assert_eq!(row.layer_semantic_order(), 0);
    assert_eq!(row.clip_bounds(), input.bounds);
}

#[test]
fn non_area_clip_and_receipt_mismatches_are_typed_denials() {
    let mut input = fixture();
    input.bounds = canonical_box(32.0, 32.0, 0.0, 96.0);
    input.clip_bounds = input.bounds;
    assert_eq!(
        UiMountedFilledRectMechanic::complete_from_runtime_mounting(input),
        Err(UiMountedFilledRectCompletionDenial::NonAreaGeometry)
    );

    let mut input = fixture();
    input.clip_bounds = UiMountedCanonicalBox::canonicalize(UiMountedCanonicalBoxInput {
        x: 32.0,
        y: 32.0,
        width: 159.0,
        height: 96.0,
        coordinate_space: UiMountedCoordinateSpace::Window,
    })
    .unwrap();
    assert_eq!(
        UiMountedFilledRectMechanic::complete_from_runtime_mounting(input),
        Err(UiMountedFilledRectCompletionDenial::ClipMismatch)
    );

    let mut input = fixture();
    let foreign_frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    input.node_receipt = UiMountedNodeReceiptIssuer::mint_for(foreign_frame)
        .unwrap()
        .receipt_for(input.mounted_instance);
    assert_eq!(
        UiMountedFilledRectMechanic::complete_from_runtime_mounting(input),
        Err(UiMountedFilledRectCompletionDenial::NodeReceiptFrameMismatch)
    );

    let mut input = fixture();
    let foreign_instance = UiMountedInstanceIdentity::mint_unbound().unwrap();
    input.node_receipt = UiMountedNodeReceiptIssuer::mint_for(input.frame)
        .unwrap()
        .receipt_for(foreign_instance);
    assert_eq!(
        UiMountedFilledRectMechanic::complete_from_runtime_mounting(input),
        Err(UiMountedFilledRectCompletionDenial::NodeReceiptInstanceMismatch)
    );
}

#[test]
fn equal_counts_do_not_alias_changed_static_paint_meaning() {
    let baseline = fixture();
    let baseline_digest = complete(baseline).semantic_digest();
    let variants = [
        with_bounds(baseline, canonical_box(33.0, 32.0, 160.0, 96.0)),
        with_color(baseline, UiMountedRgba8::new(48, 129, 247, 255)),
        with_instance(baseline, UiMountedInstanceIdentity::mint_unbound().unwrap()),
        with_layer(baseline, 1),
        with_binding(
            baseline,
            UiSurfaceBindingGeneration::mint_unbound().unwrap(),
        ),
        with_surface(baseline, UiSemanticSurfaceIdentity::mint_unbound().unwrap()),
        with_allocation_basis(
            baseline,
            UiMountedAllocationBasis::new(1, 3, 3, UiMountedTransformProjection::Identity),
        ),
        with_clip(baseline, canonical_box(32.0, 33.0, 160.0, 95.0)),
        with_frame(baseline, UiMountedFrameIdentity::mint_unbound().unwrap()),
    ];

    for variant in variants {
        assert_ne!(complete(variant).semantic_digest(), baseline_digest);
    }
}

#[test]
fn retained_paint_equivalence_excludes_lineage_but_not_paint_inputs() {
    let baseline = fixture();
    let baseline_row = complete(baseline);
    assert!(
        baseline_row.same_retained_paint_meaning(complete(with_frame(
            baseline,
            UiMountedFrameIdentity::mint_unbound().unwrap(),
        )))
    );

    let variants = [
        with_bounds(baseline, canonical_box(33.0, 32.0, 160.0, 96.0)),
        with_color(baseline, UiMountedRgba8::new(48, 129, 247, 255)),
        with_instance(baseline, UiMountedInstanceIdentity::mint_unbound().unwrap()),
        with_layer(baseline, 1),
        with_binding(
            baseline,
            UiSurfaceBindingGeneration::mint_unbound().unwrap(),
        ),
        with_surface(baseline, UiSemanticSurfaceIdentity::mint_unbound().unwrap()),
        with_allocation_basis(
            baseline,
            UiMountedAllocationBasis::new(1, 3, 3, UiMountedTransformProjection::Identity),
        ),
        with_clip(baseline, canonical_box(32.0, 33.0, 160.0, 95.0)),
    ];
    for variant in variants {
        assert!(!baseline_row.same_retained_paint_meaning(complete(variant)));
    }
}

#[test]
fn complete_table_has_an_enforced_capacity() {
    let row = complete(fixture());
    let rows = vec![row; UiMountedFilledRectTable::MAX_ROWS + 1];
    assert_eq!(
        UiMountedFilledRectTable::from_runtime_mounting(rows),
        Err(UiMountedFilledRectTableDenial::CapacityExceeded)
    );
}

fn fixture() -> UiMountedFilledRectCompletionInput {
    let frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let mounted_instance = UiMountedInstanceIdentity::mint_unbound().unwrap();
    let bounds = canonical_box(32.0, 32.0, 160.0, 96.0);
    UiMountedFilledRectCompletionInput {
        frame,
        surface: UiSemanticSurfaceIdentity::mint_unbound().unwrap(),
        binding: UiSurfaceBindingGeneration::mint_unbound().unwrap(),
        mounted_instance,
        node_receipt: UiMountedNodeReceiptIssuer::mint_for(frame)
            .unwrap()
            .receipt_for(mounted_instance),
        allocation_basis: UiMountedAllocationBasis::new(
            1,
            2,
            3,
            UiMountedTransformProjection::Identity,
        ),
        bounds,
        color: UiMountedRgba8::new(47, 129, 247, 255),
        layer_semantic_order: 0,
        clip_bounds: bounds,
    }
}

fn complete(input: UiMountedFilledRectCompletionInput) -> UiMountedFilledRectMechanic {
    UiMountedFilledRectMechanic::complete_from_runtime_mounting(input).unwrap()
}

fn canonical_box(x: f32, y: f32, width: f32, height: f32) -> UiMountedCanonicalBox {
    UiMountedCanonicalBox::canonicalize(UiMountedCanonicalBoxInput {
        x,
        y,
        width,
        height,
        coordinate_space: UiMountedCoordinateSpace::HostSurface,
    })
    .unwrap()
}

fn with_bounds(
    mut input: UiMountedFilledRectCompletionInput,
    bounds: UiMountedCanonicalBox,
) -> UiMountedFilledRectCompletionInput {
    input.bounds = bounds;
    input.clip_bounds = bounds;
    input
}

fn with_color(
    mut input: UiMountedFilledRectCompletionInput,
    color: UiMountedRgba8,
) -> UiMountedFilledRectCompletionInput {
    input.color = color;
    input
}

fn with_instance(
    mut input: UiMountedFilledRectCompletionInput,
    mounted_instance: UiMountedInstanceIdentity,
) -> UiMountedFilledRectCompletionInput {
    input.mounted_instance = mounted_instance;
    input.node_receipt = UiMountedNodeReceiptIssuer::mint_for(input.frame)
        .unwrap()
        .receipt_for(mounted_instance);
    input
}

fn with_layer(
    mut input: UiMountedFilledRectCompletionInput,
    layer: u32,
) -> UiMountedFilledRectCompletionInput {
    input.layer_semantic_order = layer;
    input
}

fn with_binding(
    mut input: UiMountedFilledRectCompletionInput,
    binding: UiSurfaceBindingGeneration,
) -> UiMountedFilledRectCompletionInput {
    input.binding = binding;
    input
}

fn with_surface(
    mut input: UiMountedFilledRectCompletionInput,
    surface: UiSemanticSurfaceIdentity,
) -> UiMountedFilledRectCompletionInput {
    input.surface = surface;
    input
}

fn with_allocation_basis(
    mut input: UiMountedFilledRectCompletionInput,
    allocation_basis: UiMountedAllocationBasis,
) -> UiMountedFilledRectCompletionInput {
    input.allocation_basis = allocation_basis;
    input
}

fn with_clip(
    mut input: UiMountedFilledRectCompletionInput,
    clip_bounds: UiMountedCanonicalBox,
) -> UiMountedFilledRectCompletionInput {
    input.clip_bounds = clip_bounds;
    input
}

fn with_frame(
    mut input: UiMountedFilledRectCompletionInput,
    frame: UiMountedFrameIdentity,
) -> UiMountedFilledRectCompletionInput {
    input.frame = frame;
    input.node_receipt = UiMountedNodeReceiptIssuer::mint_for(frame)
        .unwrap()
        .receipt_for(input.mounted_instance);
    input
}
