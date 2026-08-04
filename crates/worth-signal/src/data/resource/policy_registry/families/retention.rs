use super::super::super::policy::ResourceRetentionPolicyDeclaration;
use super::super::errors::ResourcePolicyResolutionError;
use super::super::identity::{
    ResourcePolicyDigest, ResourcePolicyKind, ResourcePolicySelectionBasis,
};
use super::super::reference::ValidatedResourcePolicyReference;
use super::super::registration::built_in_resource_policy_registration;
use super::super::registration::ResourcePolicyRegistration;
use super::super::registry::FrozenResourcePolicyRegistry;

impl FrozenResourcePolicyRegistry {
    pub(in crate::data::resource::policy_registry) fn resolve_retention(
        &self,
        policy: &ResourceRetentionPolicyDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceRetentionPolicyDeclaration::RetainAllTransitions => self.built_in_policy(
                ResourcePolicyKind::Retention,
                "signal.resource.retention.retain-all-transitions",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("retention:retain-all-transitions"),
            )?,
            ResourceRetentionPolicyDeclaration::RetainOperationalLifecycleSummary => self
                .built_in_policy(
                    ResourcePolicyKind::Retention,
                    "signal.resource.retention.terminal-summaries-only",
                    ResourcePolicySelectionBasis::BuiltInDefault,
                    ResourcePolicyDigest::new("retention:terminal-summaries-only"),
                )?,
            ResourceRetentionPolicyDeclaration::TerminalSummariesOnly => self.built_in_policy(
                ResourcePolicyKind::Retention,
                "signal.resource.retention.terminal-summaries-only",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("retention:terminal-summaries-only"),
            )?,
            ResourceRetentionPolicyDeclaration::CompactSuperseded => self.built_in_policy(
                ResourcePolicyKind::Retention,
                "signal.resource.retention.compact-superseded",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("retention:compact-superseded"),
            )?,
            ResourceRetentionPolicyDeclaration::CompactCancelled => self.built_in_policy(
                ResourcePolicyKind::Retention,
                "signal.resource.retention.compact-cancelled",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("retention:compact-cancelled"),
            )?,
            ResourceRetentionPolicyDeclaration::CompactTimedOut => self.built_in_policy(
                ResourcePolicyKind::Retention,
                "signal.resource.retention.compact-timed-out",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("retention:compact-timed-out"),
            )?,
            ResourceRetentionPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Retention, name)?
            }
        })
    }
}

pub(super) fn built_in_registrations() -> Vec<ResourcePolicyRegistration> {
    [
        (
            13,
            ResourcePolicyKind::Retention,
            "signal.resource.retention.terminal-summaries-only",
            7,
        ),
        (
            46,
            ResourcePolicyKind::Retention,
            "signal.resource.retention.retain-all-transitions",
            7,
        ),
        (
            47,
            ResourcePolicyKind::Retention,
            "signal.resource.retention.compact-superseded",
            7,
        ),
        (
            48,
            ResourcePolicyKind::Retention,
            "signal.resource.retention.compact-cancelled",
            7,
        ),
        (
            49,
            ResourcePolicyKind::Retention,
            "signal.resource.retention.compact-timed-out",
            7,
        ),
    ]
    .into_iter()
    .map(|(id, kind, name, contract)| {
        built_in_resource_policy_registration(id, kind, name, contract)
    })
    .collect()
}
