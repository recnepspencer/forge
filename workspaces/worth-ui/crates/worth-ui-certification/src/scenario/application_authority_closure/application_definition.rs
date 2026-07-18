use worth_ui::facade::app::{WorthUi, WorthUiBuilder};
use worth_ui::facade::graph::UiGraphWorldProfile;
use worth_ui::facade::host::{WorthUiHeadlessHost, WorthUiOperationalHostAdapter};
use worth_ui::facade::query_binding::WorthUiQueryViewRegistration;
use worth_ui::facade::registry::{
    ComponentChildPolicy, ComponentDescriptor, ComponentId, ComponentPropSchema,
    ComponentStateOwnership, MeasurementConstraint, MeasurementValue, MosaicChildRule,
    MosaicClippingPosture, MosaicFocusScopeKind, MosaicHitTestPosture, MosaicMeasurementAuthority,
    MosaicOverflowBehavior, MosaicParentGrowthBehavior, MosaicRegionKindDescriptor,
    MosaicRegionKindId, MosaicRegionPersistence, MosaicRegionRole, MosaicResizePermission,
    MosaicScrollOwnership, MosaicSizingBehavior, MosaicSizingContractDescriptor,
    MosaicSizingContractId, MosaicSizingKind, MosaicSizingPersistence, MosaicViewportConstraint,
    NamedMeasurementDefinition, NamedMeasurementToken, SurfacePlacementClass,
};
use worth_ui_query_binding::certification::{
    worth_ui_query_snapshot_prerequisites, WorthUiInstalledQueryTestFixture,
};

pub(super) const CURRENT_COMPONENT: &str = "workspace.component.authority_current";
pub(super) const CANDIDATE_COMPONENT: &str = "workspace.component.authority_candidate";
pub(super) const REGION: &str = "workspace.region.authority_primary";
pub(super) const SIZING: &str = "workspace.sizing.authority_primary";

pub(super) fn application_builder(query: &WorthUiInstalledQueryTestFixture) -> WorthUiBuilder {
    application_builder_with_host(query, WorthUiHeadlessHost)
}

pub(super) fn application_builder_with_host<Host>(
    query: &WorthUiInstalledQueryTestFixture,
    host: Host,
) -> WorthUiBuilder
where
    Host: WorthUiOperationalHostAdapter + 'static,
{
    WorthUi::app()
        .with_host(host)
        .with_graph_world_profile(UiGraphWorldProfile::query_snapshot_basis(
            worth_ui_query_snapshot_prerequisites(
                "authority-closure",
                ["worth-ui.phase14", "application", "authority-closure"],
            ),
        ))
        .register_component(component(CURRENT_COMPONENT))
        .register_component(component(CANDIDATE_COMPONENT))
        .register_mosaic_region_kind(region())
        .register_mosaic_sizing_contract(sizing())
        .register_query_view(WorthUiQueryViewRegistration::new(query.installed_view()))
        .expect("installed Query view should register through the production builder")
}

pub(super) fn application_builder_with_capability_drift(
    query: &WorthUiInstalledQueryTestFixture,
) -> WorthUiBuilder {
    application_builder(query)
        .register_component(component("workspace.component.authority_capability_drift"))
}

fn component(id: &str) -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new(id).expect("valid scenario component id"),
        ComponentPropSchema::named(format!("{id}.props")),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn region() -> MosaicRegionKindDescriptor {
    MosaicRegionKindDescriptor::new(
        MosaicRegionKindId::new(REGION).expect("valid scenario region id"),
        MosaicRegionRole::primary(),
    )
    .with_sizing_behavior(MosaicSizingBehavior::fills_available_space())
    .with_scroll_ownership(MosaicScrollOwnership::region_owned())
    .with_focus_scope(MosaicFocusScopeKind::active_surface_scope())
    .with_child_rule(MosaicChildRule::accepts_surfaces())
    .with_allowed_surface_class(SurfacePlacementClass::primary_region())
    .with_persistence(MosaicRegionPersistence::restorable())
    .with_clipping(MosaicClippingPosture::clip_to_region())
    .with_hit_test(MosaicHitTestPosture::participates())
}

fn sizing() -> MosaicSizingContractDescriptor {
    MosaicSizingContractDescriptor::new(
        MosaicSizingContractId::new(SIZING).expect("valid scenario sizing id"),
        MosaicSizingKind::fill(),
    )
    .with_measurement_authority(MosaicMeasurementAuthority::runtime_token())
    .with_resize_permission(MosaicResizePermission::user_resizable())
    .with_persistence(MosaicSizingPersistence::restorable())
    .with_overflow_behavior(MosaicOverflowBehavior::scroll_when_constrained())
    .with_parent_growth_behavior(MosaicParentGrowthBehavior::does_not_force_parent())
    .with_viewport_constraint(MosaicViewportConstraint::clamp_to_viewport())
    .with_named_measurement(NamedMeasurementDefinition::new(
        NamedMeasurementToken::new("workspace.measurement.authority_primary")
            .expect("valid scenario measurement token"),
        MeasurementValue::logical_pixels(320),
        MeasurementConstraint::between(
            MeasurementValue::logical_pixels(200),
            MeasurementValue::logical_pixels(640),
        ),
    ))
}
