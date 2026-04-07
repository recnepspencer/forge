use crate::routing::{
    BridgeAdmissionProfileIdentity, BridgeBulkDecisionLog, BridgeBulkPlanningCounters,
    BridgeBulkPlanningFailure, BridgeCanonicalBulkPlanRecord, BridgeCanonicalPlanningIdentity,
    BridgePreparationMode, BridgeWorkloadIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeBulkPlanExplanation {
    workload_identity: BridgeWorkloadIdentity,
    canonical_planning_identity: BridgeCanonicalPlanningIdentity,
    admission_profile_identity: BridgeAdmissionProfileIdentity,
    selected_mode: BridgePreparationMode,
    request_segment_count: usize,
    packet_set_digest: String,
    execution_plan_digest: String,
    reduced_artifact_digest: String,
    decision_log: BridgeBulkDecisionLog,
    counters: BridgeBulkPlanningCounters,
    planning_failures: Vec<BridgeBulkPlanningFailure>,
}

impl BridgeBulkPlanExplanation {
    pub(crate) fn from_canonical_record(record: &BridgeCanonicalBulkPlanRecord) -> Self {
        Self {
            workload_identity: record.workload_identity().clone(),
            canonical_planning_identity: record.canonical_planning_identity().clone(),
            admission_profile_identity: record.admission_profile_identity().clone(),
            selected_mode: record.selected_mode(),
            request_segment_count: record.request().segments().len(),
            packet_set_digest: record.packet_set_digest().to_owned(),
            execution_plan_digest: record.execution_plan_digest().to_owned(),
            reduced_artifact_digest: record.reduced_artifact_digest().to_owned(),
            decision_log: record.decision_log().clone(),
            counters: record.counters().clone(),
            planning_failures: record.planning_failures().to_vec(),
        }
    }

    pub fn workload_identity(&self) -> &BridgeWorkloadIdentity {
        &self.workload_identity
    }

    pub fn canonical_planning_identity(&self) -> &BridgeCanonicalPlanningIdentity {
        &self.canonical_planning_identity
    }

    pub fn admission_profile_identity(&self) -> &BridgeAdmissionProfileIdentity {
        &self.admission_profile_identity
    }

    pub fn selected_mode(&self) -> BridgePreparationMode {
        self.selected_mode
    }

    pub fn request_segment_count(&self) -> usize {
        self.request_segment_count
    }

    pub fn packet_set_digest(&self) -> &str {
        &self.packet_set_digest
    }

    pub fn execution_plan_digest(&self) -> &str {
        &self.execution_plan_digest
    }

    pub fn reduced_artifact_digest(&self) -> &str {
        &self.reduced_artifact_digest
    }

    pub fn decision_log(&self) -> &BridgeBulkDecisionLog {
        &self.decision_log
    }

    pub fn decision_log_digest(&self) -> &str {
        self.decision_log.digest()
    }

    pub fn counters(&self) -> &BridgeBulkPlanningCounters {
        &self.counters
    }

    pub fn planning_failures(&self) -> &[BridgeBulkPlanningFailure] {
        &self.planning_failures
    }

    pub fn planning_failure_count(&self) -> usize {
        self.planning_failures.len()
    }
}
