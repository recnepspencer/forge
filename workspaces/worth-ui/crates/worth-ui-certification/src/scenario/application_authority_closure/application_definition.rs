use worth_ui::facade::app::{WorthUi, WorthUiBuilder};
use worth_ui::facade::graph::UiGraphWorldProfile;
use worth_ui::facade::host::{WorthUiHeadlessHost, WorthUiOperationalHostAdapter};
use worth_ui::facade::query_binding::WorthUiQueryViewRegistration;
use worth_ui::facade::registry::{
    CommandDescriptor, CommandId, ComponentCanvasSpatialContract, ComponentChildPolicy,
    ComponentDescriptor, ComponentId, ComponentPropSchema, ComponentRealtimeOverlayContract,
    ComponentRealtimeOverlayPriority, ComponentStateOwnership, MeasurementConstraint,
    MeasurementValue, MosaicChildRule, MosaicClippingPosture, MosaicFocusScopeKind,
    MosaicHitTestPosture, MosaicMeasurementAuthority, MosaicOverflowBehavior,
    MosaicParentGrowthBehavior, MosaicRegionKindDescriptor, MosaicRegionKindId,
    MosaicRegionPersistence, MosaicRegionRole, MosaicResizePermission, MosaicScrollOwnership,
    MosaicSizingBehavior, MosaicSizingContractDescriptor, MosaicSizingContractId, MosaicSizingKind,
    MosaicSizingPersistence, MosaicStateOwnerIdentity, MosaicStatePersistencePolicy,
    MosaicStateReplacementRule, MosaicStateSlotDescriptor, MosaicStateSlotId, MosaicStateSlotKind,
    MosaicStateTruthPosture, MosaicViewportConstraint, NamedMeasurementDefinition,
    NamedMeasurementToken, SurfaceDescriptor, SurfaceId, SurfaceKind, SurfacePlacementClass,
    SurfaceStateClass, ThemeColorValue, ThemeTokenAlias, ThemeTokenDescriptor, ThemeTokenFamily,
    ThemeTokenId, ThemeTokenSource, ThemeTokenValue, ViewBindingId,
};
use worth_ui_query_binding::certification::WorthUiInstalledQueryTestFixture;

pub(crate) const CURRENT_COMPONENT: &str = "workspace.component.authority_current";
pub(crate) const CANDIDATE_COMPONENT: &str = "workspace.component.authority_candidate";
pub(crate) const IMPORTED_CURRENT_COMPONENT: &str =
    "workspace.component.authority_imported_current";
pub(crate) const IMPORTED_CANDIDATE_COMPONENT: &str =
    "workspace.component.authority_imported_candidate";
pub(crate) const REGION: &str = "workspace.region.authority_primary";
pub(crate) const SIZING: &str = "workspace.sizing.authority_primary";
pub(crate) const COMMAND: &str = "workspace.command.authority_save";
pub(crate) const SURFACE: &str = "workspace.surface.authority_main";
pub(crate) const STATE_SLOT: &str = "workspace.state.authority_scroll";
pub(crate) const TOKEN: &str = "theme.text.authority_default";
pub(crate) const QUERY_BINDING: &str = "inspector.measurements";
pub(crate) const CROSS_LANE_CANVAS: &str = "workspace.component.cross_lane_canvas";
pub(crate) const CROSS_LANE_REALTIME: &str = "workspace.component.cross_lane_realtime";

pub(crate) fn application_builder(query: &WorthUiInstalledQueryTestFixture) -> WorthUiBuilder {
    application_builder_with_host(query, WorthUiHeadlessHost)
}

pub(crate) fn application_builder_with_host<Host>(
    query: &WorthUiInstalledQueryTestFixture,
    host: Host,
) -> WorthUiBuilder
where
    Host: WorthUiOperationalHostAdapter + 'static,
{
    WorthUi::app()
        .with_host(host)
        .with_graph_world_profile(UiGraphWorldProfile::settled_query_view(
            ViewBindingId::new(QUERY_BINDING).expect("valid Query view binding id"),
            &query.installed_view(),
        ))
        .register_component(component(CURRENT_COMPONENT))
        .register_component(component(CANDIDATE_COMPONENT))
        .register_component(component(IMPORTED_CURRENT_COMPONENT))
        .register_component(component(IMPORTED_CANDIDATE_COMPONENT))
        .register_command(CommandDescriptor::new(
            CommandId::new(COMMAND).expect("valid scenario command id"),
            "Save",
        ))
        .register_surface(
            SurfaceDescriptor::new(
                SurfaceId::new(SURFACE).expect("valid scenario surface id"),
                SurfaceKind::primary_content(),
                ComponentId::new(CURRENT_COMPONENT).expect("valid scenario component id"),
                SurfacePlacementClass::primary_region(),
                SurfaceStateClass::restorable(),
            )
            .with_command_slot(CommandId::new(COMMAND).expect("valid scenario command id")),
        )
        .register_theme_token(ThemeTokenDescriptor::define(
            ThemeTokenId::new("theme.text.authority_primary").expect("valid primary token id"),
            ThemeTokenFamily::text(),
            ThemeTokenSource::application(),
            ThemeTokenValue::color(ThemeColorValue::hex("#101820").expect("valid theme color")),
        ))
        .register_theme_token(ThemeTokenDescriptor::alias(
            ThemeTokenId::new(TOKEN).expect("valid scenario token id"),
            ThemeTokenFamily::text(),
            ThemeTokenSource::application(),
            ThemeTokenAlias::to(
                ThemeTokenId::new("theme.text.authority_primary").expect("valid primary token id"),
            ),
        ))
        .register_mosaic_region_kind(region())
        .register_mosaic_sizing_contract(sizing())
        .register_mosaic_state_slot(state_slot())
        .register_query_view(WorthUiQueryViewRegistration::new(query.installed_view()))
        .expect("installed Query view should register through the production builder")
}

pub(crate) fn cross_lane_application_builder_with_host<Host>(
    query: &WorthUiInstalledQueryTestFixture,
    host: Host,
) -> WorthUiBuilder
where
    Host: WorthUiOperationalHostAdapter + 'static,
{
    application_builder_with_host(query, host)
        .register_component(
            component(CROSS_LANE_CANVAS).with_canvas_spatial_contract(
                ComponentCanvasSpatialContract::new(64, 2, 1)
                    .expect("cross-lane spatial contract is bounded"),
            ),
        )
        .register_component(
            component(CROSS_LANE_REALTIME).with_realtime_overlay_contract(
                ComponentRealtimeOverlayContract::new(
                    2,
                    1,
                    16,
                    ComponentRealtimeOverlayPriority::HudOverlay,
                )
                .expect("cross-lane realtime contract fits its frame budget"),
            ),
        )
}

pub(crate) fn scaled_canvas_application_builder_with_host<Host>(
    query: &WorthUiInstalledQueryTestFixture,
    host: Host,
    canvas_count: usize,
) -> WorthUiBuilder
where
    Host: WorthUiOperationalHostAdapter + 'static,
{
    let mut builder = application_builder_with_host(query, host);
    for index in 0..canvas_count {
        let identity = format!("workspace.component.scaled_canvas_{index:04}");
        builder = builder.register_component(
            component(&identity).with_canvas_spatial_contract(
                ComponentCanvasSpatialContract::new(64, 2, 1)
                    .expect("scaled spatial contract is bounded"),
            ),
        );
    }
    builder
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

fn state_slot() -> MosaicStateSlotDescriptor {
    MosaicStateSlotDescriptor::new(
        MosaicStateSlotId::new(STATE_SLOT).expect("valid scenario state slot id"),
        MosaicStateSlotKind::scroll_position(),
    )
    .with_owner_identity(MosaicStateOwnerIdentity::mosaic_region_kind(
        MosaicRegionKindId::new(REGION).expect("valid scenario region id"),
    ))
    .with_persistence_policy(MosaicStatePersistencePolicy::restore_across_hot_reload())
    .with_replacement_rule(MosaicStateReplacementRule::preserve_when_owner_matches())
    .with_truth_posture(MosaicStateTruthPosture::ui_runtime_state())
}
