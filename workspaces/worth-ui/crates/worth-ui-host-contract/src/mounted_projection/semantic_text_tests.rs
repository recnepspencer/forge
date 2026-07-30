use std::sync::Arc;

use super::{
    UiMountedCollectionRowCorrelation, UiMountedSemanticTextCompletionDenial,
    UiMountedSemanticTextCompletionInput, UiMountedSemanticTextMechanic,
    UiMountedSemanticTextTable, UiMountedSemanticTextTableDenial, UiSemanticTextProfile,
    UiSemanticTextSlot,
};
use crate::{
    UiMountedAllocationBasis, UiMountedCanonicalBox, UiMountedCanonicalBoxInput,
    UiMountedContentGeneration, UiMountedCoordinateSpace, UiMountedFrameIdentity,
    UiMountedInstanceIdentity, UiMountedNodeReceiptIssuer, UiMountedRgba8,
    UiMountedTransformProjection, UiSemanticSurfaceIdentity, UiSurfaceBindingGeneration,
    WorthUiHostCapabilityObservationGeneration,
};

#[test]
fn completion_preserves_runtime_owned_semantic_text_meaning() {
    let input = fixture();
    let row = complete(input.clone());

    assert_eq!(row.content_generation(), input.content_generation);
    assert_eq!(row.frame(), input.frame);
    assert_eq!(row.surface(), input.surface);
    assert_eq!(row.binding(), input.binding);
    assert_eq!(row.mounted_instance(), input.mounted_instance);
    assert_eq!(row.node_receipt(), input.node_receipt);
    assert_eq!(row.allocation_basis(), input.allocation_basis);
    assert_eq!(row.bounds(), input.bounds);
    assert_eq!(row.clip_bounds(), input.clip_bounds);
    assert_eq!((row.origin_x(), row.origin_y()), (32.0, 40.0));
    assert_eq!(row.text(), "ONLINE");
    assert_eq!(row.slot(), UiSemanticTextSlot::Value);
    assert_eq!(row.profile(), UiSemanticTextProfile::BodyDefault);
    assert_eq!(row.capability_generation(), input.capability_generation);
    assert_eq!(
        row.capability_profile_digest(),
        input.capability_profile_digest
    );
}

#[test]
fn geometry_origin_and_receipt_mismatches_are_typed_denials() {
    let mut input = fixture();
    input.bounds = canonical_box(32.0, 32.0, 0.0, 96.0);
    input.clip_bounds = input.bounds;
    assert_denial(
        input,
        UiMountedSemanticTextCompletionDenial::NonAreaGeometry,
    );

    let mut input = fixture();
    input.clip_bounds = canonical_box(32.0, 32.0, 159.0, 96.0);
    assert_denial(input, UiMountedSemanticTextCompletionDenial::ClipMismatch);

    for (origin_x, origin_y) in [(f32::NAN, 40.0), (31.0, 40.0), (32.0, 129.0)] {
        let mut input = fixture();
        input.origin_x = origin_x;
        input.origin_y = origin_y;
        assert_denial(
            input,
            UiMountedSemanticTextCompletionDenial::InvalidTextOrigin,
        );
    }

    let mut input = fixture();
    let foreign_frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    input.node_receipt = UiMountedNodeReceiptIssuer::mint_for(foreign_frame)
        .unwrap()
        .receipt_for(input.mounted_instance);
    assert_denial(
        input,
        UiMountedSemanticTextCompletionDenial::NodeReceiptFrameMismatch,
    );

    let mut input = fixture();
    input.node_receipt = UiMountedNodeReceiptIssuer::mint_for(input.frame)
        .unwrap()
        .receipt_for(UiMountedInstanceIdentity::mint_unbound().unwrap());
    assert_denial(
        input,
        UiMountedSemanticTextCompletionDenial::NodeReceiptInstanceMismatch,
    );
}

#[test]
fn digest_changes_with_content_context_and_placement() {
    let baseline = fixture();
    let digest = complete(baseline.clone()).semantic_digest();
    let mut variants = Vec::new();
    variants.push(with(&baseline, |input| input.text = Arc::from("UPDATED")));
    variants.push(with(&baseline, |input| input.origin_y = 41.0));
    variants.push(with(&baseline, |input| {
        input.slot = UiSemanticTextSlot::Posture
    }));
    variants.push(with(&baseline, |input| input.layer_semantic_order = 8));
    variants.push(with(&baseline, |input| {
        input.capability_generation = WorthUiHostCapabilityObservationGeneration::new(8)
    }));
    variants.push(with(&baseline, |input| {
        input.capability_profile_digest = 10
    }));
    variants.push(with(&baseline, |input| {
        input.content_generation = UiMountedContentGeneration::mint_unbound().unwrap()
    }));

    for variant in variants {
        assert_ne!(complete(variant).semantic_digest(), digest);
    }
}

#[test]
fn collection_slot_and_row_correlation_are_atomic() {
    let mut missing_identity = fixture();
    missing_identity.slot = UiSemanticTextSlot::CollectionValue {
        selected_field_ordinal: 0,
    };
    assert_denial(
        missing_identity,
        UiMountedSemanticTextCompletionDenial::CollectionIdentityMismatch,
    );

    let mut scalar_with_identity = fixture();
    scalar_with_identity.collection_row = Some(
        UiMountedCollectionRowCorrelation::from_runtime_mounting(Arc::from("row-a")),
    );
    assert_denial(
        scalar_with_identity,
        UiMountedSemanticTextCompletionDenial::CollectionIdentityMismatch,
    );

    let mut collection = fixture();
    collection.slot = UiSemanticTextSlot::CollectionValue {
        selected_field_ordinal: 0,
    };
    collection.collection_row = Some(UiMountedCollectionRowCorrelation::from_runtime_mounting(
        Arc::from("row-a"),
    ));
    let row = complete(collection);
    assert_eq!(
        row.collection_row().unwrap().identity_for_reporting(),
        "row-a"
    );
}

#[test]
fn row_and_table_byte_caps_are_enforced() {
    let mut input = fixture();
    input.text = Arc::from("x".repeat(UiMountedSemanticTextMechanic::MAX_CONTENT_BYTES + 1));
    assert_denial(
        input,
        UiMountedSemanticTextCompletionDenial::ContentCapacityExceeded,
    );

    let mut row_input = fixture();
    row_input.text = Arc::from("x".repeat(UiMountedSemanticTextMechanic::MAX_CONTENT_BYTES));
    let row = complete(row_input);
    let rows = vec![row; UiMountedSemanticTextTable::MAX_BYTES / 4_096 + 1];
    assert_eq!(
        UiMountedSemanticTextTable::from_runtime_mounting(rows),
        Err(UiMountedSemanticTextTableDenial::ByteCapacityExceeded)
    );
}

fn fixture() -> UiMountedSemanticTextCompletionInput {
    let frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let mounted_instance = UiMountedInstanceIdentity::mint_unbound().unwrap();
    let bounds = canonical_box(32.0, 32.0, 160.0, 96.0);
    UiMountedSemanticTextCompletionInput {
        content_generation: UiMountedContentGeneration::mint_unbound().unwrap(),
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
        clip_bounds: bounds,
        origin_x: 32.0,
        origin_y: 40.0,
        text: Arc::from("ONLINE"),
        slot: UiSemanticTextSlot::Value,
        collection_row: None,
        color: UiMountedRgba8::new(255, 255, 255, 255),
        profile: UiSemanticTextProfile::BodyDefault,
        layer_semantic_order: 7,
        capability_generation: WorthUiHostCapabilityObservationGeneration::new(7),
        capability_profile_digest: 9,
    }
}

fn complete(input: UiMountedSemanticTextCompletionInput) -> UiMountedSemanticTextMechanic {
    UiMountedSemanticTextMechanic::complete_from_runtime_mounting(input).unwrap()
}

fn assert_denial(
    input: UiMountedSemanticTextCompletionInput,
    expected: UiMountedSemanticTextCompletionDenial,
) {
    assert_eq!(
        UiMountedSemanticTextMechanic::complete_from_runtime_mounting(input),
        Err(expected)
    );
}

fn with(
    input: &UiMountedSemanticTextCompletionInput,
    mutate: impl FnOnce(&mut UiMountedSemanticTextCompletionInput),
) -> UiMountedSemanticTextCompletionInput {
    let mut input = input.clone();
    mutate(&mut input);
    input
}

fn canonical_box(x: f32, y: f32, width: f32, height: f32) -> UiMountedCanonicalBox {
    UiMountedCanonicalBox::canonicalize(UiMountedCanonicalBoxInput {
        x,
        y,
        width,
        height,
        coordinate_space: UiMountedCoordinateSpace::Viewport,
    })
    .unwrap()
}
