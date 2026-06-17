use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::workload_vocabulary::{
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceLedgerError, WorkloadEvidenceStage,
    WorkloadEvidenceStageLinkSet,
};

use super::support::OperatorWorkloadError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperatorEvidenceBinding {
    stage_index_identity: String,
    required_stage_links: WorkloadEvidenceStageLinkSet,
    evidence_row_count: usize,
    binding_identity: String,
}

impl OperatorEvidenceBinding {
    pub(super) fn from_ledger(
        ledger: &CompleteWorkloadEvidenceLedger,
        required_stages: &[WorkloadEvidenceStage],
    ) -> Result<Self, OperatorWorkloadError> {
        let stage_index = ledger.stage_index();
        let required_stage_links = stage_index
            .link_required_stages(required_stages)
            .map_err(map_stage_link_error)?;
        let stage_index_identity = stage_index.index_identity().to_string();
        let evidence_row_count = stage_index.counters().row_count();
        let binding_identity = binding_identity(
            &stage_index_identity,
            required_stage_links.link_set_identity(),
            evidence_row_count,
        );
        Ok(Self {
            stage_index_identity,
            required_stage_links,
            evidence_row_count,
            binding_identity,
        })
    }

    pub fn stage_index_identity(&self) -> &str {
        &self.stage_index_identity
    }

    pub fn required_stage_links(&self) -> &WorkloadEvidenceStageLinkSet {
        &self.required_stage_links
    }

    pub fn evidence_row_count(&self) -> usize {
        self.evidence_row_count
    }

    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }
}

fn binding_identity(
    stage_index_identity: &str,
    link_set_identity: &str,
    evidence_row_count: usize,
) -> String {
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "operator-evidence-binding".to_string(),
            format!("stage-index:{stage_index_identity}"),
            format!("stage-links:{link_set_identity}"),
            format!("evidence-rows:{evidence_row_count}"),
        ],
    )
}

fn map_stage_link_error(error: WorkloadEvidenceLedgerError) -> OperatorWorkloadError {
    match error {
        WorkloadEvidenceLedgerError::MissingAuthorityStage(stage) => {
            OperatorWorkloadError::MissingRequiredStage(stage)
        }
        other => OperatorWorkloadError::EvidenceStageBindingFailed(other),
    }
}
