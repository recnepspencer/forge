use worth_ui_host_contract::{
    UiHostSurfaceIdentity, UiHostSurfacePresentationMode, UiMountedAllocationBasis,
    UiMountedCanonicalBox, UiMountedCanonicalBoxInput, UiMountedClipTable,
    UiMountedContentGeneration, UiMountedCoordinateSpace, UiMountedFilledRectCompletionInput,
    UiMountedFilledRectMechanic, UiMountedFilledRectReference, UiMountedFilledRectTable,
    UiMountedFrameIdentity, UiMountedHitTestProjection, UiMountedHitTestTable,
    UiMountedInstanceIdentity, UiMountedLayerTable, UiMountedLogicalDamage,
    UiMountedMechanicalRole, UiMountedNodeProjectionView, UiMountedNodeProjectionViewInput,
    UiMountedNodeReceiptIssuer, UiMountedOmissionReason, UiMountedPaintBatchTable,
    UiMountedPaintCommand, UiMountedPaintCommandChange, UiMountedPaintOrderEdit,
    UiMountedPaintOrderIdentity, UiMountedPaintOrderIntegrity, UiMountedPaintProjection,
    UiMountedParticipation, UiMountedParticipationFact, UiMountedParticipationInput,
    UiMountedParticipationStatus, UiMountedPresentationDelta, UiMountedPresentationDeltaInput,
    UiMountedPresentationInitial, UiMountedPresentationInitialInput,
    UiMountedPresentationUnchanged, UiMountedPresentationUnchangedInput, UiMountedProjectionView,
    UiMountedProjectionViewInput, UiMountedRealtimeBatchTable, UiMountedResourceTable,
    UiMountedRgba8, UiMountedSemanticTextTable, UiMountedSpatialBatchTable,
    UiMountedSurfaceBindingRequirement, UiMountedTransformProjection, UiSemanticSurfaceIdentity,
    UiSurfaceBindingGeneration, WorthUiHostCapabilityObservationGeneration,
};

use super::{UiNativeRetainedDrawList, UiNativeRetainedDrawListDenial};

#[test]
fn exact_delta_updates_draw_order_damage_and_replay_without_retained_scans() {
    let world = DrawListWorld::new();
    let initial_frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let initial_rows = [
        world.rect(
            initial_frame,
            world.first,
            0.0,
            UiMountedRgba8::new(20, 30, 40, 255),
        ),
        world.rect(
            initial_frame,
            world.second,
            80.0,
            UiMountedRgba8::new(50, 60, 70, 255),
        ),
    ];
    let initial = world.initial(initial_frame, initial_rows.clone());
    let mut retained = UiNativeRetainedDrawList::initial(&initial).unwrap();
    let successor_frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let replaced = world.rect(
        successor_frame,
        world.first,
        0.0,
        UiMountedRgba8::new(90, 100, 110, 255),
    );
    let inserted = world.rect(
        successor_frame,
        world.third,
        160.0,
        UiMountedRgba8::new(120, 130, 140, 255),
    );
    let replaced_command = command(replaced);
    let inserted_command = command(inserted);
    let removed_identity = UiMountedPaintOrderIdentity::for_command(
        UiMountedPaintCommand::identity(&command(initial_rows[1])),
    );
    let retained_identity = UiMountedPaintOrderIdentity::for_command(replaced_command.identity());
    let inserted_identity = UiMountedPaintOrderIdentity::for_command(inserted_command.identity());
    let damage = [
        initial_rows[0].bounds(),
        replaced.bounds(),
        initial_rows[1].bounds(),
        inserted.bounds(),
    ]
    .map(UiMountedLogicalDamage::from_runtime_mounting);
    let delta = UiMountedPresentationDelta::from_inert_mechanics(UiMountedPresentationDeltaInput {
        predecessor: initial_frame,
        successor: successor_frame,
        surface: world.surface,
        binding: world.binding,
        content: world.content,
        baseline: world.requirement.baseline(),
        changes: vec![
            UiMountedPaintCommandChange::Replace(replaced_command),
            UiMountedPaintCommandChange::Remove(removed_identity.command()),
            UiMountedPaintCommandChange::Insert(inserted_command),
        ],
        order: vec![
            UiMountedPaintOrderEdit::remove(removed_identity),
            UiMountedPaintOrderEdit::place_after(inserted_identity, Some(retained_identity)),
        ],
        order_integrity: UiMountedPaintOrderIntegrity::for_order(&[
            retained_identity,
            inserted_identity,
        ]),
        damage: damage.to_vec(),
        auxiliary: None,
        production_cost: Default::default(),
    });

    let (staged, undo) = retained.stage_delta(&delta).unwrap();
    assert_eq!(
        retained.order.ordered().collect::<Vec<_>>(),
        vec![retained_identity, inserted_identity]
    );
    retained.rollback_delta(undo).unwrap();
    assert_eq!(
        retained.order.ordered().collect::<Vec<_>>(),
        vec![retained_identity, removed_identity]
    );
    assert_eq!(
        retained.command(removed_identity.command()),
        Some(&initial.commands()[1])
    );
    let plan = retained.apply_delta(&delta).unwrap();
    assert_eq!(plan, staged);
    assert_eq!(plan.baseline_rgba8, [0, 0, 0, 0]);
    assert_eq!(plan.clear_regions.len(), 3);
    assert_eq!(
        plan.replay.as_ref(),
        &[retained_identity.command(), inserted_identity.command()]
    );
    assert_eq!(plan.counters.draw_mutations, 3);
    assert_eq!(plan.counters.order_mutations, 2);
    assert_eq!(plan.counters.damage_regions, 4);
    assert_eq!(plan.counters.retained_command_scans, 0);
    assert!(retained.command(removed_identity.command()).is_none());
}

#[test]
fn unchanged_advances_exact_affinity_without_draw_order_or_damage_work() {
    let world = DrawListWorld::new();
    let frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let initial = world.initial(
        frame,
        [world.rect(frame, world.first, 0.0, UiMountedRgba8::new(1, 2, 3, 255))],
    );
    let identity = initial.commands()[0].identity();
    let mut retained = UiNativeRetainedDrawList::initial(&initial).unwrap();
    let successor = UiMountedFrameIdentity::mint_unbound().unwrap();
    let unchanged =
        UiMountedPresentationUnchanged::from_inert_mechanics(UiMountedPresentationUnchangedInput {
            predecessor: frame,
            successor,
            surface: world.surface,
            binding: world.binding,
            content: world.content,
            baseline: world.requirement.baseline(),
            production_cost: Default::default(),
        });
    retained.apply_unchanged(&unchanged).unwrap();
    assert_eq!(retained.frame, successor);
    assert_eq!(retained.command(identity), Some(&initial.commands()[0]));
    assert_eq!(retained.order.ordered().count(), 1);
}

#[test]
fn stale_delta_denies_without_mutating_retained_commands() {
    let world = DrawListWorld::new();
    let frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let mechanic = world.rect(frame, world.first, 0.0, UiMountedRgba8::new(1, 2, 3, 255));
    let initial = world.initial(frame, [mechanic]);
    let identity = initial.commands()[0].identity();
    let mut retained = UiNativeRetainedDrawList::initial(&initial).unwrap();
    let delta = UiMountedPresentationDelta::from_inert_mechanics(UiMountedPresentationDeltaInput {
        predecessor: UiMountedFrameIdentity::mint_unbound().unwrap(),
        successor: UiMountedFrameIdentity::mint_unbound().unwrap(),
        surface: world.surface,
        binding: world.binding,
        content: world.content,
        baseline: world.requirement.baseline(),
        changes: Vec::new(),
        order: Vec::new(),
        order_integrity: initial.order_integrity(),
        damage: Vec::new(),
        auxiliary: None,
        production_cost: Default::default(),
    });
    assert!(matches!(
        retained.apply_delta(&delta),
        Err(UiNativeRetainedDrawListDenial::AffinityMismatch)
    ));
    assert_eq!(retained.command(identity), Some(&initial.commands()[0]));
}

struct DrawListWorld {
    surface: UiSemanticSurfaceIdentity,
    binding: UiSurfaceBindingGeneration,
    content: UiMountedContentGeneration,
    first: UiMountedInstanceIdentity,
    second: UiMountedInstanceIdentity,
    third: UiMountedInstanceIdentity,
    requirement: UiMountedSurfaceBindingRequirement,
}

impl DrawListWorld {
    fn new() -> Self {
        let surface = UiSemanticSurfaceIdentity::mint_unbound().unwrap();
        let binding = UiSurfaceBindingGeneration::mint_unbound().unwrap();
        let requirement = UiMountedSurfaceBindingRequirement::new(
            surface,
            UiHostSurfaceIdentity::mint_unbound().unwrap(),
            binding,
            WorthUiHostCapabilityObservationGeneration::new(7),
            11,
            UiHostSurfacePresentationMode::NativeDisplay,
        );
        Self {
            surface,
            binding,
            content: UiMountedContentGeneration::mint_unbound().unwrap(),
            first: UiMountedInstanceIdentity::mint_unbound().unwrap(),
            second: UiMountedInstanceIdentity::mint_unbound().unwrap(),
            third: UiMountedInstanceIdentity::mint_unbound().unwrap(),
            requirement,
        }
    }

    fn rect(
        &self,
        frame: UiMountedFrameIdentity,
        instance: UiMountedInstanceIdentity,
        x: f32,
        color: UiMountedRgba8,
    ) -> UiMountedFilledRectMechanic {
        let bounds = canonical_box(x, 0.0, 32.0, 24.0);
        UiMountedFilledRectMechanic::complete_from_runtime_mounting(
            UiMountedFilledRectCompletionInput {
                frame,
                surface: self.surface,
                binding: self.binding,
                mounted_instance: instance,
                node_receipt: UiMountedNodeReceiptIssuer::mint_for(frame)
                    .unwrap()
                    .receipt_for(instance),
                allocation_basis: UiMountedAllocationBasis::new(
                    1,
                    2,
                    3,
                    UiMountedTransformProjection::Identity,
                ),
                bounds,
                color,
                layer_semantic_order: 0,
                clip_bounds: bounds,
            },
        )
        .unwrap()
    }

    fn initial<const N: usize>(
        &self,
        frame: UiMountedFrameIdentity,
        rows: [UiMountedFilledRectMechanic; N],
    ) -> UiMountedPresentationInitial {
        let commands = rows
            .iter()
            .map(|mechanic| command(*mechanic))
            .collect::<Vec<_>>();
        let order = commands
            .iter()
            .map(|command| UiMountedPaintOrderIdentity::for_command(command.identity()))
            .collect::<Vec<_>>();
        let projection = projection(self, frame, rows.to_vec());
        UiMountedPresentationInitial::from_inert_mechanics(UiMountedPresentationInitialInput {
            successor: frame,
            surface: self.surface,
            binding: self.binding,
            content: self.content,
            baseline: self.requirement.baseline(),
            projection,
            commands,
            order_integrity: UiMountedPaintOrderIntegrity::for_order(&order),
            order,
            damage: rows
                .iter()
                .map(|row| UiMountedLogicalDamage::from_runtime_mounting(row.bounds()))
                .collect(),
            production_cost: Default::default(),
        })
    }
}

fn command(mechanic: UiMountedFilledRectMechanic) -> UiMountedPaintCommand {
    UiMountedPaintCommand::FilledRect {
        identity: worth_ui_host_contract::UiMountedPaintCommandIdentity::filled_rect(&mechanic),
        mechanic,
    }
}

fn projection(
    world: &DrawListWorld,
    frame: UiMountedFrameIdentity,
    rows: Vec<UiMountedFilledRectMechanic>,
) -> UiMountedProjectionView {
    let nodes = rows
        .iter()
        .enumerate()
        .map(|(index, row)| rect_node(index, row))
        .collect();
    let authored_paint_commands = rows
        .iter()
        .copied()
        .map(command)
        .collect::<Vec<_>>();
    let mut authored_paint_order = authored_paint_commands
        .iter()
        .enumerate()
        .map(|(ordinal, command)| {
            (
                command.layer_semantic_order(),
                ordinal,
                UiMountedPaintOrderIdentity::for_command(command.identity()),
            )
        })
        .collect::<Vec<_>>();
    authored_paint_order.sort_by_key(|source| (source.0, source.1));
    let authored_paint_order = authored_paint_order
        .into_iter()
        .map(|source| source.2)
        .collect();
    UiMountedProjectionView::new(UiMountedProjectionViewInput {
        frame,
        surface: world.surface,
        binding: world.binding,
        content_generation: world.content,
        nodes,
        clips: UiMountedClipTable::produced(Vec::new()),
        layers: UiMountedLayerTable::produced(Vec::new()),
        filled_rects: UiMountedFilledRectTable::from_runtime_mounting(rows).unwrap(),
        semantic_text: UiMountedSemanticTextTable::empty(),
        hit_tests: UiMountedHitTestTable::empty(),
        paint_batches: UiMountedPaintBatchTable::new(Vec::new()),
        spatial_batches: UiMountedSpatialBatchTable::new(Vec::new()),
        realtime_batches: UiMountedRealtimeBatchTable::new(Vec::new()),
        resources: UiMountedResourceTable::new(Vec::new()),
        authored_paint_commands,
        authored_paint_order,
    })
}

fn rect_node(index: usize, row: &UiMountedFilledRectMechanic) -> UiMountedNodeProjectionView {
    let admitted = UiMountedParticipationFact::new(UiMountedParticipationStatus::Admitted);
    let withheld = UiMountedParticipationFact::new(UiMountedParticipationStatus::Withheld);
    let omitted = UiMountedOmissionReason::NotDefinedByCurrentRuntime;
    let reference = UiMountedFilledRectReference::from_runtime_mounting(index as u16);
    UiMountedNodeProjectionView::new(UiMountedNodeProjectionViewInput {
        mounted_instance: row.mounted_instance(),
        node_receipt: row.node_receipt(),
        role: UiMountedMechanicalRole::Control,
        participation: UiMountedParticipation::new(UiMountedParticipationInput {
            paint: admitted,
            clip: admitted,
            input: withheld,
            focus: withheld,
            hit_test: withheld,
            accessibility: withheld,
            motion: withheld,
            diagnostic: withheld,
        }),
        allocation: worth_ui_host_contract::UiMountedAllocationProjection::Known {
            bounds: row.bounds(),
            basis: row.allocation_basis(),
        },
        preview: worth_ui_host_contract::UiMountedPreviewProjection::Omitted(omitted),
        paint: UiMountedPaintProjection::FilledRect(reference),
        hit_test: UiMountedHitTestProjection::Omitted(omitted),
        accessibility: worth_ui_host_contract::UiMountedAccessibilityProjection::Omitted(omitted),
        motion: worth_ui_host_contract::UiMountedMotionProjection::Omitted(omitted),
        diagnostic: worth_ui_host_contract::UiMountedDiagnosticProjection::Omitted(omitted),
        drawables: vec![worth_ui_host_contract::UiMountedDrawableReference::FilledRect(reference)],
        semantic_text: Vec::new(),
    })
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
