use worth_spatial::facade::workload_vocabulary::{
    BooleanEvidenceReceipt, BooleanEvidenceRowAuthority, BooleanEvidenceStageKind,
    CompleteWorkloadEvidenceLedger, WorkloadEvidenceRow, WorkloadEvidenceStageCounters,
    WorkloadEvidenceSupport,
};

struct ForgedBooleanReceipt;

impl BooleanEvidenceReceipt for ForgedBooleanReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::DeclarationEntry
    }

    fn evidence_identity(&self) -> &str {
        "forged-boolean-receipt"
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_declaration()
    }
}

impl BooleanEvidenceRowAuthority for ForgedBooleanReceipt {}

fn mint_lookup_from_external_receipt(ledger: &CompleteWorkloadEvidenceLedger) {
    let receipt = ForgedBooleanReceipt;
    let _row = WorkloadEvidenceRow::from_boolean_evidence_receipt(&receipt);
    let _lookup = ledger.require_boolean_receipt_lookup(&receipt);
}

fn main() {}
