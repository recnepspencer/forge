use std::collections::BTreeMap;

use super::counters::PlanarBooleanLoopDecisionLogCounters;
use super::row::PlanarBooleanLoopDecisionRow;
use super::vocabulary::{
    PlanarBooleanLoopDecisionAffectedArtifact, PlanarBooleanLoopDecisionKind,
    PlanarBooleanLoopDecisionPhase,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopDecisionLookupIndex {
    decision_indexes: BTreeMap<String, usize>,
    artifact_indexes: BTreeMap<String, Vec<usize>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanLoopFailureLocalization {
    phase: PlanarBooleanLoopDecisionPhase,
    kind: PlanarBooleanLoopDecisionKind,
    affected_artifact: PlanarBooleanLoopDecisionAffectedArtifact,
    affected_artifact_identity: String,
    policy_or_denial_kind: Option<String>,
    human_reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanStructuredLoopReconstructionFailureReport {
    localization: PlanarBooleanLoopFailureLocalization,
    related_decision_identities: Vec<String>,
}

impl PlanarBooleanLoopDecisionLookupIndex {
    pub(crate) fn build(
        rows: &[PlanarBooleanLoopDecisionRow],
        counters: &mut PlanarBooleanLoopDecisionLogCounters,
    ) -> Self {
        let mut decision_indexes = BTreeMap::new();
        let mut artifact_indexes: BTreeMap<String, Vec<usize>> = BTreeMap::new();
        for (index, row) in rows.iter().enumerate() {
            decision_indexes.insert(row.decision_identity().to_string(), index);
            artifact_indexes
                .entry(row.affected_artifact_identity().to_string())
                .or_default()
                .push(index);
        }
        counters.indexed_lookup_entries(rows.len());
        Self {
            decision_indexes,
            artifact_indexes,
        }
    }

    pub(crate) fn decision_index(&self, decision_identity: &str) -> Option<usize> {
        self.decision_indexes.get(decision_identity).copied()
    }

    pub(crate) fn artifact_indexes(&self, artifact_identity: &str) -> &[usize] {
        self.artifact_indexes
            .get(artifact_identity)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }
}

impl PlanarBooleanLoopFailureLocalization {
    pub(crate) fn from_row(row: &PlanarBooleanLoopDecisionRow) -> Self {
        Self {
            phase: row.phase(),
            kind: row.kind(),
            affected_artifact: row.affected_artifact(),
            affected_artifact_identity: row.affected_artifact_identity().to_string(),
            policy_or_denial_kind: row.policy_or_denial_kind().map(str::to_string),
            human_reason: row.human_reason().to_string(),
        }
    }

    pub fn phase(&self) -> PlanarBooleanLoopDecisionPhase {
        self.phase
    }

    pub fn kind(&self) -> PlanarBooleanLoopDecisionKind {
        self.kind
    }

    pub fn affected_artifact(&self) -> PlanarBooleanLoopDecisionAffectedArtifact {
        self.affected_artifact
    }

    pub fn affected_artifact_identity(&self) -> &str {
        &self.affected_artifact_identity
    }

    pub fn policy_or_denial_kind(&self) -> Option<&str> {
        self.policy_or_denial_kind.as_deref()
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}

impl PlanarBooleanStructuredLoopReconstructionFailureReport {
    pub(crate) fn from_localization(
        localization: PlanarBooleanLoopFailureLocalization,
        related_decision_identities: Vec<String>,
    ) -> Self {
        Self {
            localization,
            related_decision_identities,
        }
    }

    pub fn localization(&self) -> &PlanarBooleanLoopFailureLocalization {
        &self.localization
    }

    pub fn related_decision_identities(&self) -> &[String] {
        &self.related_decision_identities
    }
}
