use crate::capability::{
    CapabilitySupportCatalog, RegistrationCandidate, COMMAND_FAMILY_NAME,
    COMMAND_PROJECTION_FAMILY_NAME, COMPONENT_FAMILY_NAME, ICON_FAMILY_NAME, SURFACE_FAMILY_NAME,
    THEME_TOKEN_FAMILY_NAME, VIEW_BINDING_FAMILY_NAME,
};

use super::binding_snapshot_support::merge_support_candidates;

pub(super) fn support_catalog_with_extra<const N: usize>(
    extra: [RegistrationCandidate; N],
) -> CapabilitySupportCatalog {
    merge_support_candidates(
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
            RegistrationCandidate::admitted(SURFACE_FAMILY_NAME, "workspace.surface.inspector"),
            RegistrationCandidate::admitted(
                VIEW_BINDING_FAMILY_NAME,
                "workspace.view_binding.selection",
            ),
            RegistrationCandidate::admitted(THEME_TOKEN_FAMILY_NAME, "theme.text.primary"),
            RegistrationCandidate::admitted(THEME_TOKEN_FAMILY_NAME, "theme.text.default"),
        ],
        extra,
    )
}
