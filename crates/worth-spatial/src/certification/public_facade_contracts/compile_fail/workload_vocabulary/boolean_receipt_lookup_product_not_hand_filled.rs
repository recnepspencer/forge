use worth_spatial::facade::workload_vocabulary::{
    BooleanEvidenceStageKind, WorkloadEvidenceBooleanReceiptLookupProduct,
    WorkloadEvidenceStage, WorkloadEvidenceStageCounters, WorkloadEvidenceSupport,
};

fn main() {
    let _ = WorkloadEvidenceBooleanReceiptLookupProduct {
        boolean_stage: BooleanEvidenceStageKind::DeclarationEntry,
        evidence_stage: WorkloadEvidenceStage::BooleanDeclarationEntry,
        evidence_identity: "boolean-declaration:forged".to_string(),
        support: WorkloadEvidenceSupport::Admitted,
        counters: WorkloadEvidenceStageCounters::boolean_declaration(),
        stage_index_identity: "stage-index:forged".to_string(),
    };
}
