#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeCanonicalBulkPlanRecord {
    schema_version: Arc<str>,
    request: BridgeBulkWorkloadRequest,
    workload_identity: BridgeWorkloadIdentity,
    canonical_request_digest: Arc<str>,
    normalized_summary_digest: Arc<str>,
    canonical_planning_identity: BridgeCanonicalPlanningIdentity,
    admission_profile_identity: BridgeAdmissionProfileIdentity,
    packet_set_digest: Arc<str>,
    execution_plan_digest: Arc<str>,
    reduced_artifact_digest: Arc<str>,
    selected_mode: BridgePreparationMode,
    decision_log: BridgeBulkDecisionLog,
    counters: BridgeBulkPlanningCounters,
    planning_failures: Arc<[BridgeBulkPlanningFailure]>,
}

impl BridgeCanonicalBulkPlanRecord {
    pub(crate) fn from_bulk_workload_plan(plan: &BridgeBulkWorkloadPlan) -> Self {
        Self {
            schema_version: Arc::from(BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1),
            request: plan.request().clone(),
            workload_identity: plan.workload_identity().clone(),
            canonical_request_digest: Arc::from(plan.canonical_request().digest().to_owned()),
            normalized_summary_digest: Arc::from(plan.normalized_summary().digest().to_owned()),
            canonical_planning_identity: plan.canonical_planning_identity().clone(),
            admission_profile_identity: plan.admission_profile_identity().clone(),
            packet_set_digest: Arc::from(plan.packet_set().digest().to_owned()),
            execution_plan_digest: Arc::from(plan.execution_plan().digest().to_owned()),
            reduced_artifact_digest: Arc::from(
                plan.execution_plan().reduced_artifact().digest().to_owned(),
            ),
            selected_mode: plan.execution_plan().selected_mode(),
            decision_log: plan.execution_plan().decision_log().clone(),
            counters: plan.execution_plan().counters().clone(),
            planning_failures: Arc::from(plan.execution_plan().planning_failures().to_vec()),
        }
    }

    pub fn schema_version(&self) -> &str {
        self.schema_version.as_ref()
    }

    pub fn request(&self) -> &BridgeBulkWorkloadRequest {
        &self.request
    }

    pub fn workload_identity(&self) -> &BridgeWorkloadIdentity {
        &self.workload_identity
    }

    pub fn canonical_request_digest(&self) -> &str {
        self.canonical_request_digest.as_ref()
    }

    pub fn normalized_summary_digest(&self) -> &str {
        self.normalized_summary_digest.as_ref()
    }

    pub fn canonical_planning_identity(&self) -> &BridgeCanonicalPlanningIdentity {
        &self.canonical_planning_identity
    }

    pub fn admission_profile_identity(&self) -> &BridgeAdmissionProfileIdentity {
        &self.admission_profile_identity
    }

    pub fn packet_set_digest(&self) -> &str {
        self.packet_set_digest.as_ref()
    }

    pub fn execution_plan_digest(&self) -> &str {
        self.execution_plan_digest.as_ref()
    }

    pub fn reduced_artifact_digest(&self) -> &str {
        self.reduced_artifact_digest.as_ref()
    }

    pub fn selected_mode(&self) -> BridgePreparationMode {
        self.selected_mode
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

    #[cfg(test)]
    pub(crate) fn with_schema_version_for_test(
        mut self,
        schema_version: impl Into<Arc<str>>,
    ) -> Self {
        self.schema_version = schema_version.into();
        self
    }

    pub(crate) fn decode(&self) -> Result<Self, BridgeReplayError> {
        if self.schema_version() != BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1 {
            return Err(BridgeReplayError::new(
                BridgeReplayErrorKind::CanonicalArtifactCompatibilityFailure,
                format!(
                    "Bridge canonical bulk plan record schema `{}` is not supported; expected `{}`.",
                    self.schema_version(),
                    BRIDGE_CANONICAL_BULK_PLAN_RECORD_SCHEMA_V1
                ),
            )
            .with_context(BridgeErrorContext::default()));
        }

        Ok(self.clone())
    }
}

use super::*;
