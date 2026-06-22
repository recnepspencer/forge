use worth_spatial::facade::workload_vocabulary::{
    WorkloadEvidenceBooleanReceiptLookupProduct, WorkloadEvidenceRow, WorkloadEvidenceStage,
};

fn require_production_lookup(_: WorkloadEvidenceBooleanReceiptLookupProduct) {}

fn main() {
    let rows = vec![WorkloadEvidenceRow::new(
        WorkloadEvidenceStage::BooleanDeclarationEntry,
        "boolean-declaration:raw-row",
    )];

    require_production_lookup(rows);
}
