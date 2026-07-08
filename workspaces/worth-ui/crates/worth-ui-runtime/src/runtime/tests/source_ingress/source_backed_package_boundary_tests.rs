use crate::capability::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, MeasurementConstraint, MeasurementValue, MosaicClippingPosture,
    MosaicFocusScopeKind, MosaicHitTestPosture, MosaicMeasurementAuthority,
    MosaicOverflowBehavior, MosaicParentGrowthBehavior, MosaicRegionKindDescriptor,
    MosaicRegionKindId, MosaicRegionPersistence, MosaicRegionRole, MosaicResizePermission,
    MosaicScrollOwnership, MosaicSizingBehavior, MosaicSizingContractDescriptor,
    MosaicSizingContractId, MosaicSizingKind, MosaicSizingPersistence,
    MosaicViewportConstraint, NamedMeasurementDefinition, NamedMeasurementToken,
    SurfacePlacementClass,
};
use crate::facade::WorthUi;
use crate::graph::UiGraphNodeIdentity;
use crate::runtime::tests::source_ingress_test_support::{empty_artifact, runtime_from_artifact};
use crate::runtime::{WorthUiSourceProvider, WorthUiWatcherEvent};

#[test]
fn file_source_ingress_derives_sealed_source_backed_package_without_helper_sidecar() {
    let support_app = support_app_with_sizing(source_backed_boundary_sizing());
    let submission = runtime_from_artifact(empty_artifact())
        .source_ingress(
            WorthUiSourceProvider::in_memory("source-backed-package-boundary").with_file(
                "app/source_backed_package_boundary.wui",
                "component workspace.component.source_backed_boundary { region workspace.region.primary { sizing workspace.sizing.source_backed_boundary; } }",
            ),
        )
        .start()
        .ingest([WorthUiWatcherEvent::provider_revision(
            "source-backed-package-boundary",
        )])
        .expect("source-backed provider should debounce")
        .lower_to_candidate_submission(support_app.capabilities())
        .expect("source-backed provider should lower through ingress");
    let (dsl_package, declaration_witness) = submission
        .source_backed_dsl_package()
        .cloned()
        .expect("file ingress should derive one sealed source-backed package")
        .into_parts();

    assert_eq!(dsl_package.admitted_declarations().len(), 1);
    assert!(
        declaration_witness
            .claims_for("app/source_backed_package_boundary.wui", 0)
            .is_some()
    );
}

#[test]
fn source_backed_membership_identity_uses_full_module_path_not_same_stem_heuristics() {
    let support_app = support_app_with_sizing(source_backed_boundary_sizing());
    let left = source_backed_submission(
        &support_app,
        "source-backed-left",
        "app/panels/editor.wui",
        "workspace.component.source_backed_boundary",
        "workspace.sizing.source_backed_boundary",
    );
    let right = source_backed_submission(
        &support_app,
        "source-backed-right",
        "app/dialogs/editor.wui",
        "workspace.component.source_backed_boundary",
        "workspace.sizing.source_backed_boundary",
    );
    let (_, left_witness) = left
        .source_backed_dsl_package()
        .cloned()
        .expect("left ingress should derive a sealed package")
        .into_parts();
    let (_, right_witness) = right
        .source_backed_dsl_package()
        .cloned()
        .expect("right ingress should derive a sealed package")
        .into_parts();
    let left_name = left_witness
        .claims_for("app/panels/editor.wui", 0)
        .expect("left claims should exist")
        .mosaic_membership_name();
    let right_name = right_witness
        .claims_for("app/dialogs/editor.wui", 0)
        .expect("right claims should exist")
        .mosaic_membership_name();

    assert_ne!(left_name, right_name);
}

#[test]
fn same_file_source_backed_declarations_do_not_collapse_into_one_mosaic_membership() {
    let support_app = WorthUi::app()
        .register_component(source_backed_boundary_component())
        .register_component(ComponentDescriptor::new(
            ComponentId::new("workspace.component.source_backed_boundary.peer").unwrap(),
            ComponentPropSchema::named("workspace.component.source_backed_boundary.peer.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .register_mosaic_region_kind(source_backed_boundary_region())
        .register_mosaic_sizing_contract(source_backed_boundary_sizing())
        .freeze();
    let submission = runtime_from_artifact(empty_artifact())
        .source_ingress(
            WorthUiSourceProvider::in_memory("source-backed-same-file").with_file(
                "app/source_backed_same_file.wui",
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
            ),
        )
        .start()
        .ingest([WorthUiWatcherEvent::provider_revision(
            "source-backed-same-file",
        )])
        .expect("same-file source-backed provider should debounce")
        .lower_to_candidate_submission(support_app.capabilities())
        .expect("same-file source-backed provider should lower through ingress");
    let source_backed_package = submission
        .source_backed_dsl_package()
        .cloned()
        .expect("same-file ingress should derive a sealed source-backed package");
    let (_, witness) = source_backed_package.clone().into_parts();
    let left_name = witness
        .claims_for("app/source_backed_same_file.wui", 0)
        .expect("left same-file claims should exist")
        .mosaic_membership_name()
        .to_owned();
    let right_name = witness
        .claims_for("app/source_backed_same_file.wui", 1)
        .expect("right same-file claims should exist")
        .mosaic_membership_name()
        .to_owned();

    assert_ne!(left_name, right_name);

    let app = WorthUi::app()
        .register_component(source_backed_boundary_component())
        .register_component(ComponentDescriptor::new(
            ComponentId::new("workspace.component.source_backed_boundary.peer").unwrap(),
            ComponentPropSchema::named("workspace.component.source_backed_boundary.peer.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .register_mosaic_region_kind(source_backed_boundary_region())
        .register_mosaic_sizing_contract(source_backed_boundary_sizing())
        .with_source_backed_dsl_package(source_backed_package)
        .freeze();

    assert_eq!(app.graph_snapshot().lookup().mosaic_members(&left_name).value().len(), 1);
    assert_eq!(
        app.graph_snapshot()
            .lookup()
            .mosaic_members(&right_name)
            .value()
            .len(),
        1
    );
}

#[test]
fn same_file_equivalent_declaration_reorder_preserves_membership_identity_set() {
    let support_app = WorthUi::app()
        .register_component(source_backed_boundary_component())
        .register_component(ComponentDescriptor::new(
            ComponentId::new("workspace.component.source_backed_boundary.peer").unwrap(),
            ComponentPropSchema::named("workspace.component.source_backed_boundary.peer.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .register_mosaic_region_kind(source_backed_boundary_region())
        .register_mosaic_sizing_contract(source_backed_boundary_sizing())
        .freeze();
    let first = runtime_from_artifact(empty_artifact())
        .source_ingress(
            WorthUiSourceProvider::in_memory("source-backed-reorder-a").with_file(
                "app/source_backed_reorder.wui",
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
            ),
        )
        .start()
        .ingest([WorthUiWatcherEvent::provider_revision(
            "source-backed-reorder-a",
        )])
        .expect("reorder-a provider should debounce")
        .lower_to_candidate_submission(support_app.capabilities())
        .expect("reorder-a provider should lower through ingress");
    let second = runtime_from_artifact(empty_artifact())
        .source_ingress(
            WorthUiSourceProvider::in_memory("source-backed-reorder-b").with_file(
                "app/source_backed_reorder.wui",
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
            ),
        )
        .start()
        .ingest([WorthUiWatcherEvent::provider_revision(
            "source-backed-reorder-b",
        )])
        .expect("reorder-b provider should debounce")
        .lower_to_candidate_submission(support_app.capabilities())
        .expect("reorder-b provider should lower through ingress");
    let (_, first_witness) = first
        .source_backed_dsl_package()
        .cloned()
        .expect("reorder-a ingress should derive a sealed package")
        .into_parts();
    let (_, second_witness) = second
        .source_backed_dsl_package()
        .cloned()
        .expect("reorder-b ingress should derive a sealed package")
        .into_parts();

    assert_eq!(
        first_witness.sorted_mosaic_membership_names(),
        second_witness.sorted_mosaic_membership_names()
    );
}

#[test]
fn unconstrained_source_backed_sizing_does_not_synthesize_bounded_measurement_posture() {
    let support_app = support_app_with_sizing(source_backed_unconstrained_sizing());
    let submission = source_backed_submission(
        &support_app,
        "source-backed-unconstrained",
        "app/source_backed_unconstrained.wui",
        "workspace.component.source_backed_boundary",
        "workspace.sizing.source_backed_unconstrained",
    );
    let source_backed_package = submission
        .source_backed_dsl_package()
        .cloned()
        .expect("unconstrained ingress should still derive a sealed package");
    let (_, witness) = source_backed_package.clone().into_parts();

    assert_eq!(
        witness
            .claims_for("app/source_backed_unconstrained.wui", 0)
            .expect("unconstrained claims should exist")
            .measurement_constraint_modifier(),
        None
    );

    let app = WorthUi::app()
        .register_component(source_backed_boundary_component())
        .register_mosaic_region_kind(source_backed_boundary_region())
        .register_mosaic_sizing_contract(source_backed_unconstrained_sizing())
        .with_source_backed_dsl_package(source_backed_package)
        .freeze();
    let node = graph_node_identity_for_provenance(&app, "app/source_backed_unconstrained.wui", 0);
    let graph_node = app
        .graph_snapshot()
        .nodes()
        .iter()
        .find(|candidate| candidate.graph_node_identity() == node)
        .expect("graph node should exist");

    assert_eq!(graph_node.measurement_constraint_modifier(), None);
}

fn source_backed_submission(
    support_app: &crate::facade::WorthUiApp,
    provider_revision: &str,
    module_path: &str,
    component_id: &str,
    sizing_contract_id: &str,
) -> crate::runtime::WorthUiWatchedCandidateSubmission {
    runtime_from_artifact(empty_artifact())
        .source_ingress(WorthUiSourceProvider::in_memory(provider_revision).with_file(
            module_path,
            format!(
                "component {component_id} {{ region workspace.region.primary {{ sizing {sizing_contract_id}; }} }}"
            ),
        ))
        .start()
        .ingest([WorthUiWatcherEvent::provider_revision(provider_revision)])
        .expect("source-backed provider should debounce")
        .lower_to_candidate_submission(support_app.capabilities())
        .expect("source-backed provider should lower through ingress")
}

fn support_app_with_sizing(
    sizing: MosaicSizingContractDescriptor,
) -> crate::facade::WorthUiApp {
    WorthUi::app()
        .register_component(source_backed_boundary_component())
        .register_mosaic_region_kind(source_backed_boundary_region())
        .register_mosaic_sizing_contract(sizing)
        .freeze()
}

fn source_backed_boundary_component() -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new("workspace.component.source_backed_boundary").unwrap(),
        ComponentPropSchema::named("workspace.component.source_backed_boundary.props"),
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

fn source_backed_unconstrained_sizing() -> MosaicSizingContractDescriptor {
    MosaicSizingContractDescriptor::new(
        MosaicSizingContractId::new("workspace.sizing.source_backed_unconstrained").unwrap(),
        MosaicSizingKind::fill(),
    )
    .with_measurement_authority(MosaicMeasurementAuthority::runtime_token())
    .with_resize_permission(MosaicResizePermission::user_resizable())
    .with_persistence(MosaicSizingPersistence::restorable())
    .with_overflow_behavior(MosaicOverflowBehavior::scroll_when_constrained())
    .with_parent_growth_behavior(MosaicParentGrowthBehavior::does_not_force_parent())
    .with_viewport_constraint(MosaicViewportConstraint::clamp_to_viewport())
    .with_named_measurement(NamedMeasurementDefinition::new(
        NamedMeasurementToken::new("workspace.measurement.source_backed_unconstrained").unwrap(),
        MeasurementValue::logical_pixels(320),
        MeasurementConstraint::unconstrained(),
    ))
}

fn graph_node_identity_for_provenance(
    app: &crate::facade::WorthUiApp,
    module_path: &str,
    declaration_index: usize,
) -> UiGraphNodeIdentity {
    let artifact = app
        .declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == module_path
                && provenance.declaration_index() == declaration_index
        })
        .expect("expected declaration artifact for requested provenance row");
    app.graph_snapshot()
        .lookup()
        .declaration_instances(artifact.identity())
        .value()
        .first()
        .copied()
        .expect("declaration should project one graph node")
}
