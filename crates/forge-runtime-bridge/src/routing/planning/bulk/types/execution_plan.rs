#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdmittedBridgeExecutionPlan {
    workload_identity: BridgeWorkloadIdentity,
    canonical_planning_identity: BridgeCanonicalPlanningIdentity,
    admission_profile_identity: BridgeAdmissionProfileIdentity,
    reduced_artifact: ReducedBridgeWorkloadArtifact,
    counters: BridgeBulkPlanningCounters,
    locality_footprint: BridgeLocalityFootprint,
    selected_mode: BridgePreparationMode,
    legality_decision: BridgeParallelLegalityDecision,
    profitability_decision: BridgeParallelProfitabilityDecision,
    parallel_admission: BridgeParallelAdmission,
    legality_proof: ParallelPreparationLegalityProof,
    decision_log: BridgeBulkDecisionLog,
    planning_failures: Arc<[BridgeBulkPlanningFailure]>,
    digest: Arc<str>,
}

impl AdmittedBridgeExecutionPlan {
    pub(crate) fn new(
        workload_identity: BridgeWorkloadIdentity,
        canonical_planning_identity: BridgeCanonicalPlanningIdentity,
        admission_profile_identity: BridgeAdmissionProfileIdentity,
        reduced_artifact: ReducedBridgeWorkloadArtifact,
        counters: BridgeBulkPlanningCounters,
        locality_footprint: BridgeLocalityFootprint,
        selected_mode: BridgePreparationMode,
        legality_decision: BridgeParallelLegalityDecision,
        profitability_decision: BridgeParallelProfitabilityDecision,
        parallel_admission: BridgeParallelAdmission,
        legality_proof: ParallelPreparationLegalityProof,
        decision_log: BridgeBulkDecisionLog,
        planning_failures: Vec<BridgeBulkPlanningFailure>,
    ) -> Self {
        let planning_failures: Arc<[BridgeBulkPlanningFailure]> = planning_failures.into();
        let failure_count = planning_failures.len();
        let basis = format!(
            "admitted-bridge-execution-plan|workload={}|planning={}|profile={}|reduced-artifact={}|packet-count={}|reduction-output-count={}|locality={}|mode={}|legality={}|profitability={}|parallel-admission={}|legality-proof={}|decision-log={}|failure-count={}",
            workload_identity.as_str(),
            canonical_planning_identity.as_str(),
            admission_profile_identity.as_str(),
            reduced_artifact.digest(),
            counters.bulk_packet_count(),
            counters.bulk_reduction_output_count(),
            locality_footprint.digest(),
            super::super::planner::preparation_mode_label(selected_mode),
            legality_decision.digest(),
            profitability_decision.digest(),
            parallel_admission.digest(),
            legality_proof.digest(),
            decision_log.digest(),
            failure_count,
        );
        Self {
            workload_identity,
            canonical_planning_identity,
            admission_profile_identity,
            reduced_artifact,
            counters,
            locality_footprint,
            selected_mode,
            legality_decision,
            profitability_decision,
            parallel_admission,
            legality_proof,
            decision_log,
            planning_failures,
            digest: digest_string("admitted-bridge-execution-plan", &basis),
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

    pub fn reduced_artifact(&self) -> &ReducedBridgeWorkloadArtifact {
        &self.reduced_artifact
    }

    pub fn counters(&self) -> &BridgeBulkPlanningCounters {
        &self.counters
    }

    pub fn locality_footprint(&self) -> &BridgeLocalityFootprint {
        &self.locality_footprint
    }

    pub fn selected_mode(&self) -> BridgePreparationMode {
        self.selected_mode
    }

    pub fn legality_decision(&self) -> &BridgeParallelLegalityDecision {
        &self.legality_decision
    }

    pub fn profitability_decision(&self) -> &BridgeParallelProfitabilityDecision {
        &self.profitability_decision
    }

    pub fn parallel_admission(&self) -> &BridgeParallelAdmission {
        &self.parallel_admission
    }

    pub fn legality_proof(&self) -> &ParallelPreparationLegalityProof {
        &self.legality_proof
    }

    pub fn decision_log(&self) -> &BridgeBulkDecisionLog {
        &self.decision_log
    }

    pub fn planning_failures(&self) -> &[BridgeBulkPlanningFailure] {
        &self.planning_failures
    }

    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

use super::*;
