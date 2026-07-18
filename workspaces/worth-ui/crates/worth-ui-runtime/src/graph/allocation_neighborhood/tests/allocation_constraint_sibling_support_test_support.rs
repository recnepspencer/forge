use crate::capability::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, MeasurementConstraint, MeasurementValue, MosaicClippingPosture,
    MosaicFocusScopeKind, MosaicHitTestPosture, MosaicMeasurementAuthority, MosaicOverflowBehavior,
    MosaicParentGrowthBehavior, MosaicRegionPersistence, MosaicResizePermission,
    MosaicScrollOwnership, MosaicSizingContractDescriptor, MosaicSizingContractId,
    MosaicSizingKind, MosaicSizingPersistence, MosaicViewportConstraint,
    NamedMeasurementDefinition, NamedMeasurementToken, SurfacePlacementClass,
};
use crate::facade::{WorthUi, WorthUiApp};
use crate::graph::UiGraphNodeIdentity;
use crate::runtime::tests::source_ingress_test_support::{empty_artifact, runtime_from_artifact};
use crate::runtime::{WorthUiSourceProvider, WorthUiWatcherEvent};

pub(super) fn mosaic_peer_app(world_profile: crate::graph::UiGraphWorldProfile) -> WorthUiApp {
    mosaic_peer_app_with_contracts(
        world_profile,
        "worth-ui.runtime.graph.allocation-constraint-sibling-support",
        [
            mosaic_runtime_sizing_contract_id().as_str(),
            mosaic_runtime_sizing_contract_id().as_str(),
            mosaic_runtime_sizing_contract_id().as_str(),
        ],
        false,
    )
}

pub(super) fn mosaic_peer_app_with_contracts(
    world_profile: crate::graph::UiGraphWorldProfile,
    package_name: &str,
    sizing_contract_ids: [&str; 3],
    include_alternate_contract: bool,
) -> WorthUiApp {
    let source_provider = source_backed_mosaic_source_provider(package_name, sizing_contract_ids);
    let support_app = mosaic_peer_builder(
        crate::graph::UiGraphWorldProfile::authoritative(),
        include_alternate_contract,
    )
    .freeze()
    .expect("application preparation should succeed");
    let submission = runtime_from_artifact(empty_artifact())
        .source_ingress(source_provider)
        .start()
        .ingest([WorthUiWatcherEvent::provider_revision(package_name)])
        .expect("source-backed mosaic provider should debounce to one candidate batch")
        .lower_to_candidate_submission(support_app.capabilities())
        .expect("source-backed mosaic candidate should lower through source ingress");
    mosaic_peer_builder(world_profile, include_alternate_contract)
        .with_candidate_submission(submission)
        .freeze()
        .expect("application preparation should succeed")
}

fn mosaic_peer_builder(
    world_profile: crate::graph::UiGraphWorldProfile,
    include_alternate_contract: bool,
) -> crate::facade::entry::WorthUiBuilder {
    let mut builder = WorthUi::app()
        .with_graph_world_profile(world_profile)
        .register_component(component_descriptor("workspace.component.workflow_editor"))
        .register_component(component_descriptor(
            "workspace.component.workflow_editor.peer_a",
        ))
        .register_component(component_descriptor(
            "workspace.component.workflow_editor.peer_b",
        ))
        .register_mosaic_region_kind(primary_region())
        .register_mosaic_sizing_contract(mosaic_runtime_sizing());
    if include_alternate_contract {
        builder = builder.register_mosaic_sizing_contract(mosaic_alternate_runtime_sizing());
    }
    builder
}

pub(super) fn graph_node_identity_for_provenance(
    app: &WorthUiApp,
    declaration_index: usize,
) -> UiGraphNodeIdentity {
    let artifact = app
        .declaration_artifacts()
        .iter()
        .find(|artifact| {
            let provenance = artifact.provenance().source_provenance();
            provenance.module_path() == "app/allocation_constraint_sibling_support_tests.wui"
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

pub(super) fn component_descriptor(id: &str) -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new(id).unwrap(),
        ComponentPropSchema::named(format!("{id}.props")),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

pub(super) fn primary_region() -> crate::capability::MosaicRegionKindDescriptor {
    crate::capability::MosaicRegionKindDescriptor::new(
        crate::capability::MosaicRegionKindId::new("workspace.region.primary").unwrap(),
        crate::capability::MosaicRegionRole::primary(),
    )
    .with_sizing_behavior(crate::capability::MosaicSizingBehavior::fills_available_space())
    .with_scroll_ownership(MosaicScrollOwnership::region_owned())
    .with_focus_scope(MosaicFocusScopeKind::active_surface_scope())
    .with_child_rule(crate::capability::MosaicChildRule::accepts_surfaces())
    .with_allowed_surface_class(SurfacePlacementClass::primary_region())
    .with_persistence(MosaicRegionPersistence::restorable())
    .with_clipping(MosaicClippingPosture::clip_to_region())
    .with_hit_test(MosaicHitTestPosture::participates())
}

pub(super) fn mosaic_runtime_sizing() -> MosaicSizingContractDescriptor {
    MosaicSizingContractDescriptor::new(
        mosaic_runtime_sizing_contract_id(),
        MosaicSizingKind::fill(),
    )
    .with_measurement_authority(MosaicMeasurementAuthority::runtime_token())
    .with_resize_permission(MosaicResizePermission::user_resizable())
    .with_persistence(MosaicSizingPersistence::restorable())
    .with_overflow_behavior(MosaicOverflowBehavior::scroll_when_constrained())
    .with_parent_growth_behavior(MosaicParentGrowthBehavior::does_not_force_parent())
    .with_viewport_constraint(MosaicViewportConstraint::clamp_to_viewport())
    .with_named_measurement(NamedMeasurementDefinition::new(
        NamedMeasurementToken::new("workspace.measurement.mosaic_support").unwrap(),
        MeasurementValue::logical_pixels(320),
        MeasurementConstraint::between(
            MeasurementValue::logical_pixels(200),
            MeasurementValue::logical_pixels(640),
        ),
    ))
}

pub(super) fn mosaic_runtime_sizing_contract_id() -> MosaicSizingContractId {
    MosaicSizingContractId::new("workspace.sizing.mosaic_support").unwrap()
}

pub(super) fn mosaic_alternate_runtime_sizing() -> MosaicSizingContractDescriptor {
    MosaicSizingContractDescriptor::new(
        mosaic_alternate_runtime_sizing_contract_id(),
        MosaicSizingKind::fill(),
    )
    .with_measurement_authority(MosaicMeasurementAuthority::runtime_token())
    .with_resize_permission(MosaicResizePermission::user_resizable())
    .with_persistence(MosaicSizingPersistence::restorable())
    .with_overflow_behavior(MosaicOverflowBehavior::scroll_when_constrained())
    .with_parent_growth_behavior(MosaicParentGrowthBehavior::does_not_force_parent())
    .with_viewport_constraint(MosaicViewportConstraint::clamp_to_viewport())
    .with_named_measurement(NamedMeasurementDefinition::new(
        NamedMeasurementToken::new("workspace.measurement.mosaic_support.alternate").unwrap(),
        MeasurementValue::logical_pixels(360),
        MeasurementConstraint::between(
            MeasurementValue::logical_pixels(240),
            MeasurementValue::logical_pixels(720),
        ),
    ))
}

pub(super) fn mosaic_alternate_runtime_sizing_contract_id() -> MosaicSizingContractId {
    MosaicSizingContractId::new("workspace.sizing.mosaic_support.alternate").unwrap()
}

fn source_backed_mosaic_source_provider(
    package_name: &str,
    sizing_contract_ids: [&str; 3],
) -> WorthUiSourceProvider {
    WorthUiSourceProvider::in_memory(package_name).with_file(
        "app/allocation_constraint_sibling_support_tests.wui",
        mosaic_source_text(sizing_contract_ids),
    )
}

fn mosaic_source_text(sizing_contract_ids: [&str; 3]) -> String {
    format!(
        r#"
component workspace.component.workflow_editor {{
    region workspace.region.primary {{
        sizing {};
    }}
}}
component workspace.component.workflow_editor.peer_a {{
    region workspace.region.primary {{
        sizing {};
    }}
}}
component workspace.component.workflow_editor.peer_b {{
    region workspace.region.primary {{
        sizing {};
    }}
}}
"#,
        sizing_contract_ids[0], sizing_contract_ids[1], sizing_contract_ids[2]
    )
}
