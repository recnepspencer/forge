use crate::capability::{
    CapabilitySnapshot, CapabilitySupportCatalog, RegisteredCapabilitySet, RegistrationCandidate,
    COMMAND_FAMILY_NAME, COMMAND_PROJECTION_FAMILY_NAME, COMPONENT_FAMILY_NAME, ICON_FAMILY_NAME,
    MOSAIC_PLACEMENT_POLICY_FAMILY_NAME, MOSAIC_REGION_KIND_FAMILY_NAME,
    MOSAIC_SIZING_CONTRACT_FAMILY_NAME, MOSAIC_STATE_SLOT_FAMILY_NAME, SURFACE_FAMILY_NAME,
    THEME_TOKEN_FAMILY_NAME, VIEW_BINDING_FAMILY_NAME,
};

pub(super) const SAMPLE_SNAPSHOT_FAMILY_COUNT: usize = 11;
pub(super) const SAMPLE_SNAPSHOT_TOTAL_WIDTH: usize = 21;

pub(super) fn sample_snapshot_with_support_catalog(
    base: &CapabilitySnapshot,
    support_catalog: CapabilitySupportCatalog,
) -> CapabilitySnapshot {
    CapabilitySnapshot::from_registered_capabilities_commands_command_projections_components_icons_surfaces_mosaic_regions_mosaic_placements_mosaic_sizing_mosaic_state_native_capabilities_plugin_slots_view_bindings_runtime_outcome_projections_settings_task_presentations_and_theme_tokens(
        clone_registered_capabilities(base),
        base.commands().clone(),
        base.command_projections().clone(),
        base.components().clone(),
        base.icons().clone(),
        base.surfaces().clone(),
        base.mosaic_regions().clone(),
        base.mosaic_placement_policies().clone(),
        base.mosaic_sizing_contracts().clone(),
        base.mosaic_state_slots().clone(),
        base.native_capabilities().clone(),
        base.plugin_slots().clone(),
        base.view_bindings().clone(),
        base.runtime_outcome_projections().clone(),
        base.settings().clone(),
        base.task_presentations().clone(),
        base.theme_tokens().clone(),
        support_catalog,
    )
}

pub(super) fn sample_support_catalog_with_extra<const N: usize>(
    extra: [RegistrationCandidate; N],
) -> CapabilitySupportCatalog {
    let mut candidates = sample_admitted_support_candidates();
    for candidate in extra {
        candidates.retain(|existing| {
            existing.family_name() != candidate.family_name()
                || existing.identity_text() != candidate.identity_text()
        });
        candidates.push(candidate);
    }
    CapabilitySupportCatalog::from_registration_candidates(&candidates)
}

fn sample_admitted_support_candidates() -> Vec<RegistrationCandidate> {
    vec![
        RegistrationCandidate::admitted(COMMAND_FAMILY_NAME, "workspace.command.inspect"),
        RegistrationCandidate::admitted(
            COMMAND_PROJECTION_FAMILY_NAME,
            "workspace.command_projection.inspect_actions",
        ),
        RegistrationCandidate::admitted(COMPONENT_FAMILY_NAME, "workspace.component.dashboard"),
        RegistrationCandidate::admitted(
            COMPONENT_FAMILY_NAME,
            "workspace.component.inspector_panel",
        ),
        RegistrationCandidate::admitted(ICON_FAMILY_NAME, "workspace.icon.inspect"),
        RegistrationCandidate::admitted(ICON_FAMILY_NAME, "workspace.icon.surface.inspector"),
        RegistrationCandidate::admitted(SURFACE_FAMILY_NAME, "workspace.surface.main"),
        RegistrationCandidate::admitted(SURFACE_FAMILY_NAME, "workspace.surface.overlay"),
        RegistrationCandidate::admitted(SURFACE_FAMILY_NAME, "workspace.surface.inspector"),
        RegistrationCandidate::admitted(
            VIEW_BINDING_FAMILY_NAME,
            "workspace.view_binding.selection",
        ),
        RegistrationCandidate::admitted(THEME_TOKEN_FAMILY_NAME, "theme.text.primary"),
        RegistrationCandidate::admitted(THEME_TOKEN_FAMILY_NAME, "theme.text.default"),
        RegistrationCandidate::admitted(MOSAIC_REGION_KIND_FAMILY_NAME, "workspace.region.primary"),
        RegistrationCandidate::admitted(MOSAIC_REGION_KIND_FAMILY_NAME, "workspace.region.overlay"),
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
            "workspace.state.primary_surface",
        ),
    ]
}

fn clone_registered_capabilities(snapshot: &CapabilitySnapshot) -> RegisteredCapabilitySet {
    let registered = snapshot.registered_capabilities();
    RegisteredCapabilitySet::from_counts(
        registered.registered_family_count(),
        registered.total_width(),
    )
}
