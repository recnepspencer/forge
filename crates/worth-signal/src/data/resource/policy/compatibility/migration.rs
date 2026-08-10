use crate::data::resource::policy::{
    ResourceDiagnosticsDecisionClass, ResourceRetentionDecisionClass,
};
use crate::data::resource::policy_registry::{
    FrozenResourcePolicyDescriptor, FrozenResourcePolicyRegistry,
    ResourcePolicyCompatibilityPosture, ResourcePolicyKind,
};

use super::vocabulary::ResourcePolicyCompatibilityClass;

#[derive(Debug, Clone)]
pub(super) struct CompatibilityMigrationEvidence {
    pub(super) class: ResourcePolicyCompatibilityClass,
    pub(super) historical_retention_class: Option<ResourceRetentionDecisionClass>,
    pub(super) current_retention_class: Option<ResourceRetentionDecisionClass>,
    pub(super) historical_diagnostics_class: Option<ResourceDiagnosticsDecisionClass>,
    pub(super) current_diagnostics_class: Option<ResourceDiagnosticsDecisionClass>,
    pub(super) defaulted_parameter_names: Vec<String>,
    pub(super) canonical_truth_preserved: bool,
    pub(super) retained_history_unavailable: bool,
    pub(super) diagnostics_details_unavailable: bool,
}

fn evidence(
    class: ResourcePolicyCompatibilityClass,
    historical: &FrozenResourcePolicyDescriptor,
    current: Option<&FrozenResourcePolicyDescriptor>,
    defaulted_parameter_names: Vec<String>,
    canonical_truth_preserved: bool,
    retained_history_unavailable: bool,
    diagnostics_details_unavailable: bool,
) -> CompatibilityMigrationEvidence {
    CompatibilityMigrationEvidence {
        class,
        historical_retention_class: retention_class(historical),
        current_retention_class: current.and_then(retention_class),
        historical_diagnostics_class: diagnostics_class(historical),
        current_diagnostics_class: current.and_then(diagnostics_class),
        defaulted_parameter_names,
        canonical_truth_preserved,
        retained_history_unavailable,
        diagnostics_details_unavailable,
    }
}

pub(super) fn classify_family_compatibility(
    historical: &FrozenResourcePolicyDescriptor,
    current: &FrozenResourcePolicyDescriptor,
    registry: &FrozenResourcePolicyRegistry,
) -> CompatibilityMigrationEvidence {
    let Some(registry_descriptor) = registry.resolve_by_id(historical.descriptor().id()) else {
        return evidence(
            ResourcePolicyCompatibilityClass::MissingDescriptor,
            historical,
            None,
            Vec::new(),
            false,
            false,
            false,
        );
    };

    if registry_descriptor.descriptor_digest() != historical.descriptor().descriptor_digest() {
        return classify_descriptor_drift(
            historical,
            current,
            registry_descriptor.compatibility_posture(),
        );
    }
    if current.descriptor().descriptor_digest() != historical.descriptor().descriptor_digest() {
        return classify_descriptor_drift(
            historical,
            current,
            current.descriptor().compatibility_posture(),
        );
    }
    if current.frozen_digest() != historical.frozen_digest() {
        return evidence(
            ResourcePolicyCompatibilityClass::ParameterDigestDrift,
            historical,
            Some(current),
            Vec::new(),
            false,
            false,
            false,
        );
    }
    evidence(
        ResourcePolicyCompatibilityClass::ExactDescriptorMatch,
        historical,
        Some(current),
        Vec::new(),
        true,
        false,
        false,
    )
}

fn classify_descriptor_drift(
    historical: &FrozenResourcePolicyDescriptor,
    current: &FrozenResourcePolicyDescriptor,
    posture: ResourcePolicyCompatibilityPosture,
) -> CompatibilityMigrationEvidence {
    match posture {
        ResourcePolicyCompatibilityPosture::IncompatibleVersion => evidence(
            ResourcePolicyCompatibilityClass::VersionIncompatible,
            historical,
            Some(current),
            Vec::new(),
            false,
            false,
            false,
        ),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch => evidence(
            ResourcePolicyCompatibilityClass::DecisionSemanticsDrift,
            historical,
            Some(current),
            Vec::new(),
            false,
            false,
            false,
        ),
        ResourcePolicyCompatibilityPosture::CompatibleVersion => {
            classify_compatible_version_drift(historical, current)
        }
    }
}

fn classify_compatible_version_drift(
    historical: &FrozenResourcePolicyDescriptor,
    current: &FrozenResourcePolicyDescriptor,
) -> CompatibilityMigrationEvidence {
    match historical.descriptor().kind() {
        ResourcePolicyKind::Retention => classify_retention_compatible_drift(historical, current),
        ResourcePolicyKind::Diagnostics => {
            classify_diagnostics_parameter_or_richness_drift(historical, current)
        }
        _ => evidence(
            ResourcePolicyCompatibilityClass::DecisionSemanticsDrift,
            historical,
            Some(current),
            Vec::new(),
            false,
            false,
            false,
        ),
    }
}

fn classify_diagnostics_parameter_or_richness_drift(
    historical: &FrozenResourcePolicyDescriptor,
    current: &FrozenResourcePolicyDescriptor,
) -> CompatibilityMigrationEvidence {
    if let Some(expansion) = classify_diagnostics_parameter_expansion(historical, current) {
        return expansion;
    }
    classify_diagnostics_compatible_drift(historical, current)
}

fn classify_diagnostics_parameter_expansion(
    historical: &FrozenResourcePolicyDescriptor,
    current: &FrozenResourcePolicyDescriptor,
) -> Option<CompatibilityMigrationEvidence> {
    let historical_name = historical.descriptor().semantic_name().as_str();
    let current_name = current.descriptor().semantic_name().as_str();
    if historical_name != "signal.resource.diagnostics.budgeted-expansion"
        || current_name != "signal.resource.diagnostics.forensic-expansion-budget"
    {
        return None;
    }

    let historical_replay_width = diagnostics_replay_width(historical)?;
    let current_replay_width = diagnostics_replay_width(current)?;
    let current_forensic_width = diagnostics_forensic_width(current)?;
    let class = if historical_replay_width == current_replay_width
        && current_forensic_width == historical_replay_width
    {
        ResourcePolicyCompatibilityClass::CompatibleParameterExpansion
    } else {
        ResourcePolicyCompatibilityClass::DecisionSemanticsDrift
    };
    Some(evidence(
        class,
        historical,
        Some(current),
        if class == ResourcePolicyCompatibilityClass::CompatibleParameterExpansion {
            vec!["max_forensic_reconstruction_width".to_owned()]
        } else {
            Vec::new()
        },
        class == ResourcePolicyCompatibilityClass::CompatibleParameterExpansion,
        false,
        false,
    ))
}

fn classify_retention_compatible_drift(
    historical: &FrozenResourcePolicyDescriptor,
    current: &FrozenResourcePolicyDescriptor,
) -> CompatibilityMigrationEvidence {
    let historical_class = retention_class(historical);
    let current_class = retention_class(current);
    if matches!(
        historical_class,
        Some(ResourceRetentionDecisionClass::RetainAllTransitions)
    ) && current_class.is_some()
        && current_class != historical_class
    {
        evidence(
            ResourcePolicyCompatibilityClass::CompatibleRetentionNarrowing,
            historical,
            Some(current),
            Vec::new(),
            true,
            true,
            false,
        )
    } else {
        evidence(
            ResourcePolicyCompatibilityClass::DecisionSemanticsDrift,
            historical,
            Some(current),
            Vec::new(),
            false,
            false,
            false,
        )
    }
}

fn classify_diagnostics_compatible_drift(
    historical: &FrozenResourcePolicyDescriptor,
    current: &FrozenResourcePolicyDescriptor,
) -> CompatibilityMigrationEvidence {
    let historical_class = diagnostics_class(historical);
    let current_class = diagnostics_class(current);
    if historical_class.is_some() && current_class.is_some() && historical_class != current_class {
        let details_unavailable = matches!(
            current_class,
            Some(
                ResourceDiagnosticsDecisionClass::RetainedOnly
                    | ResourceDiagnosticsDecisionClass::DenyColdExpansion
            )
        ) && !matches!(
            historical_class,
            Some(
                ResourceDiagnosticsDecisionClass::RetainedOnly
                    | ResourceDiagnosticsDecisionClass::DenyColdExpansion
            )
        );
        evidence(
            ResourcePolicyCompatibilityClass::CompatibleDiagnosticsRichnessChange,
            historical,
            Some(current),
            Vec::new(),
            true,
            false,
            details_unavailable,
        )
    } else {
        evidence(
            ResourcePolicyCompatibilityClass::DecisionSemanticsDrift,
            historical,
            Some(current),
            Vec::new(),
            false,
            false,
            false,
        )
    }
}

fn retention_class(
    frozen: &FrozenResourcePolicyDescriptor,
) -> Option<ResourceRetentionDecisionClass> {
    match frozen.descriptor().semantic_name().as_str() {
        "signal.resource.retention.retain-all-transitions" => {
            Some(ResourceRetentionDecisionClass::RetainAllTransitions)
        }
        "signal.resource.retention.terminal-summaries-only" => {
            Some(ResourceRetentionDecisionClass::TerminalSummariesOnly)
        }
        "signal.resource.retention.compact-superseded" => {
            Some(ResourceRetentionDecisionClass::CompactSuperseded)
        }
        "signal.resource.retention.compact-cancelled" => {
            Some(ResourceRetentionDecisionClass::CompactCancelled)
        }
        "signal.resource.retention.compact-timed-out" => {
            Some(ResourceRetentionDecisionClass::CompactTimedOut)
        }
        _ => None,
    }
}

fn diagnostics_class(
    frozen: &FrozenResourcePolicyDescriptor,
) -> Option<ResourceDiagnosticsDecisionClass> {
    match frozen.descriptor().semantic_name().as_str() {
        "signal.resource.diagnostics.retained-only" => {
            Some(ResourceDiagnosticsDecisionClass::RetainedOnly)
        }
        "signal.resource.diagnostics.budgeted-expansion" => {
            Some(ResourceDiagnosticsDecisionClass::BudgetedExpansion)
        }
        "signal.resource.diagnostics.forensic-expansion-budget" => {
            Some(ResourceDiagnosticsDecisionClass::ForensicExpansionBudget)
        }
        "signal.resource.diagnostics.deny-cold-expansion" => {
            Some(ResourceDiagnosticsDecisionClass::DenyColdExpansion)
        }
        _ => None,
    }
}

fn diagnostics_replay_width(frozen: &FrozenResourcePolicyDescriptor) -> Option<u32> {
    parse_diagnostics_width(
        frozen.parameter_digest().as_str(),
        "max-replay-reconstruction-width:",
    )
}

fn diagnostics_forensic_width(frozen: &FrozenResourcePolicyDescriptor) -> Option<u32> {
    parse_diagnostics_width(
        frozen.parameter_digest().as_str(),
        "max-forensic-reconstruction-width:",
    )
}

fn parse_diagnostics_width(digest: &str, marker: &str) -> Option<u32> {
    let suffix = digest.split(marker).nth(1)?;
    let value = suffix.split(':').next()?;
    if value == "none" {
        None
    } else {
        value.parse().ok()
    }
}
