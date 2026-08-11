use serde::Serialize;

use crate::data::resource::policy::{ResourceReplayDecisionClass, ResourceReplayDecisionPlan};
use crate::data::resource::policy_registry::{
    ResourcePolicyDescriptorId, ResourcePolicyDigest, ResourcePolicyKind,
};
use crate::data::resource::summary::ResourceBoundaryPerformanceEnvelope;

use super::report::ResourcePolicyCompatibilityReport;
use super::vocabulary::{
    ResourcePolicyCompatibilityClass, ResourcePolicyRestoreCompatibilityDenialClass,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ResourcePolicyRestoreCompatibilityProof {
    compatibility: ResourcePolicyCompatibilityReport,
    replay_decision_class: ResourceReplayDecisionClass,
    replay_decision_descriptor_id: ResourcePolicyDescriptorId,
    replay_decision_digest: ResourcePolicyDigest,
}

impl ResourcePolicyRestoreCompatibilityProof {
    pub(crate) fn from_compatibility(
        compatibility: ResourcePolicyCompatibilityReport,
        replay_decision_plan: &ResourceReplayDecisionPlan,
    ) -> Result<Self, ResourcePolicyCompatibilityReport> {
        if compatibility.is_compatible()
            && compatibility
                .families()
                .iter()
                .all(|family| replay_decision_plan.admits_compatible_class(family.class()))
        {
            Ok(Self {
                compatibility,
                replay_decision_class: replay_decision_plan.class(),
                replay_decision_descriptor_id: replay_decision_plan.descriptor_id(),
                replay_decision_digest: replay_decision_plan.decision_digest().clone(),
            })
        } else {
            Err(compatibility)
        }
    }

    pub fn compatibility(&self) -> &ResourcePolicyCompatibilityReport {
        &self.compatibility
    }
    pub fn compatibility_digest(&self) -> &ResourcePolicyDigest {
        self.compatibility.compatibility_digest()
    }
    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.compatibility.performance()
    }
    pub fn replay_decision_class(&self) -> ResourceReplayDecisionClass {
        self.replay_decision_class
    }
    pub fn replay_decision_descriptor_id(&self) -> ResourcePolicyDescriptorId {
        self.replay_decision_descriptor_id
    }
    pub fn replay_decision_digest(&self) -> &ResourcePolicyDigest {
        &self.replay_decision_digest
    }
    pub fn canonical_truth_preserved_width(&self) -> u32 {
        self.compatibility.canonical_truth_preserved_width()
    }
    pub fn retained_history_unavailable_width(&self) -> u32 {
        self.compatibility.retained_history_unavailable_width()
    }
    pub fn diagnostics_details_unavailable_width(&self) -> u32 {
        self.compatibility.diagnostics_details_unavailable_width()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeniedResourcePolicyRestoreCompatibility {
    class: ResourcePolicyRestoreCompatibilityDenialClass,
    primary_incompatible_kind: Option<ResourcePolicyKind>,
    compatibility: ResourcePolicyCompatibilityReport,
    replay_decision_class: ResourceReplayDecisionClass,
    replay_decision_descriptor_id: ResourcePolicyDescriptorId,
    replay_decision_digest: ResourcePolicyDigest,
}

impl DeniedResourcePolicyRestoreCompatibility {
    pub(crate) fn from_compatibility(
        compatibility: ResourcePolicyCompatibilityReport,
        replay_decision_plan: &ResourceReplayDecisionPlan,
    ) -> Self {
        let incompatibilities: Vec<_> = compatibility
            .families()
            .iter()
            .filter(|family| !family.class().is_compatible())
            .collect();
        debug_assert!(
            !incompatibilities.is_empty(),
            "restore compatibility denial requires at least one incompatible family"
        );
        let primary_incompatible_kind = incompatibilities.first().map(|family| family.kind());
        let class = if incompatibilities.len() > 1 {
            ResourcePolicyRestoreCompatibilityDenialClass::MultipleIncompatibilities
        } else {
            match incompatibilities.first().map(|family| family.class()) {
                Some(ResourcePolicyCompatibilityClass::MissingDescriptor) => {
                    ResourcePolicyRestoreCompatibilityDenialClass::MissingDescriptor
                }
                Some(ResourcePolicyCompatibilityClass::VersionIncompatible) => {
                    ResourcePolicyRestoreCompatibilityDenialClass::VersionIncompatible
                }
                Some(ResourcePolicyCompatibilityClass::CompatibleParameterExpansion)
                | Some(ResourcePolicyCompatibilityClass::CompatibleRetentionNarrowing)
                | Some(ResourcePolicyCompatibilityClass::CompatibleDiagnosticsRichnessChange) => {
                    unreachable!("restore compatibility denial constructed from compatible report")
                }
                Some(ResourcePolicyCompatibilityClass::ParameterDigestDrift) => {
                    ResourcePolicyRestoreCompatibilityDenialClass::ParameterDigestDrift
                }
                Some(ResourcePolicyCompatibilityClass::DecisionSemanticsDrift) => {
                    ResourcePolicyRestoreCompatibilityDenialClass::DecisionSemanticsDrift
                }
                Some(ResourcePolicyCompatibilityClass::ExactDescriptorMatch) | None => {
                    unreachable!("restore compatibility denial constructed from compatible report")
                }
            }
        };

        Self {
            class,
            primary_incompatible_kind,
            compatibility,
            replay_decision_class: replay_decision_plan.class(),
            replay_decision_descriptor_id: replay_decision_plan.descriptor_id(),
            replay_decision_digest: replay_decision_plan.decision_digest().clone(),
        }
    }

    pub(crate) fn from_replay_policy_gate(
        compatibility: ResourcePolicyCompatibilityReport,
        replay_decision_plan: &ResourceReplayDecisionPlan,
        primary_incompatible_kind: ResourcePolicyKind,
    ) -> Self {
        Self {
            class:
                ResourcePolicyRestoreCompatibilityDenialClass::ReplayPolicyDisallowsCompatibleDrift,
            primary_incompatible_kind: Some(primary_incompatible_kind),
            compatibility,
            replay_decision_class: replay_decision_plan.class(),
            replay_decision_descriptor_id: replay_decision_plan.descriptor_id(),
            replay_decision_digest: replay_decision_plan.decision_digest().clone(),
        }
    }

    pub fn class(&self) -> ResourcePolicyRestoreCompatibilityDenialClass {
        self.class
    }
    pub fn primary_incompatible_kind(&self) -> Option<ResourcePolicyKind> {
        self.primary_incompatible_kind
    }
    pub fn compatibility(&self) -> &ResourcePolicyCompatibilityReport {
        &self.compatibility
    }
    pub fn compatibility_digest(&self) -> &ResourcePolicyDigest {
        self.compatibility.compatibility_digest()
    }
    pub fn replay_decision_class(&self) -> ResourceReplayDecisionClass {
        self.replay_decision_class
    }
    pub fn replay_decision_descriptor_id(&self) -> ResourcePolicyDescriptorId {
        self.replay_decision_descriptor_id
    }
    pub fn replay_decision_digest(&self) -> &ResourcePolicyDigest {
        &self.replay_decision_digest
    }
    pub fn incompatible_width(&self) -> u32 {
        self.compatibility.incompatible_width()
    }
    pub fn performance(&self) -> ResourceBoundaryPerformanceEnvelope {
        self.compatibility.performance()
    }
}
