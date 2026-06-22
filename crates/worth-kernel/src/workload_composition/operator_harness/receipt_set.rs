use worth_spatial::facade::workload_operators::CoplanarOverlapOperatorReceipt;
use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceRow, WorkloadEvidenceStage, WorkloadEvidenceStageLinkSet,
};

use super::declaration::{OperatorDeclarationReceipt, WorkloadOperatorFamily};
use super::run::OperatorRun;
use super::support::OperatorSupportReceipt;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OperatorReceiptSet {
    family: WorkloadOperatorFamily,
    declaration: OperatorDeclarationReceipt,
    support: OperatorSupportReceipt,
    consumed_evidence_identities: Vec<String>,
    consumed_stage_links: WorkloadEvidenceStageLinkSet,
    operator_outcome_digest: String,
    operator_evidence_row: WorkloadEvidenceRow,
}

impl OperatorReceiptSet {
    pub(super) fn from_coplanar_overlap_receipt(
        run: &OperatorRun,
        receipt: &CoplanarOverlapOperatorReceipt,
    ) -> Self {
        Self {
            family: run.family(),
            declaration: run.declaration().clone(),
            support: run.support().clone(),
            consumed_evidence_identities: receipt.consumed_evidence_identities().to_vec(),
            consumed_stage_links: receipt.consumed_stage_links().clone(),
            operator_outcome_digest: receipt.operator_digest().to_string(),
            operator_evidence_row: WorkloadEvidenceRow::from_coplanar_overlap_operator_receipt(
                receipt,
            ),
        }
    }

    pub fn family(&self) -> WorkloadOperatorFamily {
        self.family
    }

    pub fn declaration(&self) -> &OperatorDeclarationReceipt {
        &self.declaration
    }

    pub fn support(&self) -> &OperatorSupportReceipt {
        &self.support
    }

    pub fn consumed_evidence_identities(&self) -> &[String] {
        &self.consumed_evidence_identities
    }

    pub fn consumed_stage_links(&self) -> &WorkloadEvidenceStageLinkSet {
        &self.consumed_stage_links
    }

    pub fn operator_outcome_digest(&self) -> &str {
        &self.operator_outcome_digest
    }

    pub fn operator_evidence_row(&self) -> &WorkloadEvidenceRow {
        &self.operator_evidence_row
    }

    pub fn links_to_stage(&self, stage: WorkloadEvidenceStage) -> bool {
        self.consumed_stage_links.links_to(stage)
    }
}
