use crate::facade::{
    BridgePreviewDiscardRecordIdentity, BridgePreviewLifecycleStateKind, BridgePreviewReplayBundle,
    BridgePreviewSessionIdentity, PreviewExecutionRecordIdentity, TruthBranchIdentity,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::harness::adapter::adapter_impl) struct SpeculationPreviewReplayBundleSet {
    replay_bundles: Vec<BridgePreviewReplayBundle>,
}

impl SpeculationPreviewReplayBundleSet {
    pub(super) fn from_replay_bundles(
        replay_bundles: impl IntoIterator<Item = BridgePreviewReplayBundle>,
    ) -> Self {
        Self {
            replay_bundles: replay_bundles.into_iter().collect(),
        }
    }

    pub(super) fn replay_bundles(&self) -> &[BridgePreviewReplayBundle] {
        &self.replay_bundles
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::harness::adapter::adapter_impl) struct SpeculationChurnCertification {
    preview_replay_bundle_set: SpeculationPreviewReplayBundleSet,
    branch_isolation_matrix: SpeculationBranchIsolationMatrix,
    resource_bound_report: SpeculationResourceBoundReport,
    counter_snapshot: SpeculationChurnCounterSnapshot,
}

impl SpeculationChurnCertification {
    pub(super) fn new(
        preview_replay_bundle_set: SpeculationPreviewReplayBundleSet,
        branch_isolation_matrix: SpeculationBranchIsolationMatrix,
        resource_bound_report: SpeculationResourceBoundReport,
        counter_snapshot: SpeculationChurnCounterSnapshot,
    ) -> Self {
        Self {
            preview_replay_bundle_set,
            branch_isolation_matrix,
            resource_bound_report,
            counter_snapshot,
        }
    }

    pub(super) fn preview_replay_bundle_set(&self) -> &SpeculationPreviewReplayBundleSet {
        &self.preview_replay_bundle_set
    }

    pub(super) fn branch_isolation_matrix(&self) -> &SpeculationBranchIsolationMatrix {
        &self.branch_isolation_matrix
    }

    pub(super) fn resource_bound_report(&self) -> &SpeculationResourceBoundReport {
        &self.resource_bound_report
    }

    pub(super) fn counter_snapshot(&self) -> &SpeculationChurnCounterSnapshot {
        &self.counter_snapshot
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::harness::adapter::adapter_impl) struct SpeculationBranchIsolationMatrix {
    rows: Vec<SpeculationBranchIsolationRow>,
    baseline_authoritative_route_digest: Option<String>,
    final_authoritative_route_digest: Option<String>,
}

impl SpeculationBranchIsolationMatrix {
    pub(super) fn new(
        rows: Vec<SpeculationBranchIsolationRow>,
        baseline_authoritative_route_digest: Option<String>,
        final_authoritative_route_digest: Option<String>,
    ) -> Self {
        Self {
            rows,
            baseline_authoritative_route_digest,
            final_authoritative_route_digest,
        }
    }

    pub(super) fn rows(&self) -> &[SpeculationBranchIsolationRow] {
        &self.rows
    }

    pub(super) fn baseline_authoritative_route_digest(&self) -> Option<&str> {
        self.baseline_authoritative_route_digest.as_deref()
    }

    pub(super) fn final_authoritative_route_digest(&self) -> Option<&str> {
        self.final_authoritative_route_digest.as_deref()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::harness::adapter::adapter_impl) struct SpeculationBranchIsolationRow {
    preview_session_identity: BridgePreviewSessionIdentity,
    truth_branch_identity: TruthBranchIdentity,
    execution_record_identity: PreviewExecutionRecordIdentity,
    discard_record_identity: BridgePreviewDiscardRecordIdentity,
    lifecycle_outcome: BridgePreviewLifecycleStateKind,
    authoritative_route_digest_after_discard: Option<String>,
}

impl SpeculationBranchIsolationRow {
    pub(super) fn new(
        preview_session_identity: BridgePreviewSessionIdentity,
        truth_branch_identity: TruthBranchIdentity,
        execution_record_identity: PreviewExecutionRecordIdentity,
        discard_record_identity: BridgePreviewDiscardRecordIdentity,
        lifecycle_outcome: BridgePreviewLifecycleStateKind,
        authoritative_route_digest_after_discard: Option<String>,
    ) -> Self {
        Self {
            preview_session_identity,
            truth_branch_identity,
            execution_record_identity,
            discard_record_identity,
            lifecycle_outcome,
            authoritative_route_digest_after_discard,
        }
    }

    pub(super) fn preview_session_identity(&self) -> &str {
        self.preview_session_identity.as_str()
    }

    pub(super) fn truth_branch_identity(&self) -> &str {
        self.truth_branch_identity.as_str()
    }

    pub(super) fn execution_record_identity(&self) -> &str {
        self.execution_record_identity.as_str()
    }

    pub(super) fn discard_record_identity(&self) -> &str {
        self.discard_record_identity.as_str()
    }

    pub(super) fn lifecycle_outcome(&self) -> BridgePreviewLifecycleStateKind {
        self.lifecycle_outcome
    }

    pub(super) fn authoritative_route_digest_after_discard(&self) -> Option<&str> {
        self.authoritative_route_digest_after_discard.as_deref()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::harness::adapter::adapter_impl) struct SpeculationResourceBoundReport {
    retained_preview_execution_record_count: usize,
    retained_preview_discard_record_count: usize,
    retained_preview_promotion_record_count: usize,
    max_preview_artifact_count: usize,
    max_replay_bundle_width: usize,
    authoritative_route_observation_count: usize,
}

impl SpeculationResourceBoundReport {
    pub(super) fn new(
        retained_preview_execution_record_count: usize,
        retained_preview_discard_record_count: usize,
        retained_preview_promotion_record_count: usize,
        max_preview_artifact_count: usize,
        max_replay_bundle_width: usize,
        authoritative_route_observation_count: usize,
    ) -> Self {
        Self {
            retained_preview_execution_record_count,
            retained_preview_discard_record_count,
            retained_preview_promotion_record_count,
            max_preview_artifact_count,
            max_replay_bundle_width,
            authoritative_route_observation_count,
        }
    }

    pub(super) fn retained_preview_execution_record_count(&self) -> usize {
        self.retained_preview_execution_record_count
    }

    pub(super) fn retained_preview_discard_record_count(&self) -> usize {
        self.retained_preview_discard_record_count
    }

    pub(super) fn retained_preview_promotion_record_count(&self) -> usize {
        self.retained_preview_promotion_record_count
    }

    pub(super) fn max_preview_artifact_count(&self) -> usize {
        self.max_preview_artifact_count
    }

    pub(super) fn max_replay_bundle_width(&self) -> usize {
        self.max_replay_bundle_width
    }

    pub(super) fn authoritative_route_observation_count(&self) -> usize {
        self.authoritative_route_observation_count
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::harness::adapter::adapter_impl) struct SpeculationChurnCounterSnapshot {
    preview_session_count_touched: usize,
    max_preview_artifact_count: usize,
    max_replay_bundle_width: usize,
    retained_preview_execution_record_count: usize,
    retained_preview_discard_record_count: usize,
    retained_preview_promotion_record_count: usize,
    authoritative_route_observation_count: usize,
}

impl SpeculationChurnCounterSnapshot {
    pub(super) fn from_churn_report(
        preview_session_count_touched: usize,
        resource_bound_report: &SpeculationResourceBoundReport,
    ) -> Self {
        Self {
            preview_session_count_touched,
            max_preview_artifact_count: resource_bound_report.max_preview_artifact_count(),
            max_replay_bundle_width: resource_bound_report.max_replay_bundle_width(),
            retained_preview_execution_record_count: resource_bound_report
                .retained_preview_execution_record_count(),
            retained_preview_discard_record_count: resource_bound_report
                .retained_preview_discard_record_count(),
            retained_preview_promotion_record_count: resource_bound_report
                .retained_preview_promotion_record_count(),
            authoritative_route_observation_count: resource_bound_report
                .authoritative_route_observation_count(),
        }
    }

    pub(super) fn preview_session_count_touched(&self) -> usize {
        self.preview_session_count_touched
    }

    pub(super) fn max_preview_artifact_count(&self) -> usize {
        self.max_preview_artifact_count
    }

    pub(super) fn max_replay_bundle_width(&self) -> usize {
        self.max_replay_bundle_width
    }

    pub(super) fn retained_preview_execution_record_count(&self) -> usize {
        self.retained_preview_execution_record_count
    }

    pub(super) fn retained_preview_discard_record_count(&self) -> usize {
        self.retained_preview_discard_record_count
    }

    pub(super) fn retained_preview_promotion_record_count(&self) -> usize {
        self.retained_preview_promotion_record_count
    }

    pub(super) fn authoritative_route_observation_count(&self) -> usize {
        self.authoritative_route_observation_count
    }
}

#[cfg(test)]
mod tests {
    use super::{SpeculationChurnCounterSnapshot, SpeculationResourceBoundReport};

    #[test]
    fn churn_counter_snapshot_is_derived_from_typed_resource_evidence() {
        let resource_bound_report = SpeculationResourceBoundReport::new(3, 4, 0, 8, 5, 6);
        let counter_snapshot =
            SpeculationChurnCounterSnapshot::from_churn_report(3, &resource_bound_report);

        assert_eq!(counter_snapshot.preview_session_count_touched(), 3);
        assert_eq!(counter_snapshot.max_preview_artifact_count(), 8);
        assert_eq!(counter_snapshot.max_replay_bundle_width(), 5);
        assert_eq!(
            counter_snapshot.retained_preview_execution_record_count(),
            3
        );
        assert_eq!(counter_snapshot.retained_preview_discard_record_count(), 4);
        assert_eq!(
            counter_snapshot.retained_preview_promotion_record_count(),
            0
        );
        assert_eq!(counter_snapshot.authoritative_route_observation_count(), 6);
    }
}
