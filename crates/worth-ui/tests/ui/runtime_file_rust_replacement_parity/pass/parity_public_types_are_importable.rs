use worth_ui::facade::{
    WorthUiFileRustReplacementParityBoundary, WorthUiFileRustReplacementParityCounters,
    WorthUiFileRustReplacementParityDenial, WorthUiFileRustReplacementParityDenialReason,
    WorthUiFileRustReplacementParityReceipt, WorthUiFileRustReplacementPipelineReport,
    WorthUiFileRustReplacementSemanticReceipt,
};

fn main() {
    let _ = std::any::TypeId::of::<WorthUiFileRustReplacementParityBoundary>();
    let _ = std::any::TypeId::of::<WorthUiFileRustReplacementParityCounters>();
    let _ = std::any::TypeId::of::<WorthUiFileRustReplacementParityDenial>();
    let _ = std::any::TypeId::of::<WorthUiFileRustReplacementParityDenialReason>();
    let _ = std::any::TypeId::of::<WorthUiFileRustReplacementParityReceipt>();
    let _ = std::any::TypeId::of::<WorthUiFileRustReplacementPipelineReport>();
    let _ = std::any::TypeId::of::<WorthUiFileRustReplacementSemanticReceipt>();
}
