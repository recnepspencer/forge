use crate::domain_artifacts::core_artifact::{
    impl_hadwiger_artifact, require_non_empty, HadwigerArtifactAuthorityOwner,
    HadwigerArtifactCore, HadwigerArtifactKind, HadwigerArtifactReference,
    HadwigerArtifactShapeError, HadwigerArtifactSourceReference,
};
use crate::domain_artifacts::digest_basis::{artifact_core, HadwigerArtifactPayloadEntry};

use super::source::AgentSourceRecord;
use super::suggestions::{
    AgentExperimentProposal, AgentInvariantHypothesisSuggestion, AgentMotifSuggestion,
    AgentRepairSuggestion,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum AgentBatchEntry {
    Motif(AgentMotifSuggestion),
    Invariant(AgentInvariantHypothesisSuggestion),
    Experiment(AgentExperimentProposal),
    Repair(AgentRepairSuggestion),
}

impl AgentBatchEntry {
    pub(crate) fn cited_references(&self) -> Vec<&HadwigerArtifactReference> {
        match self {
            Self::Motif(value) => vec![value.cited_evidence()],
            Self::Invariant(value) => vec![value.cited_evidence()],
            Self::Experiment(value) => vec![value.target_artifact()],
            Self::Repair(value) => vec![value.cited_evidence()],
        }
    }

    pub(crate) fn stable_token(&self) -> String {
        match self {
            Self::Motif(value) => {
                let (id, kind, refs, detail, promotion) = value.clone().into_advisory_parts();
                entry_token(&id, kind.as_str(), &refs, &detail, promotion.as_str())
            }
            Self::Invariant(value) => {
                let (id, kind, refs, detail, promotion) = value.clone().into_advisory_parts();
                entry_token(&id, kind.as_str(), &refs, &detail, promotion.as_str())
            }
            Self::Experiment(value) => {
                let (id, kind, refs, detail, promotion) = value.clone().into_advisory_parts();
                entry_token(&id, kind.as_str(), &refs, &detail, promotion.as_str())
            }
            Self::Repair(value) => {
                let (id, kind, refs, detail, promotion) = value.clone().into_advisory_parts();
                entry_token(&id, kind.as_str(), &refs, &detail, promotion.as_str())
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentExplorationBatch {
    core: HadwigerArtifactCore,
    batch_id: String,
    source: AgentSourceRecord,
    entries: Vec<AgentBatchEntry>,
}

impl AgentExplorationBatch {
    pub fn builder(
        batch_id: impl Into<String>,
        source: AgentSourceRecord,
    ) -> AgentExplorationBatchBuilder {
        AgentExplorationBatchBuilder {
            batch_id: batch_id.into(),
            source,
            entries: Vec::new(),
        }
    }

    pub fn batch_id(&self) -> &str {
        &self.batch_id
    }

    pub fn source(&self) -> &AgentSourceRecord {
        &self.source
    }

    pub(crate) fn entries(&self) -> &[AgentBatchEntry] {
        &self.entries
    }

    pub fn suggestion_count(&self) -> usize {
        self.entries.len()
    }

    pub fn admits_theorem_authority(&self) -> bool {
        false
    }
}

impl_hadwiger_artifact!(AgentExplorationBatch, core);

pub struct AgentExplorationBatchBuilder {
    batch_id: String,
    source: AgentSourceRecord,
    entries: Vec<AgentBatchEntry>,
}

impl AgentExplorationBatchBuilder {
    pub fn with_motif_suggestion(
        mut self,
        value: AgentMotifSuggestion,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        self.entries.push(AgentBatchEntry::Motif(value));
        Ok(self)
    }

    pub fn with_invariant_hypothesis_suggestion(
        mut self,
        value: AgentInvariantHypothesisSuggestion,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        self.entries.push(AgentBatchEntry::Invariant(value));
        Ok(self)
    }

    pub fn with_experiment_proposal(
        mut self,
        value: AgentExperimentProposal,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        self.entries.push(AgentBatchEntry::Experiment(value));
        Ok(self)
    }

    pub fn with_repair_suggestion(
        mut self,
        value: AgentRepairSuggestion,
    ) -> Result<Self, HadwigerArtifactShapeError> {
        self.entries.push(AgentBatchEntry::Repair(value));
        Ok(self)
    }

    pub fn finish(self) -> Result<AgentExplorationBatch, HadwigerArtifactShapeError> {
        let batch_id = require_non_empty(self.batch_id, "batch_id")?;
        if self.entries.is_empty() {
            return Err(HadwigerArtifactShapeError::EmptyField {
                field: "agent_batch_entries",
            });
        }
        let mut entries = self.entries;
        entries.sort_by_key(AgentBatchEntry::stable_token);
        entries.dedup();
        let core = artifact_core(
            HadwigerArtifactKind::AgentExplorationBatch,
            HadwigerArtifactAuthorityOwner::AgentAdvisory,
            HadwigerArtifactSourceReference::AgentAdvisory {
                source_digest: self.source.source_digest(),
            },
            Vec::new(),
            batch_payload(&batch_id, &self.source, &entries),
        )?;
        Ok(AgentExplorationBatch {
            core,
            batch_id,
            source: self.source,
            entries,
        })
    }
}

fn batch_payload(
    batch_id: &str,
    source: &AgentSourceRecord,
    entries: &[AgentBatchEntry],
) -> Vec<HadwigerArtifactPayloadEntry> {
    let mut payload = vec![
        HadwigerArtifactPayloadEntry::text("batch_id", batch_id),
        HadwigerArtifactPayloadEntry::text("source", source.stable_token()),
    ];
    for (index, entry) in entries.iter().enumerate() {
        payload.push(HadwigerArtifactPayloadEntry::text(
            "entry",
            format!("{index:04}:{}", entry.stable_token()),
        ));
    }
    payload
}

fn entry_token(
    id: &str,
    kind: &str,
    refs: &[HadwigerArtifactReference],
    detail: &str,
    promotion: &str,
) -> String {
    let mut refs = refs
        .iter()
        .map(HadwigerArtifactReference::stable_token)
        .collect::<Vec<_>>();
    refs.sort();
    format!("{id}:{kind}:{}:{detail}:{promotion}", refs.join("|"))
}
