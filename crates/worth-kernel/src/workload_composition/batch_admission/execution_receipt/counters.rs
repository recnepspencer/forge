use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_composition::{
    BatchAdmissionAdvisoryWitnessShape, BatchAdmissionPlanDenialKind,
    BatchAdmissionSupportingConflictLane, SelectedBatchAdmissionPlan,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchAdmissionExecutionCounters {
    participant_identity_count: usize,
    selected_conflict_plan_count: usize,
    supporting_conflict_family_row_count: usize,
    topology_supporting_conflict_family_row_count: usize,
    spatial_supporting_conflict_family_row_count: usize,
    parallel_independence_proof_count: usize,
    serial_independence_proof_count: usize,
    parallel_edge_breadth: usize,
    serial_edge_breadth: usize,
    selected_plan_denial_count: usize,
    declared_denied_proof_count: usize,
    advisory_query_boundary_count: usize,
    counter_digest: String,
}

impl BatchAdmissionExecutionCounters {
    pub(crate) fn from_selected_plan(plan: &SelectedBatchAdmissionPlan) -> Self {
        let topology_supporting_conflict_family_row_count = plan
            .supporting_conflict_family_rows()
            .iter()
            .filter(|row| row.conflict_lane() == BatchAdmissionSupportingConflictLane::Topology)
            .count();
        let spatial_supporting_conflict_family_row_count =
            plan.supporting_conflict_family_rows().len()
                - topology_supporting_conflict_family_row_count;
        let selected_plan_denial_count = usize::from(plan.denial().is_some_and(|denial| {
            denial.kind() == BatchAdmissionPlanDenialKind::SelectedPlanDenied
        }));
        let declared_denied_proof_count =
            usize::from(plan.denial().is_some_and(|denial| {
                denial.kind() == BatchAdmissionPlanDenialKind::DeclaredDenied
            }));
        let advisory_query_boundary_count = usize::from(plan.advisory().is_some_and(|advisory| {
            advisory.witness_shape()
                == BatchAdmissionAdvisoryWitnessShape::QueryBoundarySerialCoordination
        }));
        let counter_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                format!("participants:{}", plan.participant_identities().len()),
                format!(
                    "selected-conflict-plans:{}",
                    plan.participant_identities().len()
                ),
                format!(
                    "supporting-conflict-family-rows:{}",
                    plan.supporting_conflict_family_rows().len()
                ),
                format!("topology-supporting-rows:{topology_supporting_conflict_family_row_count}"),
                format!("spatial-supporting-rows:{spatial_supporting_conflict_family_row_count}"),
                format!(
                    "parallel-proof-count:{}",
                    plan.parallel_admission_edges().len()
                ),
                format!("serial-proof-count:{}", plan.serial_admission_edges().len()),
                format!(
                    "parallel-edge-breadth:{}",
                    plan.parallel_admission_edges().len()
                ),
                format!(
                    "serial-edge-breadth:{}",
                    plan.serial_admission_edges().len()
                ),
                format!("selected-plan-denial-count:{selected_plan_denial_count}"),
                format!("declared-denied-proof-count:{declared_denied_proof_count}"),
                format!("advisory-query-boundary-count:{advisory_query_boundary_count}"),
                "worth-kernel:batch-admission-execution-counters:v1".to_string(),
            ],
        );
        Self {
            participant_identity_count: plan.participant_identities().len(),
            selected_conflict_plan_count: plan.participant_identities().len(),
            supporting_conflict_family_row_count: plan.supporting_conflict_family_rows().len(),
            topology_supporting_conflict_family_row_count,
            spatial_supporting_conflict_family_row_count,
            parallel_independence_proof_count: plan.parallel_admission_edges().len(),
            serial_independence_proof_count: plan.serial_admission_edges().len(),
            parallel_edge_breadth: plan.parallel_admission_edges().len(),
            serial_edge_breadth: plan.serial_admission_edges().len(),
            selected_plan_denial_count,
            declared_denied_proof_count,
            advisory_query_boundary_count,
            counter_digest,
        }
    }

    pub fn participant_identity_count(&self) -> usize {
        self.participant_identity_count
    }
    pub fn selected_conflict_plan_count(&self) -> usize {
        self.selected_conflict_plan_count
    }
    pub fn supporting_conflict_family_row_count(&self) -> usize {
        self.supporting_conflict_family_row_count
    }
    pub fn topology_supporting_conflict_family_row_count(&self) -> usize {
        self.topology_supporting_conflict_family_row_count
    }
    pub fn spatial_supporting_conflict_family_row_count(&self) -> usize {
        self.spatial_supporting_conflict_family_row_count
    }
    pub fn parallel_independence_proof_count(&self) -> usize {
        self.parallel_independence_proof_count
    }
    pub fn serial_independence_proof_count(&self) -> usize {
        self.serial_independence_proof_count
    }
    pub fn parallel_edge_breadth(&self) -> usize {
        self.parallel_edge_breadth
    }
    pub fn serial_edge_breadth(&self) -> usize {
        self.serial_edge_breadth
    }
    pub fn selected_plan_denial_count(&self) -> usize {
        self.selected_plan_denial_count
    }
    pub fn declared_denied_proof_count(&self) -> usize {
        self.declared_denied_proof_count
    }
    pub fn advisory_query_boundary_count(&self) -> usize {
        self.advisory_query_boundary_count
    }
    pub fn counter_digest(&self) -> &str {
        &self.counter_digest
    }
}
