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

pub(in crate::domain_computation::primary_graph) fn installed_composed_capability_world(
    scenario: super::capability_seed::CapabilityCompositionScenario,
) -> AuthorizationWorld {
    capability_world(1, "primary", CapabilityGrantPopulation::Composed(scenario))
}

pub(in crate::domain_computation::primary_graph) fn installed_capability_world_with_same_resource_unrelated(
    unrelated: usize,
) -> AuthorizationWorld {
    capability_world(
        2,
        "primary",
        CapabilityGrantPopulation::CurrentWithSameResourceUnrelated(unrelated),
    )
}

pub(in crate::domain_computation::primary_graph) fn installed_delegated_capability_world(
) -> AuthorizationWorld {
    installed_delegated_capability_world_at_depth(2)
}

pub(in crate::domain_computation::primary_graph) fn installed_delegated_capability_world_at_depth(
    links: usize,
) -> AuthorizationWorld {
    capability_world(
        2,
        "primary",
        CapabilityGrantPopulation::Delegated {
            links,
            unrelated: 0,
        },
    )
}

pub(in crate::domain_computation::primary_graph) fn installed_delegated_capability_world_with_unrelated(
    unrelated: usize,
) -> AuthorizationWorld {
    capability_world(
        2,
        "primary",
        CapabilityGrantPopulation::Delegated {
            links: 2,
            unrelated,
        },
    )
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
