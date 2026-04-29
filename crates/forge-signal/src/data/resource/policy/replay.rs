use serde::Serialize;

use crate::data::resource::policy::ResourcePolicyCompatibilityClass;
use crate::data::resource::policy_registry::{
    FrozenResourcePolicyDescriptor, ResourcePolicyDescriptorId, ResourcePolicyDigest,
    ResourcePolicyKind, ResourcePolicyResolutionError,
};

use super::ResourceReplayPolicyDeclaration;

const IDENTICAL_ONLY_NAME: &str = "signal.resource.replay.identical-only";
const COMPATIBLE_PARAMETER_EXPANSION_NAME: &str =
    "signal.resource.replay.compatible-parameter-expansion";
const COMPATIBLE_RETENTION_NARROWING_NAME: &str =
    "signal.resource.replay.compatible-retention-narrowing";
const COMPATIBLE_DIAGNOSTICS_RICHNESS_CHANGE_NAME: &str =
    "signal.resource.replay.compatible-diagnostics-richness-change";
const COMPATIBLE_PARAMETER_RETENTION_AND_DIAGNOSTICS_NAME: &str =
    "signal.resource.replay.compatible-parameter-expansion-and-retention-narrowing-and-diagnostics-richness-change";
const COMPATIBLE_RETENTION_AND_DIAGNOSTICS_NAME: &str =
    "signal.resource.replay.compatible-retention-narrowing-and-diagnostics-richness-change";
const DENY_ON_UNKNOWN_OR_MISSING_NAME: &str = "signal.resource.replay.deny-on-unknown-or-missing";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ResourceReplayDecisionClass {
    IdenticalOnly,
    CompatibleParameterExpansion,
    CompatibleRetentionNarrowing,
    CompatibleDiagnosticsRichnessChange,
    CompatibleParameterExpansionAndRetentionNarrowingAndDiagnosticsRichnessChange,
    CompatibleRetentionNarrowingAndDiagnosticsRichnessChange,
    DenyOnUnknownOrMissing,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourceReplayDecisionPlan {
    descriptor_id: ResourcePolicyDescriptorId,
    semantic_name: String,
    class: ResourceReplayDecisionClass,
    decision_digest: ResourcePolicyDigest,
}

impl ResourceReplayDecisionPlan {
    pub(crate) fn lower(
        declaration: &ResourceReplayPolicyDeclaration,
        frozen: &FrozenResourcePolicyDescriptor,
    ) -> Result<Self, ResourcePolicyResolutionError> {
        match declaration {
            ResourceReplayPolicyDeclaration::IdenticalOnly => {
                ensure_descriptor_name(frozen, IDENTICAL_ONLY_NAME, "identical-only replay policy")?;
                Ok(Self::new(frozen, ResourceReplayDecisionClass::IdenticalOnly))
            }
            ResourceReplayPolicyDeclaration::CompatibleParameterExpansion => {
                ensure_descriptor_name(
                    frozen,
                    COMPATIBLE_PARAMETER_EXPANSION_NAME,
                    "compatible-parameter-expansion replay policy",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceReplayDecisionClass::CompatibleParameterExpansion,
                ))
            }
            ResourceReplayPolicyDeclaration::CompatibleRetentionNarrowing => {
                ensure_descriptor_name(
                    frozen,
                    COMPATIBLE_RETENTION_NARROWING_NAME,
                    "compatible-retention-narrowing replay policy",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceReplayDecisionClass::CompatibleRetentionNarrowing,
                ))
            }
            ResourceReplayPolicyDeclaration::CompatibleDiagnosticsRichnessChange => {
                ensure_descriptor_name(
                    frozen,
                    COMPATIBLE_DIAGNOSTICS_RICHNESS_CHANGE_NAME,
                    "compatible-diagnostics-richness-change replay policy",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceReplayDecisionClass::CompatibleDiagnosticsRichnessChange,
                ))
            }
            ResourceReplayPolicyDeclaration::CompatibleParameterExpansionAndRetentionNarrowingAndDiagnosticsRichnessChange => {
                ensure_descriptor_name(
                    frozen,
                    COMPATIBLE_PARAMETER_RETENTION_AND_DIAGNOSTICS_NAME,
                    "compatible parameter, retention, and diagnostics replay policy",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceReplayDecisionClass::CompatibleParameterExpansionAndRetentionNarrowingAndDiagnosticsRichnessChange,
                ))
            }
            ResourceReplayPolicyDeclaration::CompatibleRetentionNarrowingAndDiagnosticsRichnessChange => {
                ensure_descriptor_name(
                    frozen,
                    COMPATIBLE_RETENTION_AND_DIAGNOSTICS_NAME,
                    "compatible retention and diagnostics replay policy",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceReplayDecisionClass::CompatibleRetentionNarrowingAndDiagnosticsRichnessChange,
                ))
            }
            ResourceReplayPolicyDeclaration::DenyOnUnknownOrMissing => {
                ensure_descriptor_name(
                    frozen,
                    DENY_ON_UNKNOWN_OR_MISSING_NAME,
                    "deny-on-unknown-or-missing replay policy",
                )?;
                Ok(Self::new(
                    frozen,
                    ResourceReplayDecisionClass::DenyOnUnknownOrMissing,
                ))
            }
            ResourceReplayPolicyDeclaration::Named { name } => Err(
                ResourcePolicyResolutionError::UnsupportedExecutablePolicy {
                    kind: ResourcePolicyKind::Replay,
                    name: name.clone(),
                    reason: "named replay policies are descriptor-only in the first ship runtime",
                },
            ),
        }
    }

    fn new(frozen: &FrozenResourcePolicyDescriptor, class: ResourceReplayDecisionClass) -> Self {
        let decision_digest = ResourcePolicyDigest::new(format!(
            "resource-policy-replay-plan:{}:{}",
            frozen.frozen_digest().as_str(),
            class.as_str()
        ));
        Self {
            descriptor_id: frozen.descriptor().id(),
            semantic_name: frozen.descriptor().semantic_name().as_str().to_owned(),
            class,
            decision_digest,
        }
    }

    pub fn descriptor_id(&self) -> ResourcePolicyDescriptorId {
        self.descriptor_id
    }

    pub fn semantic_name(&self) -> &str {
        &self.semantic_name
    }

    pub fn class(&self) -> ResourceReplayDecisionClass {
        self.class
    }

    pub fn decision_digest(&self) -> &ResourcePolicyDigest {
        &self.decision_digest
    }

    pub fn admits_compatible_class(&self, class: ResourcePolicyCompatibilityClass) -> bool {
        match class {
            ResourcePolicyCompatibilityClass::ExactDescriptorMatch => true,
            ResourcePolicyCompatibilityClass::CompatibleParameterExpansion => matches!(
                self.class,
                ResourceReplayDecisionClass::CompatibleParameterExpansion
                    | ResourceReplayDecisionClass::CompatibleParameterExpansionAndRetentionNarrowingAndDiagnosticsRichnessChange
            ),
            ResourcePolicyCompatibilityClass::CompatibleRetentionNarrowing => matches!(
                self.class,
                ResourceReplayDecisionClass::CompatibleRetentionNarrowing
                    | ResourceReplayDecisionClass::CompatibleParameterExpansionAndRetentionNarrowingAndDiagnosticsRichnessChange
                    | ResourceReplayDecisionClass::CompatibleRetentionNarrowingAndDiagnosticsRichnessChange
            ),
            ResourcePolicyCompatibilityClass::CompatibleDiagnosticsRichnessChange => matches!(
                self.class,
                ResourceReplayDecisionClass::CompatibleDiagnosticsRichnessChange
                    | ResourceReplayDecisionClass::CompatibleParameterExpansionAndRetentionNarrowingAndDiagnosticsRichnessChange
                    | ResourceReplayDecisionClass::CompatibleRetentionNarrowingAndDiagnosticsRichnessChange
            ),
            ResourcePolicyCompatibilityClass::MissingDescriptor
            | ResourcePolicyCompatibilityClass::VersionIncompatible
            | ResourcePolicyCompatibilityClass::ParameterDigestDrift
            | ResourcePolicyCompatibilityClass::DecisionSemanticsDrift => false,
        }
    }
}

impl ResourceReplayDecisionClass {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::IdenticalOnly => "identical-only",
            Self::CompatibleParameterExpansion => "compatible-parameter-expansion",
            Self::CompatibleRetentionNarrowing => "compatible-retention-narrowing",
            Self::CompatibleDiagnosticsRichnessChange => "compatible-diagnostics-richness-change",
            Self::CompatibleParameterExpansionAndRetentionNarrowingAndDiagnosticsRichnessChange => {
                "compatible-parameter-expansion-and-retention-narrowing-and-diagnostics-richness-change"
            }
            Self::CompatibleRetentionNarrowingAndDiagnosticsRichnessChange => {
                "compatible-retention-narrowing-and-diagnostics-richness-change"
            }
            Self::DenyOnUnknownOrMissing => "deny-on-unknown-or-missing",
        }
    }

    pub fn denies_unavailable_history(self) -> bool {
        matches!(self, Self::DenyOnUnknownOrMissing)
    }
}

fn ensure_descriptor_name(
    frozen: &FrozenResourcePolicyDescriptor,
    expected: &str,
    reason: &'static str,
) -> Result<(), ResourcePolicyResolutionError> {
    if frozen.descriptor().semantic_name().as_str() == expected {
        return Ok(());
    }
    Err(ResourcePolicyResolutionError::UnsupportedExecutablePolicy {
        kind: ResourcePolicyKind::Replay,
        name: frozen.descriptor().semantic_name().clone(),
        reason,
    })
}
