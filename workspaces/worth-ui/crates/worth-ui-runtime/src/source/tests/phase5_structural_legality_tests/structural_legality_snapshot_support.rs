use crate::capability::{
    CapabilitySnapshot, CapabilitySupportCatalog, RegisteredCapabilitySet, RegistrationCandidate,
};

pub(super) fn snapshot_with_support_catalog(
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

pub(super) fn merge_support_candidates(
    mut base: Vec<RegistrationCandidate>,
    extra: impl IntoIterator<Item = RegistrationCandidate>,
) -> CapabilitySupportCatalog {
    for candidate in extra {
        base.retain(|existing| {
            existing.family_name() != candidate.family_name()
                || existing.identity_text() != candidate.identity_text()
        });
        base.push(candidate);
    }
    CapabilitySupportCatalog::from_registration_candidates(&base)
}

fn clone_registered_capabilities(snapshot: &CapabilitySnapshot) -> RegisteredCapabilitySet {
    let registered = snapshot.registered_capabilities();
    RegisteredCapabilitySet::from_counts(
        registered.registered_family_count(),
        registered.total_width(),
    )
}
