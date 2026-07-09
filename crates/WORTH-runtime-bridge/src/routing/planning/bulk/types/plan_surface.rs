#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeBulkWorkloadPlan {
    request: BridgeBulkWorkloadRequest,
    workload_identity: BridgeWorkloadIdentity,
    canonical_request: CanonicalBridgeWorkloadRequest,
    normalized_summary: NormalizedBridgeWorkloadSummary,
    canonical_planning_identity: BridgeCanonicalPlanningIdentity,
    admission_profile_identity: BridgeAdmissionProfileIdentity,
    packet_set: PlannedBridgePacketSet,
    execution_plan: AdmittedBridgeExecutionPlan,
    planned_routes: Vec<BridgePlannedRoute>,
    summary: BridgeBulkPlanningSummary,
}

impl BridgeBulkWorkloadPlan {
    pub(crate) fn new(
        request: BridgeBulkWorkloadRequest,
        workload_identity: BridgeWorkloadIdentity,
        canonical_request: CanonicalBridgeWorkloadRequest,
        normalized_summary: NormalizedBridgeWorkloadSummary,
        canonical_planning_identity: BridgeCanonicalPlanningIdentity,
        admission_profile_identity: BridgeAdmissionProfileIdentity,
        packet_set: PlannedBridgePacketSet,
        execution_plan: AdmittedBridgeExecutionPlan,
        planned_routes: Vec<BridgePlannedRoute>,
        summary: BridgeBulkPlanningSummary,
    ) -> Self {
        Self {
            request,
            workload_identity,
            canonical_request,
            normalized_summary,
            canonical_planning_identity,
            admission_profile_identity,
            packet_set,
            execution_plan,
            planned_routes,
            summary,
        }
    }

    pub fn workload_identity(&self) -> &BridgeWorkloadIdentity {
        &self.workload_identity
    }

    pub fn request(&self) -> &BridgeBulkWorkloadRequest {
        &self.request
    }

    pub fn canonical_request(&self) -> &CanonicalBridgeWorkloadRequest {
        &self.canonical_request
    }

    pub fn normalized_summary(&self) -> &NormalizedBridgeWorkloadSummary {
        &self.normalized_summary
    }

    pub fn canonical_planning_identity(&self) -> &BridgeCanonicalPlanningIdentity {
        &self.canonical_planning_identity
    }

    pub fn admission_profile_identity(&self) -> &BridgeAdmissionProfileIdentity {
        &self.admission_profile_identity
    }

    pub fn execution_plan(&self) -> &AdmittedBridgeExecutionPlan {
        &self.execution_plan
    }

    pub fn packet_set(&self) -> &PlannedBridgePacketSet {
        &self.packet_set
    }

    pub fn planned_routes(&self) -> &[BridgePlannedRoute] {
        &self.planned_routes
    }

    pub fn summary(&self) -> &BridgeBulkPlanningSummary {
        &self.summary
    }
}
use super::*;
