use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;

use super::error::{EvidenceLookupFamilyCatalogError, EvidenceLookupFamilyCatalogErrorKind};
use super::stage_receipt_identity::EvidenceLookupStageReceiptFamilyIdentity;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupStageApplicability {
    stages: Vec<WorkloadEvidenceStage>,
    stage_receipt_family_identity: EvidenceLookupStageReceiptFamilyIdentity,
}

impl EvidenceLookupStageApplicability {
    pub(crate) fn matching_stages(
        stages: Vec<WorkloadEvidenceStage>,
        stage_receipt_family_identity: EvidenceLookupStageReceiptFamilyIdentity,
    ) -> Result<Self, EvidenceLookupFamilyCatalogError> {
        if stages.is_empty() {
            return Err(EvidenceLookupFamilyCatalogError::new(
                EvidenceLookupFamilyCatalogErrorKind::EmptyStageApplicability,
            ));
        }
        if has_duplicate_stage(&stages) {
            return Err(EvidenceLookupFamilyCatalogError::new(
                EvidenceLookupFamilyCatalogErrorKind::DuplicateStageApplicability,
            ));
        }
        Ok(Self {
            stages,
            stage_receipt_family_identity,
        })
    }

    pub fn stages(&self) -> &[WorkloadEvidenceStage] {
        &self.stages
    }

    pub fn stage_count(&self) -> usize {
        self.stages.len()
    }

    pub const fn stage_receipt_family_identity(&self) -> &EvidenceLookupStageReceiptFamilyIdentity {
        &self.stage_receipt_family_identity
    }

    pub fn applies_to(&self, stage: WorkloadEvidenceStage) -> bool {
        self.stages.contains(&stage)
    }

    pub fn declares_multiple_matching_stages(&self) -> bool {
        self.stages.len() > 1
    }
}

fn has_duplicate_stage(stages: &[WorkloadEvidenceStage]) -> bool {
    for (index, stage) in stages.iter().enumerate() {
        if stages[index + 1..].contains(stage) {
            return true;
        }
    }
    false
}
