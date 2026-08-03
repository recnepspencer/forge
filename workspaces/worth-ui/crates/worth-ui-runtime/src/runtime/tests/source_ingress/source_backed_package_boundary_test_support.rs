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
use crate::facade::WorthUi;
use crate::graph::UiGraphNodeIdentity;
use crate::runtime::tests::source_ingress_test_support::{empty_artifact, runtime_from_artifact};
use crate::runtime::{WorthUiSourceProvider, WorthUiWatcherEvent};

pub(super) fn source_backed_submission(
    support_app: &crate::facade::WorthUiApp,
    provider_revision: &str,
    module_path: &str,
    component_id: &str,
    sizing_contract_id: &str,
) -> crate::runtime::WorthUiWatchedCandidateSubmission {
    runtime_from_artifact(empty_artifact())
        .source_event_ingress(WorthUiSourceProvider::in_memory(provider_revision).with_file(
            module_path,
            format!(
                "component {component_id} {{ region workspace.region.primary {{ sizing {sizing_contract_id}; }} }}"
            ),
        ))
        .start()
        .ingest([WorthUiWatcherEvent::provider_revision(provider_revision)])
        .expect("source-backed provider should debounce")
        .attempt_candidate_for_certification(support_app.capabilities())
        .expect("source-backed provider should lower through ingress")
}

pub(super) fn support_app_with_sizing(
    sizing: MosaicSizingContractDescriptor,
) -> crate::facade::WorthUiApp {
    WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .register_component(source_backed_boundary_component())
        .register_mosaic_region_kind(source_backed_boundary_region())
        .register_mosaic_sizing_contract(sizing)
        .freeze()
        .expect("application preparation should succeed")
}

pub(super) fn prepare_source_backed_submission(
    submission: crate::runtime::WorthUiWatchedCandidateSubmission,
    sizing: MosaicSizingContractDescriptor,
) -> crate::facade::WorthUiApp {
    WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .with_candidate_submission(submission)
        .register_component(source_backed_boundary_component())
        .register_mosaic_region_kind(source_backed_boundary_region())
        .register_mosaic_sizing_contract(sizing)
        .freeze()
        .expect("complete source-backed composition should prepare")
}

pub(super) fn two_component_source_backed_builder(
) -> crate::facade::entry::WorthUiApplicationBuilder {
    WorthUi::app()
        .with_change_profile(crate::runtime::rebind::UiChangeProfile::platform_pulse())
        .register_component(source_backed_boundary_component())
        .register_component(ComponentDescriptor::new(
            ComponentId::new("workspace.component.source_backed_boundary.peer").unwrap(),
            ComponentPropSchema::named("workspace.component.source_backed_boundary.peer.props"),
            ComponentChildPolicy::no_children(),
            ComponentStateOwnership::runtime_owned(),
        ))
        .register_mosaic_region_kind(source_backed_boundary_region())
        .register_mosaic_sizing_contract(source_backed_boundary_sizing())
}

pub(super) fn sorted_mosaic_membership_names(app: &crate::facade::WorthUiApp) -> Vec<String> {
    let mut names = app
        .declaration_artifacts()
        .iter()
        .filter_map(|artifact| {
            let node_identity = app
                .graph_snapshot()
                .lookup()
                .declaration_instances(artifact.identity())
                .value()
                .first()
                .copied()?;
            let topology = app
                .graph_snapshot()
                .lookup()
                .topology_node(node_identity)?
                .value();
            topology
                .mosaic_membership()
                .map(|membership| membership.mosaic_name().to_owned())
        })
        .collect::<Vec<_>>();
    names.sort();
    names
}

pub(super) fn mosaic_membership_name_for_provenance(
    app: &crate::facade::WorthUiApp,
    module_path: &str,
    declaration_index: usize,
) -> String {
    let identity = graph_node_identity_for_provenance(app, module_path, declaration_index);
    app.graph_snapshot()
        .lookup()
        .topology_node(identity)
        .expect("source-backed graph topology should exist")
        .value()
        .mosaic_membership()
        .expect("source-backed declaration should retain mosaic membership")
        .mosaic_name()
        .to_owned()
}

pub(super) fn source_backed_boundary_component() -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new("workspace.component.source_backed_boundary").unwrap(),
        ComponentPropSchema::named("workspace.component.source_backed_boundary.props"),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

pub(super) fn source_backed_boundary_region() -> MosaicRegionKindDescriptor {
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

pub(super) fn source_backed_boundary_sizing() -> MosaicSizingContractDescriptor {
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

pub(super) fn source_backed_unconstrained_sizing() -> MosaicSizingContractDescriptor {
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

pub(super) fn graph_node_identity_for_provenance(
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
