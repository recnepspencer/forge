use super::*;

pub(in crate::tests::resource_runtime) fn resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    ResourceNodeDeclaration::new(
        ResourceNodeId::from_node(node),
        ResourcePayloadContract::new(ResourcePayloadContractId::new(7))
            .with_max_payload_bytes(1024),
    )
}

pub(in crate::tests::resource_runtime) fn hide_pending_output_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_output_continuity_policy(ResourceOutputContinuityPolicyDeclaration::HideWhilePending)
}

pub(in crate::tests::resource_runtime) fn hide_after_timeout_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_output_continuity_policy(ResourceOutputContinuityPolicyDeclaration::HideAfterTimeout)
}

pub(in crate::tests::resource_runtime) fn hide_after_rejection_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_output_continuity_policy(
        ResourceOutputContinuityPolicyDeclaration::HideAfterRejection,
    )
}

pub(in crate::tests::resource_runtime) fn hide_after_cancellation_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_output_continuity_policy(
        ResourceOutputContinuityPolicyDeclaration::HideAfterCancellation,
    )
}

pub(in crate::tests::resource_runtime) fn hide_after_supersession_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_output_continuity_policy(
        ResourceOutputContinuityPolicyDeclaration::HideAfterSupersession,
    )
}

pub(in crate::tests::resource_runtime) fn retain_all_transitions_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_retention_policy(ResourceRetentionPolicyDeclaration::RetainAllTransitions)
}

pub(in crate::tests::resource_runtime) fn terminal_summaries_only_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_retention_policy(ResourceRetentionPolicyDeclaration::TerminalSummariesOnly)
}

pub(in crate::tests::resource_runtime) fn compact_cancelled_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_retention_policy(ResourceRetentionPolicyDeclaration::CompactCancelled)
}

pub(in crate::tests::resource_runtime) fn compact_superseded_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_retention_policy(ResourceRetentionPolicyDeclaration::CompactSuperseded)
}

pub(in crate::tests::resource_runtime) fn retained_only_diagnostics_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_diagnostics_policy(ResourceDiagnosticsPolicyDeclaration::RetainedOnly)
}

pub(in crate::tests::resource_runtime) fn budgeted_diagnostics_resource_declaration(
    node: NodeId,
    max_replay_reconstruction_width: u32,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_diagnostics_policy(
        ResourceDiagnosticsPolicyDeclaration::BudgetedExpansion {
            max_replay_reconstruction_width,
        },
    )
}

pub(in crate::tests::resource_runtime) fn forensic_diagnostics_resource_declaration(
    node: NodeId,
    max_replay_reconstruction_width: u32,
    max_forensic_reconstruction_width: u32,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_diagnostics_policy(
        ResourceDiagnosticsPolicyDeclaration::ForensicExpansionBudget {
            max_replay_reconstruction_width,
            max_forensic_reconstruction_width,
        },
    )
}

pub(in crate::tests::resource_runtime) fn deny_cold_diagnostics_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_diagnostics_policy(ResourceDiagnosticsPolicyDeclaration::DenyColdExpansion)
}

pub(in crate::tests::resource_runtime) fn identical_only_replay_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_replay_policy(ResourceReplayPolicyDeclaration::IdenticalOnly)
}

pub(in crate::tests::resource_runtime) fn retention_only_replay_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_replay_policy(ResourceReplayPolicyDeclaration::CompatibleRetentionNarrowing)
}

pub(in crate::tests::resource_runtime) fn diagnostics_only_replay_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_replay_policy(ResourceReplayPolicyDeclaration::CompatibleDiagnosticsRichnessChange)
}

pub(in crate::tests::resource_runtime) fn parameter_expansion_only_replay_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_replay_policy(ResourceReplayPolicyDeclaration::CompatibleParameterExpansion)
}

pub(in crate::tests::resource_runtime) fn parameter_and_retention_replay_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_replay_policy(
        ResourceReplayPolicyDeclaration::CompatibleParameterExpansionAndRetentionNarrowing,
    )
}

pub(in crate::tests::resource_runtime) fn parameter_and_diagnostics_replay_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node).with_replay_policy(
        ResourceReplayPolicyDeclaration::CompatibleParameterExpansionAndDiagnosticsRichnessChange,
    )
}

pub(in crate::tests::resource_runtime) fn deny_on_unknown_or_missing_replay_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    resource_declaration(node)
        .with_replay_policy(ResourceReplayPolicyDeclaration::DenyOnUnknownOrMissing)
}

pub(in crate::tests::resource_runtime) fn compact_timed_out_resource_declaration(
    node: NodeId,
) -> ResourceNodeDeclaration {
    timeout_resource_declaration(node, 3)
        .with_retention_policy(ResourceRetentionPolicyDeclaration::CompactTimedOut)
}

pub(in crate::tests::resource_runtime) fn compatible_policy_registry_for(
    kind: ResourcePolicyKind,
    semantic_name: &str,
) -> FrozenResourcePolicyRegistry {
    compatible_policy_registry_for_entries([(kind, semantic_name)])
}

pub(in crate::tests::resource_runtime) fn compatible_policy_registry_for_entries<const N: usize>(
    entries: [(ResourcePolicyKind, &str); N],
) -> FrozenResourcePolicyRegistry {
    let registrations = built_in_policy_registrations()
        .into_iter()
        .map(|registration| {
            if entries.iter().any(|(kind, semantic_name)| {
                registration.kind() == *kind
                    && registration.semantic_name().as_str() == *semantic_name
            }) {
                ResourcePolicyRegistration::new(
                    registration.id(),
                    registration.kind(),
                    registration.semantic_name().clone(),
                    ResourcePolicyVersion::new(2, 0),
                    registration.cost_contract(),
                    ResourcePolicyCompatibilityPosture::CompatibleVersion,
                )
            } else {
                registration
            }
        })
        .collect();
    FrozenResourcePolicyRegistry::new(registrations).expect("compatible registry should freeze")
}
