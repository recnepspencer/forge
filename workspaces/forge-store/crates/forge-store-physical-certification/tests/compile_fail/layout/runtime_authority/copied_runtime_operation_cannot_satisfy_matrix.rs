use forge_store_physical_certification::layout_harness::runtime::LayoutRuntimeCoverageMatrix;
use forge_store_physical_format::PlatformPhysicalRuntimeOperation;

fn main() {
    let copied_operation = PlatformPhysicalRuntimeOperation::AppendPhysicalRecord;
    let _ = LayoutRuntimeCoverageMatrix::default().record(copied_operation);
}
