use crate::{
    authority::AuthoritativeExportBundle,
    bulk::{BulkPlanKind, DeterministicChunkPlan},
    evidence::StoreCounterSnapshot,
};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone9CertificationBundle {
    pub plan_kind: BulkPlanKind,
    pub chunk_count: usize,
    pub truth_digest: String,
    pub history_digest: String,
    pub restore_digest: String,
    pub chunk_plan_digest: String,
    pub certification_summary: Milestone9CertificationSummary,
    pub counter_snapshot: StoreCounterSnapshot,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Milestone9CertificationSummary {
    pub truth_matches_control_lane: bool,
    pub history_matches_control_lane: bool,
    pub restore_truth_parity: bool,
    pub restore_history_parity: bool,
    pub deterministic_chunk_plan_observed: bool,
}

impl Milestone9CertificationBundle {
    pub fn new(
        executed_export: &AuthoritativeExportBundle,
        control_export: &AuthoritativeExportBundle,
        restored_export: &AuthoritativeExportBundle,
        executed_plan: &DeterministicChunkPlan,
        equivalent_plan: &DeterministicChunkPlan,
        counter_snapshot: StoreCounterSnapshot,
    ) -> Self {
        let canonical_executed = executed_export.clone().into_canonicalized();
        let canonical_control = control_export.clone().into_canonicalized();
        let canonical_restored = restored_export.clone().into_canonicalized();

        let truth_digest = stable_digest(&canonical_executed);
        let control_truth_digest = stable_digest(&canonical_control);
        let history_digest = stable_digest(&canonical_executed.commit_envelopes);
        let control_history_digest = stable_digest(&canonical_control.commit_envelopes);
        let restore_digest = stable_digest(&canonical_restored);
        let restore_history_digest = stable_digest(&canonical_restored.commit_envelopes);
        let chunk_plan_digest = stable_digest(executed_plan);
        let equivalent_chunk_plan_digest = stable_digest(equivalent_plan);

        let certification_summary = Milestone9CertificationSummary {
            truth_matches_control_lane: truth_digest == control_truth_digest,
            history_matches_control_lane: history_digest == control_history_digest,
            restore_truth_parity: truth_digest == restore_digest,
            restore_history_parity: history_digest == restore_history_digest,
            deterministic_chunk_plan_observed: chunk_plan_digest == equivalent_chunk_plan_digest,
        };

        assert!(
            certification_summary.truth_matches_control_lane,
            "milestone 9 certification requires bulk truth to match the logically serial control lane"
        );
        assert!(
            certification_summary.history_matches_control_lane,
            "milestone 9 certification requires bulk history to match the logically serial control lane"
        );
        assert!(
            certification_summary.restore_truth_parity,
            "milestone 9 certification requires restored truth to match the executed bulk lane"
        );
        assert!(
            certification_summary.restore_history_parity,
            "milestone 9 certification requires restored history to match the executed bulk lane"
        );
        assert!(
            certification_summary.deterministic_chunk_plan_observed,
            "milestone 9 certification requires equivalent planning lanes to emit the same deterministic chunk plan"
        );

        Self {
            plan_kind: executed_plan.kind(),
            chunk_count: executed_plan.chunk_count(),
            truth_digest,
            history_digest,
            restore_digest,
            chunk_plan_digest,
            certification_summary,
            counter_snapshot,
        }
    }

    pub fn canonical_json(&self) -> String {
        serde_json::to_string(self).expect("milestone 9 certification serialization")
    }
}

fn stable_digest<T: Serialize>(value: &T) -> String {
    let json = serde_json::to_vec(value).expect("milestone 9 evidence should serialize");
    let mut hasher = Sha256::new();
    hasher.update(json);
    format!("{:x}", hasher.finalize())
}
