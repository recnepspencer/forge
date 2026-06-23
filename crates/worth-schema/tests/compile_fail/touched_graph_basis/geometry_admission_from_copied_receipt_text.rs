use schema::facade::platform::authority::{
    WorthGeometryOnlyEvidenceAdmission, WorthGeometryOnlyEvidenceReceiptProjection,
};

struct CopiedReceiptText;

impl WorthGeometryOnlyEvidenceReceiptProjection for CopiedReceiptText {}

fn main() {
    let copied = CopiedReceiptText;
    let _ = WorthGeometryOnlyEvidenceAdmission::from_spatial_boolean_receipt_projection(&copied);
}
