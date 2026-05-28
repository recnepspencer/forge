use std::collections::BTreeSet;

use forge_relational::facade::history::{BranchId, CommitId};
use forge_relational::facade::identity::VersionId;
use forge_relational::facade::snapshots::SnapshotId;
use forge_relational::facade::transactions::{CommitLog, CommitResult, TransactionId};
use schema::facade::platform::aspects::Aspect;
use schema::facade::platform::authority::{DerivedTopologyReadBasis, DerivedTruthBasisIdentity, MutationOrigin};
use serde::{Deserialize, Serialize};

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
        let commit_logs = commits
            .iter()
            .map(|commit| commit.commit_log().clone())
            .collect::<Vec<_>>();
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
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BridgeTraceAnchor {
    pub route_identities: Vec<String>,
    pub invalidation_identities: Vec<String>,
    pub snapshot_identities: Vec<String>,
    pub historical_record_identities: Vec<String>,
}

impl BridgeTraceAnchor {
    pub fn new(
        route_identities: Vec<String>,
        invalidation_identities: Vec<String>,
        snapshot_identities: Vec<String>,
        historical_record_identities: Vec<String>,
    ) -> Self {
        Self {
            route_identities,
            invalidation_identities,
            snapshot_identities,
            historical_record_identities,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DerivedTraceAnchor {
    pub branch_id: BranchId,
    pub runtime_instance_id: u64,
    pub snapshot_id: SnapshotId,
    pub version_id: VersionId,
    pub truth_basis_identity: DerivedTruthBasisIdentity,
}

impl DerivedTraceAnchor {
    pub fn from_read_basis(read_basis: &DerivedTopologyReadBasis) -> Self {
        Self {
            branch_id: read_basis.branch_id().clone(),
            runtime_instance_id: read_basis.snapshot().runtime_instance_id,
            snapshot_id: read_basis.snapshot().snapshot_id,
            version_id: read_basis.snapshot().version_id,
            truth_basis_identity: read_basis.authority.truth_basis_identity.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DerivedTraceEvidence {
    pub availability: TraceAvailability,
    pub invalidation_target_count: usize,
    pub fallback_classes: Vec<String>,
    pub equivalence_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct DecisionTrace {
    pub authority_anchor: Option<AuthorityTraceAnchor>,
    pub bridge_anchor: Option<BridgeTraceAnchor>,
    pub derived_anchor: Option<DerivedTraceAnchor>,
    pub signal_anchor: Option<()>,
    pub authority: Option<AuthorityTraceEvidence>,
    pub bridge: Option<()>,
    pub derived: Option<DerivedTraceEvidence>,
    pub signal: Option<()>,
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

    pub fn primary_result(&self) -> &T {
        &self.primary_result
    }

    pub fn into_primary_result(self) -> T {
        self.primary_result
    }

    pub fn warnings(&self) -> &[TraceWarning] {
        &self.warnings
    }

    pub fn decision_trace(&self) -> &DecisionTrace {
        &self.decision_trace
    }

    pub fn integrity_markers(&self) -> &IntegrityMarkers {
        &self.integrity_markers
    }

    pub fn performance_accounting(&self) -> &PerformanceAccounting {
        &self.performance_accounting
    }

    pub fn map_primary_result<U>(self, map: impl FnOnce(T) -> U) -> BoundaryEnvelope<U> {
        BoundaryEnvelope::success(
            map(self.primary_result),
            self.warnings,
            self.decision_trace,
            self.integrity_markers,
            self.performance_accounting,
        )
    }

    pub fn map_decision_trace(mut self, map: impl FnOnce(DecisionTrace) -> DecisionTrace) -> Self {
        self.decision_trace = map(self.decision_trace);
        self
    }

    pub fn with_integrity_markers(mut self, integrity_markers: IntegrityMarkers) -> Self {
        self.integrity_markers = integrity_markers;
        self
    }

    pub fn with_performance_accounting(
        mut self,
        performance_accounting: PerformanceAccounting,
    ) -> Self {
        self.performance_accounting = performance_accounting;
        self
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

    pub fn into_error(self) -> E {
        self.error
    }
}
