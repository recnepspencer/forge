use worth_spatial::facade::workload_vocabulary::{
    SpatialGeometryEvidenceTouchRequest, WorkloadEvidenceRow, WorkloadEvidenceStage,
};

fn main() {
    let row = WorkloadEvidenceRow::new(
        WorkloadEvidenceStage::BooleanDeclarationEntry,
        "boolean-declaration:raw-row",
    );
    let _ = SpatialGeometryEvidenceTouchRequest::from_boolean_receipt(&row);
}
