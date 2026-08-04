use super::world_installation::{
    installed_authorization_world_with_principal_count, AuthorizationWorld,
    CapabilityGrantPopulation,
};
use super::WorthQueryApplicationQueryResourceProfile;

pub(in crate::domain_computation::primary_graph) fn installed_capability_authorization_world(
) -> AuthorizationWorld {
    capability_world(1, "primary", CapabilityGrantPopulation::Current)
}

pub(in crate::domain_computation::primary_graph) fn installed_capability_world_with_label(
    label: &str,
) -> AuthorizationWorld {
    capability_world(1, label, CapabilityGrantPopulation::Current)
}

pub(in crate::domain_computation::primary_graph) fn installed_capability_live_world(
) -> AuthorizationWorld {
    installed_capability_live_world_with_label("primary")
}

pub(in crate::domain_computation::primary_graph) fn installed_capability_live_world_with_label(
    label: &str,
) -> AuthorizationWorld {
    installed_authorization_world_with_principal_count(
        &[("principal-1", "account-1")],
        false,
        2,
        label,
        WorthQueryApplicationQueryResourceProfile::default(),
        CapabilityGrantPopulation::Current,
    )
}

pub(in crate::domain_computation::primary_graph) fn installed_capability_replacement_world(
) -> AuthorizationWorld {
    capability_world(
        1,
        "primary",
        CapabilityGrantPopulation::CurrentAndFutureReplacement,
    )
}

pub(in crate::domain_computation::primary_graph) fn installed_delegated_capability_world(
) -> AuthorizationWorld {
    capability_world(2, "primary", CapabilityGrantPopulation::Delegated)
}

fn capability_world(
    principal_count: usize,
    label: &str,
    grants: CapabilityGrantPopulation,
) -> AuthorizationWorld {
    installed_authorization_world_with_principal_count(
        &[],
        false,
        principal_count,
        label,
        WorthQueryApplicationQueryResourceProfile::default(),
        grants,
    )
}
