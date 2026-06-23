use schema::facade::platform::authority::{
    worth_geometry_only_evidence_digest, GeometryOnlyEvidenceAdmissionError,
    WorthGeometryEvidenceSupportPosture, WorthGeometryOnlyEvidenceAdmission,
    WorthGeometryOnlyEvidenceCounters, WorthGeometryOnlyEvidenceReceiptProjection,
    WorthTouchedGraphBooleanEvidenceStage,
};

fn main() {
    let _ = (
        WorthTouchedGraphBooleanEvidenceStage::Split,
        WorthGeometryEvidenceSupportPosture::Admitted,
        WorthGeometryOnlyEvidenceCounters::from_evidence_rows(1),
        GeometryOnlyEvidenceAdmissionError::UnsupportedReceiptAuthority,
    );
    let _ = worth_geometry_only_evidence_digest;
    let _ = WorthGeometryOnlyEvidenceAdmission::from_spatial_boolean_receipt_projection;
    let _ = std::any::type_name::<dyn WorthGeometryOnlyEvidenceReceiptProjection>();
}
