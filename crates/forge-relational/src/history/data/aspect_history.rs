use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{json, to_value, Value};
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
#[non_exhaustive]
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
#[non_exhaustive]
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
#[non_exhaustive]
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
    pub returned_entries: u64,
    pub traversed_commits: u64,
    pub traversed_lineage_events: u64,
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
#[must_use]
pub struct AspectHistoryQueryResult {
    pub entries: Vec<AspectHistoryEntry>,
    pub trace: AspectHistoryResolutionTrace,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct LineageAspectHistoryQueryResult {
    pub history: Option<LineageAspectHistory>,
    pub trace: AspectHistoryResolutionTrace,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct AspectHistoryDigest {
    pub requested_target: HistoryAspectQueryTarget,
    pub branch_id: BranchId,
    pub resolved_aspects: CanonicalAspectSet,
    pub entry_count: u64,
    pub degraded_precision_entry_count: u64,
    pub traversed_commits: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[must_use]
pub struct LineageAspectResolutionDigest {
    pub requested_target: HistoryAspectQueryTarget,
    pub branch_id: BranchId,
    pub resolved_aspects: CanonicalAspectSet,
    pub entry_count: u64,
    pub degraded_precision_entry_count: u64,
    pub traversed_commits: u64,
    pub traversed_lineage_events: u64,
    pub resolved_lineage_chain_len: u64,
}

impl AspectFilter {
    pub fn matches(&self, aspects: &CanonicalAspectSet) -> bool {
        match self.mode {
            AspectFilterMode::Any => intersects_sorted(&self.aspects, aspects),
            AspectFilterMode::All => contains_all_sorted(&self.aspects, aspects),
        }
    }
}

fn intersects_sorted(requested: &RequestedAspectSet, actual: &CanonicalAspectSet) -> bool {
    let mut requested = requested.iter().peekable();
    let mut actual = actual.iter().peekable();
    while let (Some(left), Some(right)) = (requested.peek(), actual.peek()) {
        match left.cmp(right) {
            std::cmp::Ordering::Equal => return true,
            std::cmp::Ordering::Less => {
                requested.next();
            }
            std::cmp::Ordering::Greater => {
                actual.next();
            }
        }
    }
    false
}

fn contains_all_sorted(requested: &RequestedAspectSet, actual: &CanonicalAspectSet) -> bool {
    let mut requested = requested.iter().peekable();
    let mut actual = actual.iter().peekable();
    while let Some(left) = requested.peek() {
        let Some(right) = actual.peek() else {
            return false;
        };
        match left.cmp(right) {
            std::cmp::Ordering::Equal => {
                requested.next();
                actual.next();
            }
            std::cmp::Ordering::Less => return false,
            std::cmp::Ordering::Greater => {
                actual.next();
            }
        }
    }
    true
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
        let fields = AspectHistoryResolutionTraceFields::from_trace(self);
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
                fields: trace_fields_value(&fields, "aspect history resolution trace"),
            }],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct AspectHistoryResolutionTraceFields {
    requested_target: HistoryAspectQueryTarget,
    branch_id: BranchId,
    filter_mode: Option<AspectFilterMode>,
    requested_aspects: Option<Vec<AspectKey>>,
    resolved_aspects: Vec<AspectKey>,
    searched_commit_span: Option<AspectHistoryCommitSpan>,
    searched_lineage_event_span: Option<AspectHistoryLineageEventSpan>,
    returned_entries: u64,
    traversed_commits: u64,
    traversed_lineage_events: u64,
}

impl AspectHistoryResolutionTraceFields {
    fn from_trace(trace: &AspectHistoryResolutionTrace) -> Self {
        Self {
            requested_target: trace.requested_target.clone(),
            branch_id: trace.branch_id.clone(),
            filter_mode: trace.filter.as_ref().map(|filter| filter.mode),
            requested_aspects: trace
                .filter
                .as_ref()
                .map(|filter| filter.aspects.iter().cloned().collect()),
            resolved_aspects: trace.resolved_aspects.iter().cloned().collect(),
            searched_commit_span: trace.searched_commit_span,
            searched_lineage_event_span: trace.searched_lineage_event_span,
            returned_entries: trace.returned_entries,
            traversed_commits: trace.traversed_commits,
            traversed_lineage_events: trace.traversed_lineage_events,
        }
    }
}

fn trace_fields_value<T>(fields: &T, trace_kind: &str) -> Value
where
    T: Serialize,
{
    match to_value(fields) {
        Ok(value) => value,
        Err(error) => json!({
            "trace_kind": trace_kind,
            "serialization_failure": error.to_string(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::history::data::{AspectHistoryDigest, BranchId, HistoryAspectQueryTarget};
    use crate::identity::data::{EntityId, PartitionId};
    use crate::publication::patch::data::CanonicalAspectSet;

    #[test]
    fn aspect_history_digest_counts_roundtrip_as_u64() {
        let digest = AspectHistoryDigest {
            requested_target: HistoryAspectQueryTarget::Entity(EntityId::new(PartitionId(1), 0, 1)),
            branch_id: BranchId("main".to_string()),
            resolved_aspects: CanonicalAspectSet::new([]),
            entry_count: u64::from(u32::MAX) + 17,
            degraded_precision_entry_count: 9,
            traversed_commits: u64::from(u32::MAX) + 23,
        };

        let encoded = serde_json::to_string(&digest).expect("serialize aspect history digest");
        let decoded: AspectHistoryDigest =
            serde_json::from_str(&encoded).expect("deserialize aspect history digest");

        assert_eq!(decoded, digest);
    }
}

impl AspectHistoryQueryResult {
    pub fn aspect_history_digest(&self) -> AspectHistoryDigest {
        AspectHistoryDigest {
            requested_target: self.trace.requested_target.clone(),
            branch_id: self.trace.branch_id.clone(),
            resolved_aspects: self.trace.resolved_aspects.clone(),
            entry_count: self.entries.len() as u64,
            degraded_precision_entry_count: self
                .entries
                .iter()
                .filter(|entry| entry.origin.contains_degraded_precision)
                .count() as u64,
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
                .map(|history| history.entries.len() as u64)
                .unwrap_or(0),
            degraded_precision_entry_count: self
                .history
                .as_ref()
                .map(|history| {
                    history
                        .entries
                        .iter()
                        .filter(|entry| entry.origin.contains_degraded_precision)
                        .count() as u64
                })
                .unwrap_or(0),
            traversed_commits: self.trace.traversed_commits,
            traversed_lineage_events: self.trace.traversed_lineage_events,
            resolved_lineage_chain_len: self
                .history
                .as_ref()
                .map(|history| history.resolved_lineage_chain.len() as u64)
                .unwrap_or(0),
        }
    }
}
