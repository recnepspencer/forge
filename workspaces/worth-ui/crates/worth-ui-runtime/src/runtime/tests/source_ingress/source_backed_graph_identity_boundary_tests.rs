use crate::capability::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, MeasurementConstraint, MeasurementValue, MosaicClippingPosture,
    MosaicFocusScopeKind, MosaicHitTestPosture, MosaicMeasurementAuthority, MosaicOverflowBehavior,
    MosaicParentGrowthBehavior, MosaicRegionKindDescriptor, MosaicRegionKindId,
    MosaicRegionPersistence, MosaicRegionRole, MosaicResizePermission, MosaicScrollOwnership,
    MosaicSizingBehavior, MosaicSizingContractDescriptor, MosaicSizingContractId, MosaicSizingKind,
    MosaicSizingPersistence, MosaicViewportConstraint, NamedMeasurementDefinition,
    NamedMeasurementToken, SurfacePlacementClass,
};
use crate::declaration::UiDeclarationArtifact;
use crate::facade::{WorthUi, WorthUiApp};
use crate::runtime::tests::source_ingress_test_support::{empty_artifact, runtime_from_artifact};
use crate::runtime::{WorthUiSourceProvider, WorthUiWatcherEvent};

#[test]
fn same_file_equivalent_declaration_reorder_preserves_graph_identity_set_on_ordinary_lane() {
    let first = freeze_source_backed_app(
        "source-backed-graph-reorder-a",
        r#"
component workspace.component.source_backed_boundary {
    region workspace.region.primary {
        sizing workspace.sizing.source_backed_boundary;
    }
}
component workspace.component.source_backed_boundary.peer {
    region workspace.region.primary {
        sizing workspace.sizing.source_backed_boundary;
    }
}
"#,
    );
    let second = freeze_source_backed_app(
        "source-backed-graph-reorder-b",
        r#"
component workspace.component.source_backed_boundary.peer {
    region workspace.region.primary {
        sizing workspace.sizing.source_backed_boundary;
    }
}
component workspace.component.source_backed_boundary {
    region workspace.region.primary {
        sizing workspace.sizing.source_backed_boundary;
    }
}
"#,
    );

    assert_eq!(
        source_backed_graph_identity_rows(&first),
        source_backed_graph_identity_rows(&second),
    );
    assert_eq!(
        source_backed_authored_provenance_digests(&first),
        source_backed_authored_provenance_digests(&second),
    );
}

fn freeze_source_backed_app(provider_revision: &str, source_text: &str) -> WorthUiApp {
    let support_app = WorthUi::app()
        .register_component(source_backed_boundary_component())
        .register_component(source_backed_boundary_peer_component())
        .register_mosaic_region_kind(source_backed_boundary_region())
        .register_mosaic_sizing_contract(source_backed_boundary_sizing())
        .freeze()
        .expect("application preparation should succeed");
    let submission = runtime_from_artifact(empty_artifact())
        .source_event_ingress(
            WorthUiSourceProvider::in_memory(provider_revision)
                .with_file("app/source_backed_graph_reorder.wui", source_text),
        )
        .start()
        .ingest([WorthUiWatcherEvent::provider_revision(provider_revision)])
        .expect("source-backed graph reorder provider should debounce")
        .lower_to_candidate_submission(support_app.capabilities())
        .expect("source-backed graph reorder provider should lower through ingress");
    WorthUi::app()
        .with_candidate_submission(submission)
        .register_component(source_backed_boundary_component())
        .register_component(source_backed_boundary_peer_component())
        .register_mosaic_region_kind(source_backed_boundary_region())
        .register_mosaic_sizing_contract(source_backed_boundary_sizing())
        .freeze()
        .expect("application preparation should succeed")
}

fn source_backed_graph_identity_rows(app: &WorthUiApp) -> Vec<(u64, u64, u64)> {
    let mut rows = source_backed_graph_rows(app)
        .into_iter()
        .map(
            |(
                declaration_identity_digest,
                graph_node_identity_digest,
                repeated_instance_basis_digest,
                _,
            )| {
                (
                    declaration_identity_digest,
                    graph_node_identity_digest,
                    repeated_instance_basis_digest,
                )
            },
        )
        .collect::<Vec<_>>();
    rows.sort_unstable();
    rows
}

fn source_backed_authored_provenance_digests(app: &WorthUiApp) -> Vec<u64> {
    let mut digests = source_backed_graph_rows(app)
        .into_iter()
        .map(|(_, _, _, authored_provenance_digest)| authored_provenance_digest)
        .collect::<Vec<_>>();
    digests.sort_unstable();
    digests
}

fn source_backed_graph_rows(app: &WorthUiApp) -> Vec<(u64, u64, u64, u64)> {
    let declaration_correspondence = app
        .graph_snapshot()
        .core_indexes()
        .declaration_correspondence();
    app.declaration_artifacts()
        .iter()
        .filter(|artifact: &&UiDeclarationArtifact| {
            artifact.provenance().source_provenance().module_path()
                == "app/source_backed_graph_reorder.wui"
        })
        .map(|artifact: &UiDeclarationArtifact| {
            let graph_node_identity = app
                .graph_snapshot()
                .lookup()
                .declaration_instances(artifact.identity())
                .value()
                .first()
                .copied()
                .expect("source-backed declaration should project one graph node");
            let graph_node = app
                .graph_snapshot()
                .lookup()
                .graph_node(graph_node_identity)
                .expect("source-backed graph node should exist")
                .value();
            let authored_provenance_digest = declaration_correspondence
                .authored_provenance_digest_for(graph_node_identity)
                .expect("graph node should retain authored provenance correspondence");

            (
                graph_node.declaration_identity().digest().raw(),
                graph_node.graph_node_identity().digest(),
                graph_node.repeated_instance_basis().identity_digest(),
                authored_provenance_digest,
            )
        })
        .collect::<Vec<_>>()
}

fn source_backed_boundary_component() -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new("workspace.component.source_backed_boundary").unwrap(),
        ComponentPropSchema::named("workspace.component.source_backed_boundary.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn source_backed_boundary_peer_component() -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new("workspace.component.source_backed_boundary.peer").unwrap(),
        ComponentPropSchema::named("workspace.component.source_backed_boundary.peer.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn source_backed_boundary_region() -> MosaicRegionKindDescriptor {
    MosaicRegionKindDescriptor::new(
        MosaicRegionKindId::new("workspace.region.primary").unwrap(),
        MosaicRegionRole::primary(),
    )
    .with_sizing_behavior(MosaicSizingBehavior::fills_available_space())
    .with_scroll_ownership(MosaicScrollOwnership::region_owned())
    .with_focus_scope(MosaicFocusScopeKind::active_surface_scope())
    .with_child_rule(crate::capability::MosaicChildRule::accepts_surfaces())
    .with_allowed_surface_class(SurfacePlacementClass::primary_region())
    .with_persistence(MosaicRegionPersistence::restorable())
    .with_clipping(MosaicClippingPosture::clip_to_region())
    .with_hit_test(MosaicHitTestPosture::participates())
}

fn source_backed_boundary_sizing() -> MosaicSizingContractDescriptor {
    MosaicSizingContractDescriptor::new(
        MosaicSizingContractId::new("workspace.sizing.source_backed_boundary").unwrap(),
        MosaicSizingKind::fill(),
    )
    .with_measurement_authority(MosaicMeasurementAuthority::runtime_token())
    .with_resize_permission(MosaicResizePermission::user_resizable())
    .with_persistence(MosaicSizingPersistence::restorable())
    .with_overflow_behavior(MosaicOverflowBehavior::scroll_when_constrained())
    .with_parent_growth_behavior(MosaicParentGrowthBehavior::does_not_force_parent())
    .with_viewport_constraint(MosaicViewportConstraint::clamp_to_viewport())
    .with_named_measurement(NamedMeasurementDefinition::new(
        NamedMeasurementToken::new("workspace.measurement.source_backed_boundary").unwrap(),
        MeasurementValue::logical_pixels(320),
        MeasurementConstraint::between(
            MeasurementValue::logical_pixels(200),
            MeasurementValue::logical_pixels(640),
        ),
    ))
}
