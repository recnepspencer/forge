use worth_spatial::facade::workload_vocabulary::{
    geometry_only_evidence_admission_from_boolean_evidence_receipt,
    BooleanEvidenceReceipt, BooleanEvidenceStageKind, WorkloadEvidenceStageCounters,
    WorkloadEvidenceSupport,
};

struct ForgedBooleanReceipt;

impl BooleanEvidenceReceipt for ForgedBooleanReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::Split
    }

    fn evidence_identity(&self) -> &str {
        "copied-spatial-receipt"
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_split()
    }
}

fn main() {
    let receipt = ForgedBooleanReceipt;
    let _ = geometry_only_evidence_admission_from_boolean_evidence_receipt(&receipt);
}
