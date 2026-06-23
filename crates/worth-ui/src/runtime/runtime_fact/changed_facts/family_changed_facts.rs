use super::WorthUiChangedRuntimeFacts;
use crate::runtime::{WorthUiCapabilityReloadEvidence, WorthUiValidationReloadEvidence};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCapabilityChangedFacts {
    changed_facts: WorthUiChangedRuntimeFacts,
    active_snapshot_digest_before: u64,
    active_snapshot_digest_after: u64,
}

impl WorthUiCapabilityChangedFacts {
    pub(crate) fn from_admitted_capability_reload(
        facts: crate::runtime::WorthUiRuntimeFactSet,
        active_snapshot_digest_before: u64,
        active_snapshot_digest_after: u64,
    ) -> Self {
        Self {
            changed_facts: WorthUiChangedRuntimeFacts::from_runtime(facts),
            active_snapshot_digest_before,
            active_snapshot_digest_after,
        }
    }

    pub(crate) fn from_reload_evidence(evidence: &WorthUiCapabilityReloadEvidence) -> Self {
        evidence.capability_changed_facts().clone()
    }

    pub fn changed_facts(&self) -> &WorthUiChangedRuntimeFacts {
        &self.changed_facts
    }

    pub fn active_snapshot_digest_before(&self) -> u64 {
        self.active_snapshot_digest_before
    }

    pub fn active_snapshot_digest_after(&self) -> u64 {
        self.active_snapshot_digest_after
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryBindingChangedFacts {
    changed_facts: WorthUiChangedRuntimeFacts,
    active_artifact_digest_before: u64,
    candidate_artifact_digest_after: u64,
}

impl WorthUiQueryBindingChangedFacts {
    pub(crate) fn from_comparison_facts(
        facts: crate::runtime::WorthUiRuntimeFactSet,
        active_artifact_digest_before: u64,
        candidate_artifact_digest_after: u64,
    ) -> Self {
        Self {
            changed_facts: WorthUiChangedRuntimeFacts::from_runtime(facts),
            active_artifact_digest_before,
            candidate_artifact_digest_after,
        }
    }

    pub fn changed_facts(&self) -> &WorthUiChangedRuntimeFacts {
        &self.changed_facts
    }

    pub fn active_artifact_digest_before(&self) -> u64 {
        self.active_artifact_digest_before
    }

    pub fn candidate_artifact_digest_after(&self) -> u64 {
        self.candidate_artifact_digest_after
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiValidationChangedFacts {
    changed_facts: WorthUiChangedRuntimeFacts,
    active_artifact_digest_before: u64,
    active_artifact_digest_after: u64,
    active_plan_digest_before: u64,
    active_plan_digest_after: u64,
}

impl WorthUiValidationChangedFacts {
    pub(crate) fn from_reload_evidence(evidence: &WorthUiValidationReloadEvidence) -> Self {
        Self {
            changed_facts: WorthUiChangedRuntimeFacts::from_runtime(
                evidence.changed_facts().clone(),
            ),
            active_artifact_digest_before: evidence.active_artifact_digest_before(),
            active_artifact_digest_after: evidence.active_artifact_digest_after(),
            active_plan_digest_before: evidence.active_plan_digest_before(),
            active_plan_digest_after: evidence.active_plan_digest_after(),
        }
    }

    pub fn changed_facts(&self) -> &WorthUiChangedRuntimeFacts {
        &self.changed_facts
    }

    pub fn active_artifact_digest_before(&self) -> u64 {
        self.active_artifact_digest_before
    }

    pub fn active_artifact_digest_after(&self) -> u64 {
        self.active_artifact_digest_after
    }

    pub fn active_plan_digest_before(&self) -> u64 {
        self.active_plan_digest_before
    }

    pub fn active_plan_digest_after(&self) -> u64 {
        self.active_plan_digest_after
    }
}
