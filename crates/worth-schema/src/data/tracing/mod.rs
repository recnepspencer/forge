use std::collections::BTreeSet;

use forge_relational::facade::history::{BranchId, CommitId};
use forge_relational::facade::identity::VersionId;
use forge_relational::facade::snapshots::SnapshotId;
use forge_relational::facade::transactions::{CommitLog, CommitResult, TransactionId};
use forge_signal::facade::{
    diagnostics::{LineageArtifactId, ReplayCursor},
    NodeId as SignalNodeId,
};
use serde::{Deserialize, Serialize};

use crate::data::aspects::Aspect;
use crate::data::authority::{DerivedTruthBasisIdentity, MutationOrigin};

#[cfg(test)]
use forge_relational::facade::runtime::{RelationalReadView, RelationalRuntime};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceAvailability {
    Present,
    OmittedByPolicy,
    Unavailable,
}

impl Default for TraceAvailability {
    fn default() -> Self {
        Self::Unavailable
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraceWarning {
    pub code: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamedCounter {
    pub name: String,
    pub value: u64,
}

impl NamedCounter {
    pub fn new(name: impl Into<String>, value: u64) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct PerformanceAccounting {
    pub counters: Vec<NamedCounter>,
}

impl PerformanceAccounting {
    pub fn new(counters: impl IntoIterator<Item = NamedCounter>) -> Self {
        Self {
            counters: counters.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct IntegrityMarkers {
    pub branch_id: Option<BranchId>,
    pub touched_aspects: BTreeSet<Aspect>,
    pub authoritative_mutation_origin: Option<MutationOrigin>,
    pub truth_basis_identity: Option<DerivedTruthBasisIdentity>,
    pub precision_fallback_count: usize,
    pub precision_budget_fallback_count: usize,
}

impl IntegrityMarkers {
    pub fn new(
        branch_id: Option<BranchId>,
        touched_aspects: BTreeSet<Aspect>,
        authoritative_mutation_origin: Option<MutationOrigin>,
        truth_basis_identity: Option<DerivedTruthBasisIdentity>,
        precision_fallback_count: usize,
        precision_budget_fallback_count: usize,
    ) -> Self {
        Self {
            branch_id,
            touched_aspects,
            authoritative_mutation_origin,
            truth_basis_identity,
            precision_fallback_count,
            precision_budget_fallback_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityTraceEvidence {
    pub branch_id: BranchId,
    pub commit_count: usize,
    pub published_commit_count: usize,
    pub total_phase_count: usize,
    pub history_summary_count: usize,
    pub publication_summary_count: usize,
    pub invariant_result_count: usize,
    pub commit_logs: Vec<CommitLog>,
}

impl AuthorityTraceEvidence {
    pub fn from_commit_results(branch_id: BranchId, commits: &[CommitResult]) -> Self {
        Self::from_commit_logs(
            branch_id,
            commits
                .iter()
                .map(|commit| commit.commit_log().clone())
                .collect(),
        )
    }

    pub fn from_commit_logs(branch_id: BranchId, commit_logs: Vec<CommitLog>) -> Self {
        let commit_count = commit_logs.len();
        let published_commit_count = commit_logs
            .iter()
            .filter(|log| log.has_commit_published())
            .count();
        let total_phase_count = commit_logs
            .iter()
            .map(|log| log.summary().phase_count)
            .sum();
        let history_summary_count = commit_logs
            .iter()
            .filter(|log| log.history_summary_event().is_some())
            .count();
        let publication_summary_count = commit_logs
            .iter()
            .filter(|log| log.publication_summary_event().is_some())
            .count();
        let invariant_result_count = commit_logs
            .iter()
            .map(|log| log.summary().invariant_result_count)
            .sum();
        Self {
            branch_id,
            commit_count,
            published_commit_count,
            total_phase_count,
            history_summary_count,
            publication_summary_count,
            invariant_result_count,
            commit_logs,
        }
    }

    pub fn performance_accounting(&self) -> PerformanceAccounting {
        PerformanceAccounting::new([
            NamedCounter::new("authority.commit_count", self.commit_count as u64),
            NamedCounter::new(
                "authority.published_commit_count",
                self.published_commit_count as u64,
            ),
            NamedCounter::new("authority.total_phase_count", self.total_phase_count as u64),
            NamedCounter::new(
                "authority.history_summary_count",
                self.history_summary_count as u64,
            ),
            NamedCounter::new(
                "authority.publication_summary_count",
                self.publication_summary_count as u64,
            ),
            NamedCounter::new(
                "authority.invariant_result_count",
                self.invariant_result_count as u64,
            ),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityTraceAnchor {
    pub branch_id: BranchId,
    pub runtime_instance_ids: Vec<u64>,
    pub transaction_ids: Vec<TransactionId>,
    pub commit_ids: Vec<CommitId>,
    pub snapshot_ids: Vec<SnapshotId>,
    pub version_ids: Vec<VersionId>,
}

impl AuthorityTraceAnchor {
    pub fn from_commit_results(branch_id: BranchId, commits: &[CommitResult]) -> Self {
        Self {
            branch_id,
            runtime_instance_ids: commits
                .iter()
                .map(|commit| commit.snapshot.runtime_instance_id)
                .collect(),
            transaction_ids: commits.iter().map(|commit| commit.transaction_id).collect(),
            commit_ids: commits
                .iter()
                .map(|commit| commit.commit.commit_id)
                .collect(),
            snapshot_ids: commits
                .iter()
                .map(|commit| commit.snapshot.snapshot_id)
                .collect(),
            version_ids: commits.iter().map(|commit| commit.version_id).collect(),
        }
    }

    #[cfg(test)]
    pub fn latest_snapshot_id(&self) -> Option<SnapshotId> {
        self.snapshot_ids.last().copied()
    }

    #[cfg(test)]
    pub fn latest_runtime_instance_id(&self) -> Option<u64> {
        self.runtime_instance_ids.last().copied()
    }

    #[cfg(test)]
    pub fn latest_version_id(&self) -> Option<VersionId> {
        self.version_ids.last().copied()
    }

    #[cfg(test)]
    pub fn open_latest_snapshot(&self, runtime: &RelationalRuntime) -> Option<RelationalReadView> {
        let snapshot_id = self.latest_snapshot_id()?;
        let version_id = self.latest_version_id()?;
        let runtime_instance_id = self.latest_runtime_instance_id()?;
        let handle = forge_relational::facade::snapshots::SnapshotHandle {
            runtime_instance_id,
            snapshot_id,
            version_id,
            read_policy:
                forge_relational::facade::snapshots::SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        };
        runtime.read_truth().read_snapshot(&handle)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BridgeTraceEvidence {
    pub availability: TraceAvailability,
    pub route_identities: Vec<String>,
    pub invalidation_identities: Vec<String>,
    pub snapshot_identities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DerivedTraceEvidence {
    pub availability: TraceAvailability,
    pub invalidation_target_count: usize,
    pub fallback_classes: Vec<String>,
    pub equivalence_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalTraceEvidence {
    pub availability: TraceAvailability,
    pub explanation_availability: Option<String>,
    pub provenance_availability: Option<String>,
    pub replay_event_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BridgeTraceAnchor {
    pub route_identities: Vec<String>,
    pub invalidation_identities: Vec<String>,
    pub snapshot_identities: Vec<String>,
    pub historical_record_identities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedTraceAnchor {
    pub branch_id: BranchId,
    pub runtime_instance_id: u64,
    pub snapshot_id: SnapshotId,
    pub version_id: VersionId,
    pub truth_basis_identity: DerivedTruthBasisIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalTraceAnchor {
    pub node: SignalNodeId,
    pub replay_cursor: Option<ReplayCursor>,
    pub execution_record_id: Option<u64>,
    pub semantic_segment_id: Option<u64>,
    pub lineage_artifact_id: Option<LineageArtifactId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DecisionTrace {
    pub authority_anchor: Option<AuthorityTraceAnchor>,
    pub bridge_anchor: Option<BridgeTraceAnchor>,
    pub derived_anchor: Option<DerivedTraceAnchor>,
    pub signal_anchor: Option<SignalTraceAnchor>,
    pub authority: Option<AuthorityTraceEvidence>,
    pub bridge: Option<BridgeTraceEvidence>,
    pub derived: Option<DerivedTraceEvidence>,
    pub signal: Option<SignalTraceEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryEnvelope<T> {
    primary_result: T,
    warnings: Vec<TraceWarning>,
    decision_trace: DecisionTrace,
    integrity_markers: IntegrityMarkers,
    performance_accounting: PerformanceAccounting,
}

impl<T> BoundaryEnvelope<T> {
    pub fn success(
        primary_result: T,
        warnings: Vec<TraceWarning>,
        decision_trace: DecisionTrace,
        integrity_markers: IntegrityMarkers,
        performance_accounting: PerformanceAccounting,
    ) -> Self {
        Self {
            primary_result,
            warnings,
            decision_trace,
            integrity_markers,
            performance_accounting,
        }
    }

    #[cfg(test)]
    pub fn primary_result(&self) -> &T {
        &self.primary_result
    }

    pub fn into_primary_result(self) -> T {
        self.primary_result
    }

    #[cfg(test)]
    pub fn map_primary_result<U>(self, map: impl FnOnce(T) -> U) -> BoundaryEnvelope<U> {
        let (primary_result, warnings, decision_trace, integrity_markers, performance_accounting) =
            self.into_parts();
        BoundaryEnvelope::success(
            map(primary_result),
            warnings,
            decision_trace,
            integrity_markers,
            performance_accounting,
        )
    }

    #[cfg(test)]
    pub fn warnings(&self) -> &[TraceWarning] {
        &self.warnings
    }

    #[cfg(test)]
    pub fn decision_trace(&self) -> &DecisionTrace {
        &self.decision_trace
    }

    #[cfg(test)]
    pub fn integrity_markers(&self) -> &IntegrityMarkers {
        &self.integrity_markers
    }

    #[cfg(test)]
    pub fn performance_accounting(&self) -> &PerformanceAccounting {
        &self.performance_accounting
    }

    #[cfg(test)]
    pub fn into_parts(
        self,
    ) -> (
        T,
        Vec<TraceWarning>,
        DecisionTrace,
        IntegrityMarkers,
        PerformanceAccounting,
    ) {
        (
            self.primary_result,
            self.warnings,
            self.decision_trace,
            self.integrity_markers,
            self.performance_accounting,
        )
    }

    #[cfg(test)]
    pub fn with_performance_accounting(
        self,
        performance_accounting: PerformanceAccounting,
    ) -> Self {
        let (primary_result, warnings, decision_trace, integrity_markers, _) = self.into_parts();
        Self::success(
            primary_result,
            warnings,
            decision_trace,
            integrity_markers,
            performance_accounting,
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryFailure<E> {
    error: E,
    warnings: Vec<TraceWarning>,
    decision_trace: DecisionTrace,
    integrity_markers: IntegrityMarkers,
    performance_accounting: PerformanceAccounting,
}

impl<E> BoundaryFailure<E> {
    pub fn failure(
        error: E,
        warnings: Vec<TraceWarning>,
        decision_trace: DecisionTrace,
        integrity_markers: IntegrityMarkers,
        performance_accounting: PerformanceAccounting,
    ) -> Self {
        Self {
            error,
            warnings,
            decision_trace,
            integrity_markers,
            performance_accounting,
        }
    }

    #[cfg(test)]
    pub fn error(&self) -> &E {
        &self.error
    }

    pub fn into_error(self) -> E {
        self.error
    }

    #[cfg(test)]
    pub fn map_error<F>(self, map: impl FnOnce(E) -> F) -> BoundaryFailure<F> {
        let (error, warnings, decision_trace, integrity_markers, performance_accounting) =
            self.into_parts();
        BoundaryFailure::failure(
            map(error),
            warnings,
            decision_trace,
            integrity_markers,
            performance_accounting,
        )
    }

    #[cfg(test)]
    pub fn warnings(&self) -> &[TraceWarning] {
        &self.warnings
    }

    #[cfg(test)]
    pub fn into_parts(
        self,
    ) -> (
        E,
        Vec<TraceWarning>,
        DecisionTrace,
        IntegrityMarkers,
        PerformanceAccounting,
    ) {
        (
            self.error,
            self.warnings,
            self.decision_trace,
            self.integrity_markers,
            self.performance_accounting,
        )
    }
}

#[cfg(test)]
mod tests;
