use std::collections::BTreeSet;

use forge_relational::facade::history::{BranchId, CommitId};
use forge_relational::facade::identity::VersionId;
use forge_relational::facade::runtime::{RelationalReadView, RelationalRuntime};
use forge_relational::facade::snapshots::SnapshotId;
use forge_relational::facade::transactions::{CommitLog, CommitResult, TransactionId};
use forge_signal::facade::{
    NodeId as SignalNodeId,
    SignalGraph,
    diagnostics::{
        DiagnosticsAvailability as SignalDiagnosticsAvailability, LineageArtifactId, ReplayCursor,
    },
};
use serde::{Deserialize, Serialize};

use crate::data::aspects::WorthAspect;
use crate::data::authority::{
    DerivedTopologyReadBasis, WorthDerivedTruthBasisIdentity, WorthMutationOrigin,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorthTraceAvailability {
    Present,
    OmittedByPolicy,
    Unavailable,
}

impl Default for WorthTraceAvailability {
    fn default() -> Self {
        Self::Unavailable
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthTraceWarning {
    pub code: String,
    pub detail: String,
}

impl WorthTraceWarning {
    pub fn new(code: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            detail: detail.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthNamedCounter {
    pub name: String,
    pub value: u64,
}

impl WorthNamedCounter {
    pub fn new(name: impl Into<String>, value: u64) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct WorthPerformanceAccounting {
    pub counters: Vec<WorthNamedCounter>,
}

impl WorthPerformanceAccounting {
    pub fn new(counters: impl IntoIterator<Item = WorthNamedCounter>) -> Self {
        Self {
            counters: counters.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct WorthIntegrityMarkers {
    pub branch_id: Option<BranchId>,
    pub touched_aspects: BTreeSet<WorthAspect>,
    pub authoritative_mutation_origin: Option<WorthMutationOrigin>,
    pub truth_basis_identity: Option<WorthDerivedTruthBasisIdentity>,
    pub precision_fallback_count: usize,
    pub precision_budget_fallback_count: usize,
}

impl WorthIntegrityMarkers {
    pub fn new(
        branch_id: Option<BranchId>,
        touched_aspects: BTreeSet<WorthAspect>,
        authoritative_mutation_origin: Option<WorthMutationOrigin>,
        truth_basis_identity: Option<WorthDerivedTruthBasisIdentity>,
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
pub struct WorthAuthorityTraceEvidence {
    pub branch_id: BranchId,
    pub commit_count: usize,
    pub published_commit_count: usize,
    pub total_phase_count: usize,
    pub history_summary_count: usize,
    pub publication_summary_count: usize,
    pub invariant_result_count: usize,
    pub commit_logs: Vec<CommitLog>,
}

impl WorthAuthorityTraceEvidence {
    pub fn from_commit_results(branch_id: BranchId, commits: &[CommitResult]) -> Self {
        Self::from_commit_logs(
            branch_id,
            commits.iter().map(|commit| commit.commit_log().clone()).collect(),
        )
    }

    pub fn from_commit_logs(branch_id: BranchId, commit_logs: Vec<CommitLog>) -> Self {
        let commit_count = commit_logs.len();
        let published_commit_count = commit_logs.iter().filter(|log| log.has_commit_published()).count();
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

    pub fn performance_accounting(&self) -> WorthPerformanceAccounting {
        WorthPerformanceAccounting::new([
            WorthNamedCounter::new("authority.commit_count", self.commit_count as u64),
            WorthNamedCounter::new(
                "authority.published_commit_count",
                self.published_commit_count as u64,
            ),
            WorthNamedCounter::new(
                "authority.total_phase_count",
                self.total_phase_count as u64,
            ),
            WorthNamedCounter::new(
                "authority.history_summary_count",
                self.history_summary_count as u64,
            ),
            WorthNamedCounter::new(
                "authority.publication_summary_count",
                self.publication_summary_count as u64,
            ),
            WorthNamedCounter::new(
                "authority.invariant_result_count",
                self.invariant_result_count as u64,
            ),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthAuthorityTraceAnchor {
    pub branch_id: BranchId,
    pub runtime_instance_ids: Vec<u64>,
    pub transaction_ids: Vec<TransactionId>,
    pub commit_ids: Vec<CommitId>,
    pub snapshot_ids: Vec<SnapshotId>,
    pub version_ids: Vec<VersionId>,
}

impl WorthAuthorityTraceAnchor {
    pub fn from_commit_results(branch_id: BranchId, commits: &[CommitResult]) -> Self {
        Self {
            branch_id,
            runtime_instance_ids: commits
                .iter()
                .map(|commit| commit.snapshot.runtime_instance_id)
                .collect(),
            transaction_ids: commits.iter().map(|commit| commit.transaction_id).collect(),
            commit_ids: commits.iter().map(|commit| commit.commit.commit_id).collect(),
            snapshot_ids: commits
                .iter()
                .map(|commit| commit.snapshot.snapshot_id)
                .collect(),
            version_ids: commits.iter().map(|commit| commit.version_id).collect(),
        }
    }

    pub fn latest_commit_id(&self) -> Option<CommitId> {
        self.commit_ids.last().copied()
    }

    pub fn latest_snapshot_id(&self) -> Option<SnapshotId> {
        self.snapshot_ids.last().copied()
    }

    pub fn latest_runtime_instance_id(&self) -> Option<u64> {
        self.runtime_instance_ids.last().copied()
    }

    pub fn latest_version_id(&self) -> Option<VersionId> {
        self.version_ids.last().copied()
    }

    pub fn open_latest_snapshot(
        &self,
        runtime: &RelationalRuntime,
    ) -> Option<RelationalReadView> {
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
pub struct WorthBridgeTraceEvidence {
    pub availability: WorthTraceAvailability,
    pub route_identities: Vec<String>,
    pub invalidation_identities: Vec<String>,
    pub snapshot_identities: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct WorthDerivedTraceEvidence {
    pub availability: WorthTraceAvailability,
    pub invalidation_target_count: usize,
    pub fallback_classes: Vec<String>,
    pub equivalence_digest: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthSignalTraceEvidence {
    pub availability: WorthTraceAvailability,
    pub explanation_availability: Option<String>,
    pub provenance_availability: Option<String>,
    pub replay_event_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct WorthBridgeTraceAnchor {
    pub route_identities: Vec<String>,
    pub invalidation_identities: Vec<String>,
    pub snapshot_identities: Vec<String>,
    pub historical_record_identities: Vec<String>,
}

impl WorthBridgeTraceAnchor {
    pub fn new(
        route_identities: impl IntoIterator<Item = String>,
        invalidation_identities: impl IntoIterator<Item = String>,
        snapshot_identities: impl IntoIterator<Item = String>,
        historical_record_identities: impl IntoIterator<Item = String>,
    ) -> Self {
        Self {
            route_identities: route_identities.into_iter().collect(),
            invalidation_identities: invalidation_identities.into_iter().collect(),
            snapshot_identities: snapshot_identities.into_iter().collect(),
            historical_record_identities: historical_record_identities.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthDerivedTraceAnchor {
    pub branch_id: BranchId,
    pub runtime_instance_id: u64,
    pub snapshot_id: SnapshotId,
    pub version_id: VersionId,
    pub truth_basis_identity: WorthDerivedTruthBasisIdentity,
}

impl WorthDerivedTraceAnchor {
    pub fn from_read_basis(basis: &DerivedTopologyReadBasis) -> Self {
        Self {
            branch_id: basis.branch_id().clone(),
            runtime_instance_id: basis.snapshot().runtime_instance_id,
            snapshot_id: basis.snapshot().snapshot_id,
            version_id: basis.snapshot().version_id,
            truth_basis_identity: basis.authority.truth_basis_identity.clone(),
        }
    }

    pub fn open_snapshot(&self, runtime: &RelationalRuntime) -> Option<RelationalReadView> {
        let handle = forge_relational::facade::snapshots::SnapshotHandle {
            runtime_instance_id: self.runtime_instance_id,
            snapshot_id: self.snapshot_id,
            version_id: self.version_id,
            read_policy:
                forge_relational::facade::snapshots::SnapshotReadPolicy::ImmutablePinnedNoLazyMutation,
        };
        runtime.read_truth().read_snapshot(&handle)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthSignalTraceAnchor {
    pub node: SignalNodeId,
    pub replay_cursor: Option<ReplayCursor>,
    pub execution_record_id: Option<u64>,
    pub semantic_segment_id: Option<u64>,
    pub lineage_artifact_id: Option<LineageArtifactId>,
}

impl WorthSignalTraceAnchor {
    pub fn from_graph(
        graph: &SignalGraph,
        node: SignalNodeId,
    ) -> Result<Self, forge_signal::facade::SignalError> {
        let observer = graph.observe();
        let explanation = observer.explain(node)?;
        let replay = observer.replay_for_node(node);
        Ok(Self {
            node,
            replay_cursor: replay.last().map(|event| event.cursor),
            execution_record_id: explanation.execution_record_id,
            semantic_segment_id: explanation.semantic_segment_id,
            lineage_artifact_id: observer.current_lineage_artifact(node),
        })
    }
}

impl WorthSignalTraceEvidence {
    pub fn from_graph(
        graph: &SignalGraph,
        node: SignalNodeId,
    ) -> Result<Self, forge_signal::facade::SignalError> {
        let observer = graph.observe();
        let replay = observer.replay_for_node(node);
        let forensic = forge_signal::facade::diagnostics_for_graph(graph).forensic();
        let (_, explanation_availability) = forensic.materialize_explanation_artifact(node)?;
        let (_, provenance_availability) = forensic.materialize_provenance_artifact(node)?;
        Ok(Self {
            availability: WorthTraceAvailability::Present,
            explanation_availability: Some(format_signal_diagnostics_availability(
                explanation_availability,
            )),
            provenance_availability: Some(format_signal_diagnostics_availability(
                provenance_availability,
            )),
            replay_event_count: replay.len(),
        })
    }
}

fn format_signal_diagnostics_availability(
    availability: SignalDiagnosticsAvailability,
) -> String {
    format!("{availability:?}")
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct WorthDecisionTrace {
    pub authority_anchor: Option<WorthAuthorityTraceAnchor>,
    pub bridge_anchor: Option<WorthBridgeTraceAnchor>,
    pub derived_anchor: Option<WorthDerivedTraceAnchor>,
    pub signal_anchor: Option<WorthSignalTraceAnchor>,
    pub authority: Option<WorthAuthorityTraceEvidence>,
    pub bridge: Option<WorthBridgeTraceEvidence>,
    pub derived: Option<WorthDerivedTraceEvidence>,
    pub signal: Option<WorthSignalTraceEvidence>,
}

impl WorthDecisionTrace {
    pub fn authority_anchor(&self) -> Option<&WorthAuthorityTraceAnchor> {
        self.authority_anchor.as_ref()
    }

    pub fn bridge_anchor(&self) -> Option<&WorthBridgeTraceAnchor> {
        self.bridge_anchor.as_ref()
    }

    pub fn derived_anchor(&self) -> Option<&WorthDerivedTraceAnchor> {
        self.derived_anchor.as_ref()
    }

    pub fn signal_anchor(&self) -> Option<&WorthSignalTraceAnchor> {
        self.signal_anchor.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorthBoundaryEnvelope<T> {
    primary_result: T,
    warnings: Vec<WorthTraceWarning>,
    decision_trace: WorthDecisionTrace,
    integrity_markers: WorthIntegrityMarkers,
    performance_accounting: WorthPerformanceAccounting,
}

impl<T> WorthBoundaryEnvelope<T> {
    pub fn success(
        primary_result: T,
        warnings: Vec<WorthTraceWarning>,
        decision_trace: WorthDecisionTrace,
        integrity_markers: WorthIntegrityMarkers,
        performance_accounting: WorthPerformanceAccounting,
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

    pub fn map_primary_result<U>(self, map: impl FnOnce(T) -> U) -> WorthBoundaryEnvelope<U> {
        let (
            primary_result,
            warnings,
            decision_trace,
            integrity_markers,
            performance_accounting,
        ) = self.into_parts();
        WorthBoundaryEnvelope::success(
            map(primary_result),
            warnings,
            decision_trace,
            integrity_markers,
            performance_accounting,
        )
    }

    pub fn warnings(&self) -> &[WorthTraceWarning] {
        &self.warnings
    }

    pub fn decision_trace(&self) -> &WorthDecisionTrace {
        &self.decision_trace
    }

    pub fn integrity_markers(&self) -> &WorthIntegrityMarkers {
        &self.integrity_markers
    }

    pub fn performance_accounting(&self) -> &WorthPerformanceAccounting {
        &self.performance_accounting
    }

    pub fn into_parts(
        self,
    ) -> (
        T,
        Vec<WorthTraceWarning>,
        WorthDecisionTrace,
        WorthIntegrityMarkers,
        WorthPerformanceAccounting,
    ) {
        (
            self.primary_result,
            self.warnings,
            self.decision_trace,
            self.integrity_markers,
            self.performance_accounting,
        )
    }

    pub fn with_decision_trace(self, decision_trace: WorthDecisionTrace) -> Self {
        let (primary_result, warnings, _, integrity_markers, performance_accounting) =
            self.into_parts();
        Self::success(
            primary_result,
            warnings,
            decision_trace,
            integrity_markers,
            performance_accounting,
        )
    }

    pub fn map_decision_trace(
        self,
        map: impl FnOnce(WorthDecisionTrace) -> WorthDecisionTrace,
    ) -> Self {
        let (primary_result, warnings, decision_trace, integrity_markers, performance_accounting) =
            self.into_parts();
        Self::success(
            primary_result,
            warnings,
            map(decision_trace),
            integrity_markers,
            performance_accounting,
        )
    }

    pub fn with_integrity_markers(self, integrity_markers: WorthIntegrityMarkers) -> Self {
        let (primary_result, warnings, decision_trace, _, performance_accounting) =
            self.into_parts();
        Self::success(
            primary_result,
            warnings,
            decision_trace,
            integrity_markers,
            performance_accounting,
        )
    }

    pub fn with_performance_accounting(
        self,
        performance_accounting: WorthPerformanceAccounting,
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
pub struct WorthBoundaryFailure<E> {
    error: E,
    warnings: Vec<WorthTraceWarning>,
    decision_trace: WorthDecisionTrace,
    integrity_markers: WorthIntegrityMarkers,
    performance_accounting: WorthPerformanceAccounting,
}

impl<E> WorthBoundaryFailure<E> {
    pub fn failure(
        error: E,
        warnings: Vec<WorthTraceWarning>,
        decision_trace: WorthDecisionTrace,
        integrity_markers: WorthIntegrityMarkers,
        performance_accounting: WorthPerformanceAccounting,
    ) -> Self {
        Self {
            error,
            warnings,
            decision_trace,
            integrity_markers,
            performance_accounting,
        }
    }

    pub fn error(&self) -> &E {
        &self.error
    }

    pub fn into_error(self) -> E {
        self.error
    }

    pub fn map_error<F>(self, map: impl FnOnce(E) -> F) -> WorthBoundaryFailure<F> {
        let (error, warnings, decision_trace, integrity_markers, performance_accounting) =
            self.into_parts();
        WorthBoundaryFailure::failure(
            map(error),
            warnings,
            decision_trace,
            integrity_markers,
            performance_accounting,
        )
    }

    pub fn warnings(&self) -> &[WorthTraceWarning] {
        &self.warnings
    }

    pub fn decision_trace(&self) -> &WorthDecisionTrace {
        &self.decision_trace
    }

    pub fn integrity_markers(&self) -> &WorthIntegrityMarkers {
        &self.integrity_markers
    }

    pub fn performance_accounting(&self) -> &WorthPerformanceAccounting {
        &self.performance_accounting
    }

    pub fn into_parts(
        self,
    ) -> (
        E,
        Vec<WorthTraceWarning>,
        WorthDecisionTrace,
        WorthIntegrityMarkers,
        WorthPerformanceAccounting,
    ) {
        (
            self.error,
            self.warnings,
            self.decision_trace,
            self.integrity_markers,
            self.performance_accounting,
        )
    }

    pub fn with_decision_trace(self, decision_trace: WorthDecisionTrace) -> Self {
        let (error, warnings, _, integrity_markers, performance_accounting) = self.into_parts();
        Self::failure(
            error,
            warnings,
            decision_trace,
            integrity_markers,
            performance_accounting,
        )
    }

    pub fn map_decision_trace(
        self,
        map: impl FnOnce(WorthDecisionTrace) -> WorthDecisionTrace,
    ) -> Self {
        let (error, warnings, decision_trace, integrity_markers, performance_accounting) =
            self.into_parts();
        Self::failure(
            error,
            warnings,
            map(decision_trace),
            integrity_markers,
            performance_accounting,
        )
    }

    pub fn with_integrity_markers(self, integrity_markers: WorthIntegrityMarkers) -> Self {
        let (error, warnings, decision_trace, _, performance_accounting) = self.into_parts();
        Self::failure(
            error,
            warnings,
            decision_trace,
            integrity_markers,
            performance_accounting,
        )
    }

    pub fn with_performance_accounting(
        self,
        performance_accounting: WorthPerformanceAccounting,
    ) -> Self {
        let (error, warnings, decision_trace, integrity_markers, _) = self.into_parts();
        Self::failure(
            error,
            warnings,
            decision_trace,
            integrity_markers,
            performance_accounting,
        )
    }
}

#[cfg(test)]
mod tests;
