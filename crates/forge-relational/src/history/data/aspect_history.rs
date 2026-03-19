use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::json;
use smallvec::SmallVec;

use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::identity::data::{EntityId, LineageId, RelationId, VersionId};
use crate::publication::patch::data::{AspectKey, CanonicalAspectSet, RecordStructuralChange};
use crate::transactions::data::RecordRef;

use super::{BranchId, CommitId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectFilterMode {
    Any,
    All,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectFilter {
    pub mode: AspectFilterMode,
    pub aspects: RequestedAspectSet,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestedAspectSet(SmallVec<[AspectKey; 4]>);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectHistoryOrigin {
    pub commit_id: CommitId,
    pub version_id: VersionId,
    pub branch_id: BranchId,
    pub target: RecordRef,
    pub structural_change: RecordStructuralChange,
    pub changed_aspects: CanonicalAspectSet,
    pub contains_degraded_precision: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AspectResolutionContext {
    DirectRecordHistory,
    ResolvedViaLineage {
        start_lineage_id: LineageId,
        traversed_event_ids: Vec<u64>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectHistoryEntry {
    pub origin: AspectHistoryOrigin,
    pub resolution: AspectResolutionContext,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageAspectHistory {
    pub requested_branch: BranchId,
    pub start_lineage_id: LineageId,
    pub resolved_lineage_chain: Vec<LineageId>,
    pub entries: Vec<AspectHistoryEntry>,
    pub traversed_event_ids: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HistoryAspectQueryTarget {
    Entity(EntityId),
    Relation(RelationId),
    Lineage(LineageId),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectHistoryResolutionTrace {
    pub requested_target: HistoryAspectQueryTarget,
    pub branch_id: BranchId,
    pub filter: Option<AspectFilter>,
    pub resolved_aspects: CanonicalAspectSet,
    pub searched_commit_span: Option<AspectHistoryCommitSpan>,
    pub searched_lineage_event_span: Option<AspectHistoryLineageEventSpan>,
    pub returned_entries: usize,
    pub traversed_commits: usize,
    pub traversed_lineage_events: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectHistoryCommitSpan {
    pub first_commit_id: CommitId,
    pub last_commit_id: CommitId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectHistoryLineageEventSpan {
    pub first_event_id: u64,
    pub last_event_id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectHistoryQueryResult {
    pub entries: Vec<AspectHistoryEntry>,
    pub trace: AspectHistoryResolutionTrace,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageAspectHistoryQueryResult {
    pub history: Option<LineageAspectHistory>,
    pub trace: AspectHistoryResolutionTrace,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AspectHistoryDigest {
    pub requested_target: HistoryAspectQueryTarget,
    pub branch_id: BranchId,
    pub resolved_aspects: CanonicalAspectSet,
    pub entry_count: usize,
    pub degraded_precision_entry_count: usize,
    pub traversed_commits: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageAspectResolutionDigest {
    pub requested_target: HistoryAspectQueryTarget,
    pub branch_id: BranchId,
    pub resolved_aspects: CanonicalAspectSet,
    pub entry_count: usize,
    pub degraded_precision_entry_count: usize,
    pub traversed_commits: usize,
    pub traversed_lineage_events: usize,
    pub resolved_lineage_chain_len: usize,
}

impl AspectFilter {
    pub fn matches(&self, aspects: &CanonicalAspectSet) -> bool {
        match self.mode {
            AspectFilterMode::Any => self
                .aspects
                .iter()
                .any(|requested| aspects.iter().any(|actual| actual == requested)),
            AspectFilterMode::All => self
                .aspects
                .iter()
                .all(|requested| aspects.iter().any(|actual| actual == requested)),
        }
    }
}

impl RequestedAspectSet {
    pub fn new(aspects: impl IntoIterator<Item = AspectKey>) -> Self {
        let mut aspects = aspects.into_iter().collect::<SmallVec<[AspectKey; 4]>>();
        if !aspects.windows(2).all(|window| window[0] < window[1]) {
            aspects.sort();
            aspects.dedup();
        }
        Self(aspects)
    }

    pub fn empty() -> Self {
        Self(SmallVec::new())
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &AspectKey> {
        self.0.iter()
    }
}

impl Default for RequestedAspectSet {
    fn default() -> Self {
        Self::empty()
    }
}

impl From<Vec<AspectKey>> for RequestedAspectSet {
    fn from(value: Vec<AspectKey>) -> Self {
        Self::new(value)
    }
}

impl Serialize for RequestedAspectSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.as_slice().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RequestedAspectSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let aspects = Vec::<AspectKey>::deserialize(deserializer)?;
        Ok(Self::new(aspects))
    }
}

impl AspectHistoryResolutionTrace {
    pub fn diagnostics_scope(&self) -> DiagnosticsScope {
        match self.requested_target {
            HistoryAspectQueryTarget::Lineage(_) => DiagnosticsScope::Lineage,
            HistoryAspectQueryTarget::Entity(_) | HistoryAspectQueryTarget::Relation(_) => {
                DiagnosticsScope::History
            }
        }
    }

    pub fn diagnostic_artifact(&self) -> RelationalDiagnosticArtifact {
        RelationalDiagnosticArtifact {
            scope: self.diagnostics_scope(),
            kind: DiagnosticsArtifactKind::DetailedTrace,
            determinism: DeterminismExpectation::Required,
            entries: vec![RelationalDiagnosticsEntry {
                code: match self.requested_target {
                    HistoryAspectQueryTarget::Lineage(_) => {
                        DiagnosticCode::LineageAspectHistoryResolved
                    }
                    HistoryAspectQueryTarget::Entity(_) | HistoryAspectQueryTarget::Relation(_) => {
                        DiagnosticCode::AspectHistoryResolved
                    }
                },
                message: "aspect history resolved from committed aspect truth".to_string(),
                fields: json!({
                    "requested_target": self.requested_target,
                    "branch_id": self.branch_id,
                    "filter_mode": self.filter.as_ref().map(|filter| filter.mode),
                    "requested_aspects": self.filter.as_ref().map(|filter| {
                        filter
                            .aspects
                            .iter()
                            .cloned()
                            .collect::<Vec<AspectKey>>()
                    }),
                    "resolved_aspects": self
                        .resolved_aspects
                        .iter()
                        .cloned()
                        .collect::<Vec<AspectKey>>(),
                    "searched_commit_span": self.searched_commit_span,
                    "searched_lineage_event_span": self.searched_lineage_event_span,
                    "returned_entries": self.returned_entries,
                    "traversed_commits": self.traversed_commits,
                    "traversed_lineage_events": self.traversed_lineage_events,
                }),
            }],
        }
    }
}

impl AspectHistoryQueryResult {
    pub fn aspect_history_digest(&self) -> AspectHistoryDigest {
        AspectHistoryDigest {
            requested_target: self.trace.requested_target.clone(),
            branch_id: self.trace.branch_id.clone(),
            resolved_aspects: self.trace.resolved_aspects.clone(),
            entry_count: self.entries.len(),
            degraded_precision_entry_count: self
                .entries
                .iter()
                .filter(|entry| entry.origin.contains_degraded_precision)
                .count(),
            traversed_commits: self.trace.traversed_commits,
        }
    }
}

impl LineageAspectHistoryQueryResult {
    pub fn lineage_aspect_resolution_digest(&self) -> LineageAspectResolutionDigest {
        LineageAspectResolutionDigest {
            requested_target: self.trace.requested_target.clone(),
            branch_id: self.trace.branch_id.clone(),
            resolved_aspects: self.trace.resolved_aspects.clone(),
            entry_count: self
                .history
                .as_ref()
                .map(|history| history.entries.len())
                .unwrap_or(0),
            degraded_precision_entry_count: self
                .history
                .as_ref()
                .map(|history| {
                    history
                        .entries
                        .iter()
                        .filter(|entry| entry.origin.contains_degraded_precision)
                        .count()
                })
                .unwrap_or(0),
            traversed_commits: self.trace.traversed_commits,
            traversed_lineage_events: self.trace.traversed_lineage_events,
            resolved_lineage_chain_len: self
                .history
                .as_ref()
                .map(|history| history.resolved_lineage_chain.len())
                .unwrap_or(0),
        }
    }
}
