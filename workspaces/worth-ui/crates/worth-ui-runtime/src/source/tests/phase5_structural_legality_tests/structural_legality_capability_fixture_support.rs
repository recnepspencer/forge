use crate::capability::{
    CapabilitySupportCatalog, ComponentChildPolicy, ComponentDescriptor, ComponentId,
    ComponentPropSchema, ComponentStateOwnership, MeasurementConstraint, MeasurementValue,
    MosaicChildRule, MosaicClippingPosture, MosaicFocusScopeKind, MosaicHitTestPosture,
    MosaicMeasurementAuthority, MosaicOverflowBehavior, MosaicParentGrowthBehavior,
    MosaicPlacementAction, MosaicPlacementConflictBehavior, MosaicPlacementPersistence,
    MosaicPlacementPolicyDescriptor, MosaicPlacementPolicyId, MosaicPlacementReloadReconciliation,
    MosaicPlacementSource, MosaicPlacementSupport, MosaicPlacementTarget,
    MosaicRegionKindDescriptor, MosaicRegionKindId, MosaicRegionPersistence, MosaicRegionRole,
    MosaicResizePermission, MosaicScrollOwnership, MosaicSizingBehavior,
    MosaicSizingContractDescriptor, MosaicSizingContractId, MosaicSizingKind,
    MosaicSizingPersistence, MosaicStableIdentityBehavior, MosaicStateOwnerIdentity,
    MosaicStatePersistencePolicy, MosaicStateReplacementRule, MosaicStateSlotDescriptor,
    MosaicStateSlotId, MosaicStateSlotKind, MosaicStateTruthPosture, MosaicViewportConstraint,
    NamedMeasurementDefinition, NamedMeasurementToken, RegistrationCandidate, SurfaceDescriptor,
    SurfaceId, SurfaceKind, SurfacePlacementClass, SurfaceStateClass, COMPONENT_FAMILY_NAME,
    MOSAIC_PLACEMENT_POLICY_FAMILY_NAME, MOSAIC_REGION_KIND_FAMILY_NAME,
    MOSAIC_SIZING_CONTRACT_FAMILY_NAME, MOSAIC_STATE_SLOT_FAMILY_NAME, SURFACE_FAMILY_NAME,
};
use crate::facade::{WorthUi, WorthUiApp};

use super::structural_legality_snapshot_support::merge_support_candidates;

pub(super) fn standard_app() -> WorthUiApp {
    standard_app_with_dashboard_component(component("workspace.component.dashboard"))
}

pub(super) fn standard_app_with_dashboard_component(
    dashboard_component: ComponentDescriptor,
) -> WorthUiApp {
    WorthUi::app()
        .register_component(dashboard_component)
        .register_component(component("workspace.component.panel"))
        .register_surface(surface(
            "workspace.surface.main",
            "workspace.component.dashboard",
            SurfacePlacementClass::primary_region(),
        ))
        .register_surface(surface(
            "workspace.surface.overlay",
            "workspace.component.panel",
            SurfacePlacementClass::overlay_layer(),
        ))
        .register_mosaic_region_kind(primary_region())
        .register_mosaic_region_kind(overlay_region())
        .register_mosaic_placement_policy(primary_placement())
        .register_mosaic_placement_policy(overlay_placement())
        .register_mosaic_sizing_contract(fill_sizing())
        .register_mosaic_sizing_contract(overlay_sizing())
        .register_mosaic_state_slot(region_scroll_state())
        .register_mosaic_state_slot(overlay_pinned_state())
        .register_mosaic_state_slot(primary_pinned_state())
        .register_mosaic_state_slot(primary_surface_state())
        .freeze()
}

pub(super) fn support_catalog_with_extra<const N: usize>(
    extra: [RegistrationCandidate; N],
) -> CapabilitySupportCatalog {
    merge_support_candidates(
        vec![
            RegistrationCandidate::admitted(COMPONENT_FAMILY_NAME, "workspace.component.dashboard"),
            RegistrationCandidate::admitted(COMPONENT_FAMILY_NAME, "workspace.component.panel"),
            RegistrationCandidate::admitted(SURFACE_FAMILY_NAME, "workspace.surface.main"),
            RegistrationCandidate::admitted(SURFACE_FAMILY_NAME, "workspace.surface.overlay"),
            RegistrationCandidate::admitted(
                MOSAIC_REGION_KIND_FAMILY_NAME,
                "workspace.region.primary",
            ),
            RegistrationCandidate::admitted(
                MOSAIC_REGION_KIND_FAMILY_NAME,
                "workspace.region.overlay",
            ),
            RegistrationCandidate::admitted(
                MOSAIC_PLACEMENT_POLICY_FAMILY_NAME,
                "workspace.placement.primary",
            ),
            RegistrationCandidate::admitted(
                MOSAIC_PLACEMENT_POLICY_FAMILY_NAME,
                "workspace.placement.overlay",
            ),
            RegistrationCandidate::admitted(
                MOSAIC_SIZING_CONTRACT_FAMILY_NAME,
                "workspace.sizing.fill",
            ),
            RegistrationCandidate::admitted(
                MOSAIC_SIZING_CONTRACT_FAMILY_NAME,
                "workspace.sizing.overlay",
            ),
            RegistrationCandidate::admitted(
                MOSAIC_STATE_SLOT_FAMILY_NAME,
                "workspace.state.region_scroll",
            ),
            RegistrationCandidate::admitted(
                MOSAIC_STATE_SLOT_FAMILY_NAME,
                "workspace.state.overlay_pinned",
            ),
            RegistrationCandidate::admitted(
                MOSAIC_STATE_SLOT_FAMILY_NAME,
                "workspace.state.primary_pinned",
            ),
            RegistrationCandidate::admitted(
                MOSAIC_STATE_SLOT_FAMILY_NAME,
                "workspace.state.primary_surface",
            ),
        ],
        extra,
    )
}

fn component(id: &str) -> ComponentDescriptor {
    component_with_schema(id, format!("{id}.props"))
}

pub(super) fn component_with_schema(
    id: &str,
    schema_name: impl Into<String>,
) -> ComponentDescriptor {
    ComponentDescriptor::new(
        ComponentId::new(id).unwrap(),
        ComponentPropSchema::named(schema_name),
        ComponentChildPolicy::no_children(),
        ComponentStateOwnership::runtime_owned(),
    )
}

fn surface(
    id: &str,
    component_id: &str,
    placement_class: SurfacePlacementClass,
) -> SurfaceDescriptor {
    SurfaceDescriptor::new(
        SurfaceId::new(id).unwrap(),
        SurfaceKind::primary_content(),
        ComponentId::new(component_id).unwrap(),
        placement_class,
        SurfaceStateClass::restorable(),
    )
}

fn primary_region() -> MosaicRegionKindDescriptor {
    MosaicRegionKindDescriptor::new(
        MosaicRegionKindId::new("workspace.region.primary").unwrap(),
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

fn overlay_region() -> MosaicRegionKindDescriptor {
    MosaicRegionKindDescriptor::new(
        MosaicRegionKindId::new("workspace.region.overlay").unwrap(),
        MosaicRegionRole::overlay(),
    )
    .with_sizing_behavior(MosaicSizingBehavior::overlay_anchored())
    .with_scroll_ownership(MosaicScrollOwnership::no_scrolling())
    .with_focus_scope(MosaicFocusScopeKind::active_surface_scope())
    .with_child_rule(MosaicChildRule::accepts_surfaces())
    .with_allowed_surface_class(SurfacePlacementClass::overlay_layer())
    .with_persistence(MosaicRegionPersistence::restorable())
    .with_clipping(MosaicClippingPosture::clip_to_region())
    .with_hit_test(MosaicHitTestPosture::participates())
}

fn primary_placement() -> MosaicPlacementPolicyDescriptor {
    MosaicPlacementPolicyDescriptor::new(
        MosaicPlacementPolicyId::new("workspace.placement.primary").unwrap(),
        MosaicPlacementAction::dock(),
    )
    .with_source(MosaicPlacementSource::surface_class(
        SurfacePlacementClass::primary_region(),
    ))
    .with_target(MosaicPlacementTarget::region_role(
        MosaicRegionRole::primary(),
    ))
    .with_persistence(MosaicPlacementPersistence::restorable())
    .with_stable_identity_behavior(MosaicStableIdentityBehavior::preserve_surface_identity())
    .with_conflict_behavior(MosaicPlacementConflictBehavior::reject_conflict())
    .with_reload_reconciliation(MosaicPlacementReloadReconciliation::restore_when_possible())
}

fn overlay_placement() -> MosaicPlacementPolicyDescriptor {
    MosaicPlacementPolicyDescriptor::new(
        MosaicPlacementPolicyId::new("workspace.placement.overlay").unwrap(),
        MosaicPlacementAction::overlay(),
    )
    .with_source(MosaicPlacementSource::surface_class(
        SurfacePlacementClass::overlay_layer(),
    ))
    .with_target(MosaicPlacementTarget::region_role(
        MosaicRegionRole::overlay(),
    ))
    .with_persistence(MosaicPlacementPersistence::restorable())
    .with_stable_identity_behavior(MosaicStableIdentityBehavior::preserve_surface_identity())
    .with_conflict_behavior(MosaicPlacementConflictBehavior::reject_conflict())
    .with_reload_reconciliation(MosaicPlacementReloadReconciliation::restore_when_possible())
    .with_support(MosaicPlacementSupport::supported())
}

fn fill_sizing() -> MosaicSizingContractDescriptor {
    MosaicSizingContractDescriptor::new(
        MosaicSizingContractId::new("workspace.sizing.fill").unwrap(),
        MosaicSizingKind::fill(),
    )
    .with_measurement_authority(MosaicMeasurementAuthority::runtime_token())
    .with_resize_permission(MosaicResizePermission::user_resizable())
    .with_persistence(MosaicSizingPersistence::restorable())
    .with_overflow_behavior(MosaicOverflowBehavior::scroll_when_constrained())
    .with_parent_growth_behavior(MosaicParentGrowthBehavior::does_not_force_parent())
    .with_viewport_constraint(MosaicViewportConstraint::clamp_to_viewport())
    .with_named_measurement(measurement("workspace.measurement.fill"))
}

fn overlay_sizing() -> MosaicSizingContractDescriptor {
    MosaicSizingContractDescriptor::new(
        MosaicSizingContractId::new("workspace.sizing.overlay").unwrap(),
        MosaicSizingKind::fixed(),
    )
    .with_measurement_authority(MosaicMeasurementAuthority::runtime_token())
    .with_resize_permission(MosaicResizePermission::user_resizable())
    .with_persistence(MosaicSizingPersistence::restorable())
    .with_overflow_behavior(MosaicOverflowBehavior::scroll_when_constrained())
    .with_parent_growth_behavior(MosaicParentGrowthBehavior::does_not_force_parent())
    .with_viewport_constraint(MosaicViewportConstraint::clamp_to_viewport())
    .with_named_measurement(measurement("workspace.measurement.overlay"))
}

fn region_scroll_state() -> MosaicStateSlotDescriptor {
    MosaicStateSlotDescriptor::new(
        MosaicStateSlotId::new("workspace.state.region_scroll").unwrap(),
        MosaicStateSlotKind::scroll_position(),
    )
    .with_owner_identity(MosaicStateOwnerIdentity::mosaic_region_kind(
        MosaicRegionKindId::new("workspace.region.primary").unwrap(),
    ))
    .with_persistence_policy(MosaicStatePersistencePolicy::restore_across_hot_reload())
    .with_replacement_rule(MosaicStateReplacementRule::preserve_when_owner_matches())
    .with_truth_posture(MosaicStateTruthPosture::ui_runtime_state())
}

fn overlay_pinned_state() -> MosaicStateSlotDescriptor {
    MosaicStateSlotDescriptor::new(
        MosaicStateSlotId::new("workspace.state.overlay_pinned").unwrap(),
        MosaicStateSlotKind::pinned_posture(),
    )
    .with_owner_identity(MosaicStateOwnerIdentity::mosaic_region_kind(
        MosaicRegionKindId::new("workspace.region.overlay").unwrap(),
    ))
    .with_persistence_policy(MosaicStatePersistencePolicy::restore_across_hot_reload())
    .with_replacement_rule(MosaicStateReplacementRule::preserve_when_owner_matches())
    .with_truth_posture(MosaicStateTruthPosture::ui_runtime_state())
}

fn primary_surface_state() -> MosaicStateSlotDescriptor {
    MosaicStateSlotDescriptor::new(
        MosaicStateSlotId::new("workspace.state.primary_surface").unwrap(),
        MosaicStateSlotKind::active_primary_surface(),
    )
    .with_owner_identity(MosaicStateOwnerIdentity::surface(
        SurfaceId::new("workspace.surface.main").unwrap(),
    ))
    .with_persistence_policy(MosaicStatePersistencePolicy::restore_across_hot_reload())
    .with_replacement_rule(MosaicStateReplacementRule::preserve_when_owner_matches())
    .with_truth_posture(MosaicStateTruthPosture::ui_runtime_state())
}

fn primary_pinned_state() -> MosaicStateSlotDescriptor {
    MosaicStateSlotDescriptor::new(
        MosaicStateSlotId::new("workspace.state.primary_pinned").unwrap(),
        MosaicStateSlotKind::pinned_posture(),
    )
    .with_owner_identity(MosaicStateOwnerIdentity::mosaic_region_kind(
        MosaicRegionKindId::new("workspace.region.primary").unwrap(),
    ))
    .with_persistence_policy(MosaicStatePersistencePolicy::restore_across_hot_reload())
    .with_replacement_rule(MosaicStateReplacementRule::preserve_when_owner_matches())
    .with_truth_posture(MosaicStateTruthPosture::ui_runtime_state())
}

fn measurement(id: &str) -> NamedMeasurementDefinition {
    NamedMeasurementDefinition::new(
        NamedMeasurementToken::new(id).unwrap(),
        MeasurementValue::logical_pixels(320),
        MeasurementConstraint::between(
            MeasurementValue::logical_pixels(200),
            MeasurementValue::logical_pixels(640),
        ),
    )
}
