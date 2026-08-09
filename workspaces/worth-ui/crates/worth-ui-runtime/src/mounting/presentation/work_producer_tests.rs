use worth_ui_host_contract::{
    UiHostSurfaceIdentity, UiHostSurfacePresentationMode, UiMountedAllocationBasis,
    UiMountedCanonicalBox, UiMountedCanonicalBoxInput, UiMountedClipTable,
    UiMountedContentGeneration, UiMountedCoordinateSpace, UiMountedFilledRectCompletionInput,
    UiMountedFilledRectMechanic, UiMountedFilledRectTable, UiMountedFrameIdentity,
    UiMountedHitTestTable, UiMountedInstanceIdentity, UiMountedLayerTable,
    UiMountedNodeReceiptIssuer, UiMountedPaintBatchTable, UiMountedPaintCommandIdentity,
    UiMountedPresentationWorkView, UiMountedProjectionView, UiMountedProjectionViewInput,
    UiMountedResourceTable, UiMountedRgba8, UiMountedSemanticTextTable, UiMountedSpatialBatchTable,
    UiMountedSurfaceBindingRequirement, UiMountedTransformProjection, UiSemanticSurfaceIdentity,
    UiSurfaceBindingGeneration, WorthUiHostCapabilityObservationGeneration,
};

use super::work_producer::UiMountedPresentationState;

#[path = "work_producer_tests/rect_node.rs"]
mod rect_node;

use rect_node::rect_node;

struct MountedPresentationWorld {
    surface: UiSemanticSurfaceIdentity,
    binding: UiSurfaceBindingGeneration,
    content: UiMountedContentGeneration,
    first_instance: UiMountedInstanceIdentity,
    second_instance: UiMountedInstanceIdentity,
    requirement: UiMountedSurfaceBindingRequirement,
}

#[test]
fn equal_layer_total_order_follows_authored_node_order_not_command_identity() {
    let world = MountedPresentationWorld::new();
    let frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let projection = world.projection(
        frame,
        [
            rect_spec(world.second_instance, 0.0),
            rect_spec(world.first_instance, 0.0),
        ],
    );
    let state = UiMountedPresentationState::from_projection(&projection, world.requirement);
    let lease = super::UiMountedPresentationLeaseGate::default()
        .claim()
        .unwrap();

    let work = state.issue_initial(&lease, &projection);
    let UiMountedPresentationWorkView::Initial(initial) = work.view() else {
        panic!("first presentation must issue initial work");
    };
    let second = UiMountedPaintCommandIdentity::filled_rect(&projection.filled_rects().rows()[0]);
    let first = UiMountedPaintCommandIdentity::filled_rect(&projection.filled_rects().rows()[1]);
    assert!(
        first.mounted_instance().diagnostic_value() < second.mounted_instance().diagnostic_value(),
        "fixture identity allocation must oppose authored order"
    );
    assert_eq!(
        initial.order(),
        &[
            worth_ui_host_contract::UiMountedPaintOrderIdentity::for_command(second),
            worth_ui_host_contract::UiMountedPaintOrderIdentity::for_command(first),
        ]
    );
}

#[test]
fn unchanged_successor_carries_zero_command_order_and_damage_work() {
    let world = MountedPresentationWorld::new();
    let predecessor_frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let successor_frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let predecessor = world.projection(predecessor_frame, [rect_spec(world.first_instance, 0.0)]);
    let successor = world.projection(successor_frame, [rect_spec(world.first_instance, 0.0)]);
    let predecessor_state =
        UiMountedPresentationState::from_projection(&predecessor, world.requirement);
    let successor_state =
        UiMountedPresentationState::from_projection(&successor, world.requirement);
    let lease = super::UiMountedPresentationLeaseGate::default()
        .claim()
        .unwrap();

    let work = predecessor_state
        .issue_successor(&successor_state, &lease)
        .unwrap();
    let UiMountedPresentationWorkView::Unchanged(unchanged) = work.view() else {
        panic!("frame-only affinity progression must be unchanged work");
    };
    assert_eq!(unchanged.affinity().predecessor(), Some(predecessor_frame));
    assert_eq!(unchanged.affinity().successor(), successor_frame);
    assert_eq!(
        unchanged.affinity().baseline(),
        world.requirement.baseline()
    );
}

#[test]
fn one_replacement_carries_one_change_and_exact_predecessor_successor_damage() {
    let world = MountedPresentationWorld::new();
    let predecessor = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        [rect_spec(world.first_instance, 0.0)],
    );
    let mut changed = rect_spec(world.first_instance, 0.0);
    changed.color = UiMountedRgba8::new(242, 204, 96, 255);
    changed.x = 12.0;
    changed.clip_x = 12.0;
    let successor = world.projection(UiMountedFrameIdentity::mint_unbound().unwrap(), [changed]);
    let predecessor_state =
        UiMountedPresentationState::from_projection(&predecessor, world.requirement);
    let successor_state =
        UiMountedPresentationState::from_projection(&successor, world.requirement);
    let lease = super::UiMountedPresentationLeaseGate::default()
        .claim()
        .unwrap();

    let work = predecessor_state
        .issue_successor(&successor_state, &lease)
        .unwrap();
    let UiMountedPresentationWorkView::Delta(delta) = work.view() else {
        panic!("changed retained command must produce delta work");
    };
    assert_eq!(delta.changes().len(), 1);
    assert_eq!(delta.damage().len(), 2);
    assert!(delta
        .damage()
        .iter()
        .any(|damage| damage.bounds().x() == 0.0));
    assert!(delta
        .damage()
        .iter()
        .any(|damage| damage.bounds().x() == 12.0));
    assert!(delta.order_integrity().admits(&[
        worth_ui_host_contract::UiMountedPaintOrderIdentity::for_command(
            delta.changes()[0].identity(),
        ),
    ]));
}

#[test]
fn removal_and_insert_carry_exact_identities_vacated_damage_and_total_order() {
    let world = MountedPresentationWorld::new();
    let predecessor = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        [
            rect_spec(world.first_instance, 0.0),
            rect_spec(world.second_instance, 40.0),
        ],
    );
    let third = UiMountedInstanceIdentity::mint_unbound().unwrap();
    let successor = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        [
            rect_spec(world.second_instance, 40.0),
            rect_spec(third, 80.0),
        ],
    );
    let predecessor_state =
        UiMountedPresentationState::from_projection(&predecessor, world.requirement);
    let successor_state =
        UiMountedPresentationState::from_projection(&successor, world.requirement);
    let lease = super::UiMountedPresentationLeaseGate::default()
        .claim()
        .unwrap();

    let work = predecessor_state
        .issue_successor(&successor_state, &lease)
        .unwrap();
    let UiMountedPresentationWorkView::Delta(delta) = work.view() else {
        panic!("membership change must produce delta work");
    };
    assert_eq!(delta.changes().len(), 2);
    let removed = UiMountedPaintCommandIdentity::filled_rect(&predecessor.filled_rects().rows()[0]);
    let retained = UiMountedPaintCommandIdentity::filled_rect(&successor.filled_rects().rows()[0]);
    let inserted = UiMountedPaintCommandIdentity::filled_rect(&successor.filled_rects().rows()[1]);
    assert!(delta
        .changes()
        .iter()
        .any(|change| matches!(change, worth_ui_host_contract::UiMountedPaintCommandChange::Remove(identity) if *identity == removed)));
    assert!(delta
        .changes()
        .iter()
        .any(|change| matches!(change, worth_ui_host_contract::UiMountedPaintCommandChange::Insert(command) if command.identity() == inserted)));
    assert!(delta
        .damage()
        .iter()
        .any(|damage| damage.bounds().x() == 0.0));
    assert!(delta
        .damage()
        .iter()
        .any(|damage| damage.bounds().x() == 80.0));
    let removed_order = worth_ui_host_contract::UiMountedPaintOrderIdentity::for_command(removed);
    let retained_order = worth_ui_host_contract::UiMountedPaintOrderIdentity::for_command(retained);
    let inserted_order = worth_ui_host_contract::UiMountedPaintOrderIdentity::for_command(inserted);
    assert_eq!(
        delta.order(),
        &[
            worth_ui_host_contract::UiMountedPaintOrderEdit::remove(removed_order),
            worth_ui_host_contract::UiMountedPaintOrderEdit::place_after(retained_order, None),
            worth_ui_host_contract::UiMountedPaintOrderEdit::place_after(
                inserted_order,
                Some(retained_order),
            ),
        ]
    );
}

#[test]
fn replacement_damage_is_clipped_to_predecessor_and_successor_visibility() {
    let world = MountedPresentationWorld::new();
    let frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let predecessor = world.projection(
        frame,
        [rect_spec_with_clip(world.first_instance, 0.0, 4.0, 16.0)],
    );
    let mut successor_spec = rect_spec_with_clip(world.first_instance, 0.0, 8.0, 10.0);
    successor_spec.color = UiMountedRgba8::new(200, 20, 40, 255);
    let successor = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        [successor_spec],
    );
    let predecessor_state =
        UiMountedPresentationState::from_projection(&predecessor, world.requirement);
    let successor_state =
        UiMountedPresentationState::from_projection(&successor, world.requirement);
    let lease = super::UiMountedPresentationLeaseGate::default()
        .claim()
        .unwrap();

    let work = predecessor_state
        .issue_successor(&successor_state, &lease)
        .unwrap();
    let UiMountedPresentationWorkView::Delta(delta) = work.view() else {
        panic!("changed visible rectangle must produce delta work");
    };
    let exact_damage = delta
        .damage()
        .iter()
        .map(|damage| (damage.bounds().x(), damage.bounds().width()))
        .collect::<Vec<_>>();
    assert_eq!(exact_damage, vec![(4.0, 16.0), (8.0, 10.0)]);
}

#[derive(Clone, Copy)]
struct RectSpec {
    instance: UiMountedInstanceIdentity,
    x: f32,
    color: UiMountedRgba8,
    clip_x: f32,
    clip_width: f32,
}

fn rect_spec(instance: UiMountedInstanceIdentity, x: f32) -> RectSpec {
    RectSpec {
        instance,
        x,
        color: UiMountedRgba8::new(47, 129, 247, 255),
        clip_x: x,
        clip_width: 32.0,
    }
}

fn rect_spec_with_clip(
    instance: UiMountedInstanceIdentity,
    x: f32,
    clip_x: f32,
    clip_width: f32,
) -> RectSpec {
    RectSpec {
        instance,
        x,
        color: UiMountedRgba8::new(47, 129, 247, 255),
        clip_x,
        clip_width,
    }
}

impl MountedPresentationWorld {
    fn new() -> Self {
        let surface = UiSemanticSurfaceIdentity::mint_unbound().unwrap();
        let binding = UiSurfaceBindingGeneration::mint_unbound().unwrap();
        let generation = WorthUiHostCapabilityObservationGeneration::new(7);
        let requirement = UiMountedSurfaceBindingRequirement::new(
            surface,
            UiHostSurfaceIdentity::mint_unbound().unwrap(),
            binding,
            generation,
            11,
            UiHostSurfacePresentationMode::RecordOnly,
        );
        Self {
            surface,
            binding,
            content: UiMountedContentGeneration::mint_unbound().unwrap(),
            first_instance: UiMountedInstanceIdentity::mint_unbound().unwrap(),
            second_instance: UiMountedInstanceIdentity::mint_unbound().unwrap(),
            requirement,
        }
    }

    fn projection(
        &self,
        frame: UiMountedFrameIdentity,
        specs: impl IntoIterator<Item = RectSpec>,
    ) -> UiMountedProjectionView {
        let rows = specs
            .into_iter()
            .map(|spec| self.rect(frame, spec))
            .collect::<Vec<_>>();
        let nodes = rows
            .iter()
            .enumerate()
            .map(|(index, row)| rect_node(index, row))
            .collect();
        UiMountedProjectionView::new(UiMountedProjectionViewInput {
            frame,
            surface: self.surface,
            binding: self.binding,
            content_generation: self.content,
            nodes,
            clips: UiMountedClipTable::produced(Vec::new()),
            layers: UiMountedLayerTable::produced(Vec::new()),
            filled_rects: UiMountedFilledRectTable::from_runtime_mounting(rows).unwrap(),
            semantic_text: UiMountedSemanticTextTable::empty(),
            hit_tests: UiMountedHitTestTable::empty(),
            paint_batches: UiMountedPaintBatchTable::new(Vec::new()),
            spatial_batches: UiMountedSpatialBatchTable::new(Vec::new()),
            realtime_batches: worth_ui_host_contract::UiMountedRealtimeBatchTable::new(Vec::new()),
            resources: UiMountedResourceTable::new(Vec::new()),
        })
    }

    fn rect(&self, frame: UiMountedFrameIdentity, spec: RectSpec) -> UiMountedFilledRectMechanic {
        let bounds = canonical_box(spec.x, 0.0, 32.0, 24.0);
        UiMountedFilledRectMechanic::complete_from_runtime_mounting(
            UiMountedFilledRectCompletionInput {
                frame,
                surface: self.surface,
                binding: self.binding,
                mounted_instance: spec.instance,
                node_receipt: UiMountedNodeReceiptIssuer::mint_for(frame)
                    .unwrap()
                    .receipt_for(spec.instance),
                allocation_basis: UiMountedAllocationBasis::new(
                    1,
                    2,
                    3,
                    UiMountedTransformProjection::Identity,
                ),
                bounds,
                color: spec.color,
                layer_semantic_order: (spec.x as u32) / 40,
                clip_bounds: canonical_box(spec.clip_x, 0.0, spec.clip_width, 24.0),
            },
        )
        .unwrap()
    }
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

trait CommandChangeIdentity {
    fn identity(&self) -> worth_ui_host_contract::UiMountedPaintCommandIdentity;
}

impl CommandChangeIdentity for worth_ui_host_contract::UiMountedPaintCommandChange {
    fn identity(&self) -> worth_ui_host_contract::UiMountedPaintCommandIdentity {
        match self {
            Self::Insert(command) | Self::Replace(command) => command.identity(),
            Self::Remove(identity) => *identity,
        }
    }
}
