use super::super::super::policy::ResourceReplayPolicyDeclaration;
use super::super::errors::ResourcePolicyResolutionError;
use super::super::identity::{
    ResourcePolicyDigest, ResourcePolicyKind, ResourcePolicySelectionBasis,
};
use super::super::reference::ValidatedResourcePolicyReference;
use super::super::registration::built_in_resource_policy_registration;
use super::super::registration::ResourcePolicyRegistration;
use super::super::registry::FrozenResourcePolicyRegistry;

impl FrozenResourcePolicyRegistry {
    pub(in crate::data::resource::policy_registry) fn resolve_replay(
        &self,
        policy: &ResourceReplayPolicyDeclaration,
    ) -> Result<ValidatedResourcePolicyReference, ResourcePolicyResolutionError> {
        Ok(match policy {
            ResourceReplayPolicyDeclaration::IdenticalOnly => self.built_in_policy(
                ResourcePolicyKind::Replay,
                "signal.resource.replay.identical-only",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("replay:identical-only"),
            )?,
            ResourceReplayPolicyDeclaration::CompatibleParameterExpansion => self.built_in_policy(
                ResourcePolicyKind::Replay,
                "signal.resource.replay.compatible-parameter-expansion",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("replay:compatible-parameter-expansion"),
            )?,
            ResourceReplayPolicyDeclaration::CompatibleRetentionNarrowing => self.built_in_policy(
                ResourcePolicyKind::Replay,
                "signal.resource.replay.compatible-retention-narrowing",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("replay:compatible-retention-narrowing"),
            )?,
            ResourceReplayPolicyDeclaration::CompatibleDiagnosticsRichnessChange => self
                .built_in_policy(
                    ResourcePolicyKind::Replay,
                    "signal.resource.replay.compatible-diagnostics-richness-change",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new("replay:compatible-diagnostics-richness-change"),
                )?,
            ResourceReplayPolicyDeclaration::CompatibleParameterExpansionAndRetentionNarrowing => self
                .built_in_policy(
                    ResourcePolicyKind::Replay,
                    "signal.resource.replay.compatible-parameter-expansion-and-retention-narrowing",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "replay:compatible-parameter-expansion-and-retention-narrowing",
                    ),
                )?,
            ResourceReplayPolicyDeclaration::CompatibleParameterExpansionAndDiagnosticsRichnessChange => self
                .built_in_policy(
                    ResourcePolicyKind::Replay,
                    "signal.resource.replay.compatible-parameter-expansion-and-diagnostics-richness-change",
                    ResourcePolicySelectionBasis::DeclaredBuiltIn,
                    ResourcePolicyDigest::new(
                        "replay:compatible-parameter-expansion-and-diagnostics-richness-change",
                    ),
                )?,
            ResourceReplayPolicyDeclaration::CompatibleParameterExpansionAndRetentionNarrowingAndDiagnosticsRichnessChange => self
                .built_in_policy(
                    ResourcePolicyKind::Replay,
                    "signal.resource.replay.compatible-parameter-expansion-and-retention-narrowing-and-diagnostics-richness-change",
                    ResourcePolicySelectionBasis::BuiltInDefault,
                    ResourcePolicyDigest::new(
                        "replay:compatible-parameter-expansion-and-retention-narrowing-and-diagnostics-richness-change",
                    ),
                )?,
            ResourceReplayPolicyDeclaration::CompatibleRetentionNarrowingAndDiagnosticsRichnessChange => self
                .built_in_policy(
                    ResourcePolicyKind::Replay,
                    "signal.resource.replay.compatible-retention-narrowing-and-diagnostics-richness-change",
                    ResourcePolicySelectionBasis::BuiltInDefault,
                    ResourcePolicyDigest::new(
                        "replay:compatible-retention-narrowing-and-diagnostics-richness-change",
                    ),
                )?,
            ResourceReplayPolicyDeclaration::DenyOnUnknownOrMissing => self.built_in_policy(
                ResourcePolicyKind::Replay,
                "signal.resource.replay.deny-on-unknown-or-missing",
                ResourcePolicySelectionBasis::DeclaredBuiltIn,
                ResourcePolicyDigest::new("replay:deny-on-unknown-or-missing"),
            )?,
            ResourceReplayPolicyDeclaration::Named { name } => {
                self.resolve_named(ResourcePolicyKind::Replay, name)?
            }
        })
    }
}

pub(super) fn built_in_registrations() -> Vec<ResourcePolicyRegistration> {
    [
        (53, ResourcePolicyKind::Replay, "signal.resource.replay.identical-only", 21),
        (59, ResourcePolicyKind::Replay, "signal.resource.replay.compatible-parameter-expansion", 21),
        (54, ResourcePolicyKind::Replay, "signal.resource.replay.compatible-retention-narrowing", 21),
        (55, ResourcePolicyKind::Replay, "signal.resource.replay.compatible-diagnostics-richness-change", 21),
        (61, ResourcePolicyKind::Replay, "signal.resource.replay.compatible-parameter-expansion-and-retention-narrowing", 21),
        (62, ResourcePolicyKind::Replay, "signal.resource.replay.compatible-parameter-expansion-and-diagnostics-richness-change", 21),
        (56, ResourcePolicyKind::Replay, "signal.resource.replay.compatible-retention-narrowing-and-diagnostics-richness-change", 21),
        (60, ResourcePolicyKind::Replay, "signal.resource.replay.compatible-parameter-expansion-and-retention-narrowing-and-diagnostics-richness-change", 21),
        (57, ResourcePolicyKind::Replay, "signal.resource.replay.deny-on-unknown-or-missing", 21),
    ]
    .into_iter()
    .map(|(id, kind, name, contract)| {
        built_in_resource_policy_registration(id, kind, name, contract)
    })
    .collect()
}
