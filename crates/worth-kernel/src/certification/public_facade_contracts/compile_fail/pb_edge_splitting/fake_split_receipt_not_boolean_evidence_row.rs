use worth_spatial::facade::workload_vocabulary::{
    BooleanEvidenceReceipt, BooleanEvidenceStageKind, WorkloadEvidenceRow,
    WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};

struct FakeSplitReceipt;

impl BooleanEvidenceReceipt for FakeSplitReceipt {
    fn boolean_stage(&self) -> BooleanEvidenceStageKind {
        BooleanEvidenceStageKind::Split
    }

    fn evidence_identity(&self) -> &str {
        "fake split"
    }

    fn evidence_support(&self) -> WorkloadEvidenceSupport {
        WorkloadEvidenceSupport::Admitted
    }

    fn evidence_counters(&self) -> WorkloadEvidenceStageCounters {
        WorkloadEvidenceStageCounters::boolean_split()
    }
}

fn main() {
    let _ = WorkloadEvidenceRow::from_boolean_evidence_receipt(&FakeSplitReceipt);
}
