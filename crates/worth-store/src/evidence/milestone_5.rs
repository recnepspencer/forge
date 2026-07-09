use crate::{
    authority::AuthoritativeExportBundle,
    delta::{stable_branch_delta_digest, BranchDeltaReadPlan},
    evidence::StoreCounterSnapshot,
};
use worth_relational::facade::history::{BranchId, CommitId};
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone5ReadPathReport {
    pub strategy: crate::BranchDeltaReadStrategy,
    pub regime: crate::BranchDeltaReadRegime,
    pub layers_traversed: usize,
    pub records_decoded: usize,
    pub replay_commit_count: usize,
    pub fallback_class: crate::BranchDeltaFallbackClass,
    pub complexity_status: crate::ComplexityStatus,
}

impl From<&BranchDeltaReadPlan> for Milestone5ReadPathReport {
    fn from(plan: &BranchDeltaReadPlan) -> Self {
        Self {
            strategy: plan.strategy,
            regime: plan.regime,
            layers_traversed: plan.performance.layers_traversed,
            records_decoded: plan.performance.records_decoded,
            replay_commit_count: plan.performance.replay_commit_count,
            fallback_class: plan.performance.fallback_class,
            complexity_status: plan.performance.complexity_status,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone5DeltaStorageReport {
    pub branch_id: BranchId,
    pub target_commit_id: CommitId,
    pub shared_base_source_branch_id: BranchId,
    pub shared_base_source_frontier_commit_id: Option<CommitId>,
    pub live_layer_count: usize,
    pub live_layer_commit_count: usize,
    pub replacement_layer_count: usize,
    pub direct_path: Milestone5ReadPathReport,
    pub control_path: Milestone5ReadPathReport,
    pub control_reference_surface: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone5CertificationBundle {
    pub truth_digest: String,
    pub history_digest: String,
    pub delta_storage_report: Milestone5DeltaStorageReport,
    pub counter_snapshot: StoreCounterSnapshot,
}

impl Milestone5CertificationBundle {
    pub fn new(
        direct_export: &AuthoritativeExportBundle,
        control_export: &AuthoritativeExportBundle,
        delta_storage_report: Milestone5DeltaStorageReport,
        counter_snapshot: StoreCounterSnapshot,
    ) -> Self {
        let canonical_direct = direct_export.clone().into_canonicalized();
        let canonical_control = control_export.clone().into_canonicalized();
        let truth_digest = stable_branch_delta_digest(&canonical_direct);
        let control_truth_digest = stable_branch_delta_digest(&canonical_control);
        assert_eq!(
            truth_digest, control_truth_digest,
            "milestone 5 certification requires direct and control truth digests to match"
        );
        Self {
            truth_digest,
            history_digest: stable_branch_delta_digest(&canonical_direct.commit_envelopes),
            delta_storage_report,
            counter_snapshot,
        }
    }

    pub fn canonical_json(&self) -> String {
        serde_json::to_string(self).expect("milestone 5 certification serialization")
    }
}
