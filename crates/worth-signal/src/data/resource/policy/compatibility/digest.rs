use crate::data::resource::policy::{
    ResourceDiagnosticsDecisionClass, ResourceRetentionDecisionClass,
};
use crate::data::resource::policy_registry::{
    ResourcePolicyDescriptorId, ResourcePolicyDigest, ResourcePolicyVersion,
};

use super::classification::ResourcePolicyCompatibilityFamilyReport;
pub(super) fn compatibility_digest(
    historical_registry_digest: &ResourcePolicyDigest,
    current_registry_digest: &ResourcePolicyDigest,
    families: &[ResourcePolicyCompatibilityFamilyReport],
) -> ResourcePolicyDigest {
    let joined = families
        .iter()
        .map(|family| {
            format!(
                "{:?}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
                family.kind(),
                family.class().as_str(),
                family.historical_descriptor_id().get(),
                family
                    .current_descriptor_id()
                    .map(ResourcePolicyDescriptorId::get)
                    .unwrap_or(u64::MAX),
                version_string(family.historical_version()),
                family
                    .current_version()
                    .map(version_string)
                    .unwrap_or_else(|| "missing".to_owned()),
                family.historical_frozen_digest().as_str(),
                family
                    .current_frozen_digest()
                    .map(ResourcePolicyDigest::as_str)
                    .unwrap_or("missing"),
                family
                    .historical_retention_class()
                    .map(retention_class_str)
                    .unwrap_or("none"),
                family
                    .current_retention_class()
                    .map(retention_class_str)
                    .unwrap_or("none"),
                family
                    .historical_diagnostics_class()
                    .map(diagnostics_class_str)
                    .unwrap_or("none"),
                family
                    .current_diagnostics_class()
                    .map(diagnostics_class_str)
                    .unwrap_or("none"),
                family.defaulted_parameter_names().join(","),
                family.canonical_truth_preserved(),
                family.retained_history_unavailable(),
                family.diagnostics_details_unavailable()
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    ResourcePolicyDigest::new(format!(
        "resource-policy-compatibility:{}:{}:{joined}",
        historical_registry_digest.as_str(),
        current_registry_digest.as_str()
    ))
}

fn version_string(version: ResourcePolicyVersion) -> String {
    format!("{}.{}", version.major(), version.minor())
}

fn retention_class_str(class: ResourceRetentionDecisionClass) -> &'static str {
    match class {
        ResourceRetentionDecisionClass::RetainAllTransitions => "retain-all-transitions",
        ResourceRetentionDecisionClass::TerminalSummariesOnly => "terminal-summaries-only",
        ResourceRetentionDecisionClass::CompactSuperseded => "compact-superseded",
        ResourceRetentionDecisionClass::CompactCancelled => "compact-cancelled",
        ResourceRetentionDecisionClass::CompactTimedOut => "compact-timed-out",
    }
}

fn diagnostics_class_str(class: ResourceDiagnosticsDecisionClass) -> &'static str {
    match class {
        ResourceDiagnosticsDecisionClass::RetainedOnly => "retained-only",
        ResourceDiagnosticsDecisionClass::BudgetedExpansion => "budgeted-expansion",
        ResourceDiagnosticsDecisionClass::ForensicExpansionBudget => "forensic-expansion-budget",
        ResourceDiagnosticsDecisionClass::DenyColdExpansion => "deny-cold-expansion",
    }
}
