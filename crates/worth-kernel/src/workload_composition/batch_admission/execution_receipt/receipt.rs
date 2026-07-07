use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::workload_vocabulary::WorkloadEvidenceStage;

use super::BatchAdmissionExecutionCounters;
#[cfg(test)]
use crate::workload_composition::BatchAdmissionPlanDenialKind;
use crate::workload_composition::{
    BatchAdmissionFamilyPosture, BatchAdmissionPlanAdvisory, BatchAdmissionPlanDenial,
    BatchAdmissionSelectedFamilyRow, BatchAdmissionSupportingConflictFamilyRow,
    SelectedBatchAdmissionPlan,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchAdmissionExecutionReceipt {
    execution_receipt_digest: String,
    selected_batch_plan_digest: String,
    posture: BatchAdmissionFamilyPosture,
    authority_digests: Vec<String>,
    selected_conflict_plan_digests: Vec<String>,
    overlap_identity_digests: Vec<String>,
    locality_footprint_digests: Vec<String>,
    participant_identities: Vec<String>,
    selected_conflict_plan_identities: Vec<String>,
    independence_proof_identities: Vec<String>,
    selected_family_rows: Vec<BatchAdmissionSelectedFamilyRow>,
    supporting_conflict_family_rows: Vec<BatchAdmissionSupportingConflictFamilyRow>,
    advisory: Option<BatchAdmissionPlanAdvisory>,
    denial: Option<BatchAdmissionPlanDenial>,
    counters: BatchAdmissionExecutionCounters,
}

impl BatchAdmissionExecutionReceipt {
    pub(crate) fn from_selected_plan(plan: &SelectedBatchAdmissionPlan) -> Self {
        let counters = BatchAdmissionExecutionCounters::from_selected_plan(plan);
        let authority_digests = plan.authority_digests().to_vec();
        let selected_conflict_plan_digests = plan.selected_conflict_plan_digests().to_vec();
        let overlap_identity_digests = plan.overlap_identity_digests().to_vec();
        let locality_footprint_digests = plan.locality_footprint_digests().to_vec();
        let participant_identities = plan.participant_identities().to_vec();
        let independence_proof_identities = plan
            .parallel_admission_edges()
            .iter()
            .chain(plan.serial_admission_edges().iter())
            .map(|edge| edge.proof_digest().to_string())
            .chain(plan.denied_proof_identities().iter().cloned())
            .collect::<Vec<_>>();
        let execution_receipt_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &participant_identities
                .iter()
                .map(|identity| format!("participant:{identity}"))
                .chain(
                    authority_digests
                        .iter()
                        .map(|digest| format!("authority:{digest}")),
                )
                .chain(
                    selected_conflict_plan_digests
                        .iter()
                        .map(|digest| format!("selected-conflict:{digest}")),
                )
                .chain(
                    overlap_identity_digests
                        .iter()
                        .map(|digest| format!("overlap:{digest}")),
                )
                .chain(
                    locality_footprint_digests
                        .iter()
                        .map(|digest| format!("locality:{digest}")),
                )
                .chain(
                    independence_proof_identities
                        .iter()
                        .map(|identity| format!("proof:{identity}")),
                )
                .chain(plan.selected_family_rows().iter().map(|row| {
                    format!(
                        "family:{}:{}",
                        row.identity().as_str(),
                        row.declaration_digest()
                    )
                }))
                .chain(plan.supporting_conflict_family_rows().iter().map(|row| {
                    format!(
                        "support:{}:{}:{}:{}",
                        row.participant_identity(),
                        row.conflict_lane().as_str(),
                        row.conflict_family_identity(),
                        row.declaration_digest()
                    )
                }))
                .chain(std::iter::once(format!(
                    "selected-batch-plan:{}",
                    plan.selected_plan_digest()
                )))
                .chain(std::iter::once(format!(
                    "posture:{}",
                    plan.posture().as_str()
                )))
                .chain(std::iter::once(format!(
                    "counter-digest:{}",
                    counters.counter_digest()
                )))
                .chain(std::iter::once(
                    "worth-kernel:batch-admission-execution-receipt:v1".to_string(),
                ))
                .collect::<Vec<_>>(),
        );
        Self {
            execution_receipt_digest,
            selected_batch_plan_digest: plan.selected_plan_digest().to_string(),
            posture: plan.posture(),
            authority_digests,
            selected_conflict_plan_digests: selected_conflict_plan_digests.clone(),
            overlap_identity_digests,
            locality_footprint_digests,
            participant_identities: participant_identities.clone(),
            selected_conflict_plan_identities: selected_conflict_plan_digests,
            independence_proof_identities,
            selected_family_rows: plan.selected_family_rows().to_vec(),
            supporting_conflict_family_rows: plan.supporting_conflict_family_rows().to_vec(),
            advisory: plan.advisory().cloned(),
            denial: plan.denial().cloned(),
            counters,
        }
    }

    pub fn execution_receipt_digest(&self) -> &str {
        &self.execution_receipt_digest
    }
    pub fn selected_batch_plan_digest(&self) -> &str {
        &self.selected_batch_plan_digest
    }
    pub fn posture(&self) -> BatchAdmissionFamilyPosture {
        self.posture
    }
    pub fn authority_digests(&self) -> &[String] {
        &self.authority_digests
    }
    pub fn selected_conflict_plan_digests(&self) -> &[String] {
        &self.selected_conflict_plan_digests
    }
    pub fn overlap_identity_digests(&self) -> &[String] {
        &self.overlap_identity_digests
    }
    pub fn locality_footprint_digests(&self) -> &[String] {
        &self.locality_footprint_digests
    }
    pub fn participant_identities(&self) -> &[String] {
        &self.participant_identities
    }
    pub fn selected_conflict_plan_identities(&self) -> &[String] {
        &self.selected_conflict_plan_identities
    }
    pub fn independence_proof_identities(&self) -> &[String] {
        &self.independence_proof_identities
    }
    pub fn selected_family_rows(&self) -> &[BatchAdmissionSelectedFamilyRow] {
        &self.selected_family_rows
    }
    pub fn supporting_conflict_family_rows(&self) -> &[BatchAdmissionSupportingConflictFamilyRow] {
        &self.supporting_conflict_family_rows
    }
    pub fn advisory(&self) -> Option<&BatchAdmissionPlanAdvisory> {
        self.advisory.as_ref()
    }
    pub fn denial(&self) -> Option<&BatchAdmissionPlanDenial> {
        self.denial.as_ref()
    }
    pub fn counters(&self) -> &BatchAdmissionExecutionCounters {
        &self.counters
    }
    pub fn evidence_stage(&self) -> WorkloadEvidenceStage {
        WorkloadEvidenceStage::BatchAdmissionExecution
    }

    #[cfg(test)]
    pub(crate) fn with_test_denial_kind(mut self, kind: BatchAdmissionPlanDenialKind) -> Self {
        self.denial = Some(BatchAdmissionPlanDenial::new(
            kind,
            "batch admission hostile denial override",
        ));
        self
    }

    #[cfg(test)]
    pub(crate) fn with_test_overlap_identity_digests(mut self, digests: Vec<String>) -> Self {
        self.overlap_identity_digests = digests;
        self
    }

    #[cfg(test)]
    pub(crate) fn with_test_locality_footprint_digests(mut self, digests: Vec<String>) -> Self {
        self.locality_footprint_digests = digests;
        self
    }
}
